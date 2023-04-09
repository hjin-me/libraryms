use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use clap::Parser;
use leptos::*;

use leptos_axum::{generate_route_list, LeptosRoutes};
use libraryms::backend::conf;
use libraryms::backend::conf::{get_conf, parse_conf};
use libraryms::components::home::*;
use libraryms::fallback::file_and_error_handler;
use std::fs;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
    #[arg(short, long, default_value = "info")]
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
    parse_conf(&args.config);

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options.clone();
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <BlogApp/> }).await;

    let conf = get_conf();
    libraryms::backend::db::init(&conf.pg_dsn).await.unwrap();

    libraryms::backend::ldap::init(
        &conf.ldap.url,
        &conf.ldap.base,
        &conf.ldap.attr,
        if let (Some(bind_dn), Some(bind_pw)) = (&conf.ldap.bind_dn, &conf.ldap.bind_pw) {
            Some((bind_dn.clone(), bind_pw.clone()))
        } else {
            None
        },
    )
    .await
    .unwrap();
    libraryms::backend::books::init(&conf.pg_dsn, &conf.isbn_api_key)
        .await
        .unwrap();
    libraryms::api::register_server_functions();

    // build our application with a route
    let app = Router::new()
        .route("/liveness", get(|| async { "I'm alive!" }))
        .route("/readiness", get(|| async { "I'm ready!" }))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(
            leptos_options.clone(),
            routes,
            |cx| view! { cx, <BlogApp/> },
        )
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));
    // .layer(
    //     ServiceBuilder::new()
    //         .layer(TraceLayer::new_for_http())
    //         .layer(CompressionLayer::new()),
    // );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
