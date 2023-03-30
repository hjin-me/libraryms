use cfg_if::cfg_if;
use leptos::*;
// boilerplate to run in different modes
cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        routing::{post, get, any},
        extract::{Extension},
        Router,
    };
    use clap::Parser;

    use libraryms::components::home::*;
    use libraryms::fallback::file_and_error_handler;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::sync::Arc;
    use std::fs;
    use tracing::{info, Level};
    use tower_http::{compression::CompressionLayer};
    use tower_http::{trace::TraceLayer};
    use tower::ServiceBuilder;
    use libraryms::backend::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
    #[arg(short, long, default_value = "info")]
    log: String,
}

    #[tokio::main]
    async fn main() {

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
        let _contents =
            fs::read_to_string(&args.config).expect("Should have been able to read the file");

        // Setting this to None means we'll be using cargo-leptos and its env vars
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options.clone();
        let addr = leptos_options.site_addr;
        let routes = generate_route_list(|cx| view! { cx, <BlogApp/> }).await;

        // build our application with a route
        let app = Router::new()
            .layer(CompressionLayer::new())
            .route("/liveness", get(|| async { "I'm alive!" }))
            .route("/readiness", get(|| async { "I'm ready!" }))
            .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
            .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <BlogApp/> } )
            .fallback(file_and_error_handler)
            .layer(Extension(Arc::new(leptos_options)))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CompressionLayer::new())
            );

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        log!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

    // client-only stuff for Trunk
    else {
        pub fn main() {
            // This example cannot be built as a trunk standalone CSR-only app.
            // Only the server may directly connect to the database.
        }
    }
}
