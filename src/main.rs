mod app;
mod conf;
mod data;
use crate::conf::get_conf;
use crate::data::books::BookMS;
use crate::data::ldap::LdapIdent;
use clap::Parser;
use tracing::{info, Level};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value = "./config.toml")]
    config: String,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("Starting up {}", &args.config);

    let conf = get_conf(&args.config);
    let p = data::get_pool(&conf.pg_dsn).await.unwrap();
    let b = if let (Some(bind_dn), Some(bind_pw)) = (conf.ldap.bind_dn, conf.ldap.bind_pw) {
        Some((bind_dn, bind_pw))
    } else {
        None
    };

    let mut li = LdapIdent::new(&conf.ldap.url, &conf.ldap.base, &conf.ldap.attr, b)
        .await
        .unwrap();

    let bms = BookMS::new(&p, &conf.isbn_api_key);

    app::start(&p, &conf.session_secret, &mut li, &bms).await;
}
