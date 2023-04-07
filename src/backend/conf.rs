use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct LDAP {
    pub url: String,
    pub base: String,
    pub attr: String,
    pub bind_dn: Option<String>,
    pub bind_pw: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub pg_dsn: String,
    pub session_secret: String,
    pub ldap: LDAP,
    pub isbn_api_key: String,
}

use anyhow::Result;
#[cfg(feature = "ssr")]
use once_cell::sync::OnceCell;
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
static INSTANCE: OnceCell<Arc<Config>> = OnceCell::new();

pub fn parse_conf(p: &str) -> Config {
    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");
    let conf: Config = toml::from_str(contents.as_str()).unwrap();
    INSTANCE.set(Arc::new(conf.clone())).unwrap();
    conf
}

pub fn get_conf() -> Arc<Config> {
    INSTANCE.get().unwrap().clone()
}
