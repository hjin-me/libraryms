mod app;
mod data;

use crate::data::ldap::LdapIdent;
use clap::Parser;
use serde_derive::Deserialize;
use std::fs;
use toml;
use tracing::{info, Level};

#[derive(Debug, Deserialize)]
struct Config {
    pg_dsn: String,
    session_secret: String,
    ldap_url: String,
    ldap_base: String,
    ldap_attr: String,
    ldap_bind_dn: Option<String>,
    ldap_bind_pw: Option<String>,
}
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

    let contents =
        fs::read_to_string(&args.config).expect("Should have been able to read the file");
    let conf: Config = toml::from_str(contents.as_str()).unwrap();
    let p = data::get_pool(&conf.pg_dsn).await.unwrap();
    let b = if let (Some(bind_dn), Some(bind_pw)) = (conf.ldap_bind_dn, conf.ldap_bind_pw) {
        Some((bind_dn, bind_pw))
    } else {
        None
    };

    let mut li = LdapIdent::new(&conf.ldap_url, &conf.ldap_base, &conf.ldap_attr, b)
        .await
        .unwrap();

    app::start(&p, &conf.session_secret, &mut li).await;
}
