use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct LDAP {
    pub url: String,
    pub base: String,
    pub attr: String,
    pub bind_dn: Option<String>,
    pub bind_pw: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg_dsn: String,
    pub session_secret: String,
    pub ldap: LDAP,
    pub isbn_api_key: String,
}

pub fn get_conf(p: &str) -> Config {
    let contents = fs::read_to_string(&p).expect("Should have been able to read the file");
    let conf: Config = toml::from_str(contents.as_str()).unwrap();
    conf
}
