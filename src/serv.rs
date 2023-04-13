use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{
    body::Body as AxumBody,
    extract::Extension,
    http::{header::HeaderMap, Request},
    routing::get,
    Router,
};
use clap::Parser;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use libraryms::backend::books::BookMS;
use libraryms::backend::conf::parse_conf;
use libraryms::backend::ldap::LdapIdent;
use libraryms::components::home::*;
use libraryms::fallback::file_and_error_handler;
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, Level};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
    #[arg(short, long, default_value = "debug")]
    log: String,
}

pub async fn serv() {
    let args = Args::parse();
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(args.log.parse::<Level>().unwrap_or(Level::INFO))
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // get pwd
    let pwd = std::env::current_dir().unwrap();
    info!("Starting up {}, {:?}", &args.config, pwd);
    let server_conf = parse_conf(&args.config).expect("解析配置文件失败");

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options.clone();
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <BlogApp/> }).await;

    let pg_pool = libraryms::backend::db::init(&server_conf.pg_dsn)
        .await
        .expect("连接数据库失败");

    let ldap_ident = libraryms::backend::ldap::init(
        &server_conf.ldap.url,
        &server_conf.ldap.base,
        &server_conf.ldap.attr,
        if let (Some(bind_dn), Some(bind_pw)) =
            (&server_conf.ldap.bind_dn, &server_conf.ldap.bind_pw)
        {
            Some((bind_dn.clone(), bind_pw.clone()))
        } else {
            None
        },
    )
    .await
    .unwrap();
    let bms = libraryms::backend::books::init(&pg_pool, &server_conf.isbn_api_key)
        .await
        .expect("图书管理模块初始化失败");
    let a_ldap_ident = Arc::new(ldap_ident);
    let a_bms = Arc::new(bms);
    let a_pg_pool = Arc::new(pg_pool);
    let l_ldap_ident = a_ldap_ident.clone();
    let l_bms = a_bms.clone();
    let l_pg_pool = a_pg_pool.clone();
    let l_server_conf = Arc::new(server_conf.clone());

    libraryms::api::register_server_functions();

    // build our application with a route
    let mut app = Router::new()
        .route("/liveness", get(|| async { "I'm alive!" }))
        .route("/readiness", get(|| async { "I'm ready!" }))
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_context(
            leptos_options.clone(),
            routes,
            move |cx| {
                provide_context(cx, l_bms.clone());
                provide_context(cx, l_ldap_ident.clone());
                provide_context(cx, l_pg_pool.clone());
                provide_context(cx, l_server_conf.clone());
            },
            |cx| {
                view! { cx, <BlogApp/> }
            },
        )
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)))
        .layer(Extension(Arc::new(server_conf.clone())))
        .layer(Extension(a_pg_pool))
        .layer(Extension(a_ldap_ident))
        .layer(Extension(a_bms));
    if server_conf.compress {
        app = app.layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new()),
        );
    }

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn server_fn_handler(
    Extension(pool): Extension<Arc<PgPool>>,
    Extension(bms): Extension<Arc<BookMS>>,
    Extension(ldap_ident): Extension<Arc<LdapIdent>>,
    Extension(server_conf): Extension<Arc<libraryms::backend::conf::Config>>,
    path: Path<String>,
    headers: HeaderMap,
    // raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    debug!("{:?}", path);

    handle_server_fns_with_context(
        path,
        headers,
        move |cx| {
            provide_context(cx, bms.clone());
            provide_context(cx, pool.clone());
            provide_context(cx, ldap_ident.clone());
            provide_context(cx, server_conf.clone());
        },
        request,
    )
    .await
}
