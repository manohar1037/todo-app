use std::env;

use dotenvy::dotenv;

use crate::constant::*;

#[derive(Debug,Clone)]
pub struct Config {
    pub basic_user: String,
    pub basic_pass: String,
    pub port: u16,
}

impl Config {
    pub fn new()-> Self {
        dotenv().ok();
        let basic_user = env::var(BASIC_AUTH_USER).unwrap_or_default();
        let basic_pass = env::var(BASIC_AUTH_PASS).unwrap_or_default();
        let port = env::var(PORT)
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);

        Config {
            basic_user,
            basic_pass,
            port,
        }
    }
}
