use anyhow::Result;
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
    #[serde(default)]
    pub compress: bool,
    pub ldap: LDAP,
    pub isbn_api_key: String,
}

pub fn parse_conf(p: &str) -> Result<Config> {
    let contents = fs::read_to_string(&p)?;
    let conf: Config = toml::from_str(contents.as_str())?;
    Ok(conf)
}
