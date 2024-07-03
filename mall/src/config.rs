use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub server_addr: String,
    pub mysql_url_1: String,
    pub mysql_url_2: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();
        Ok(Self {
            server_addr: env::var("SERVER_ADDR")?,
            mysql_url_1: env::var("MYSQL_URL_1")?,
            mysql_url_2: env::var("MYSQL_URL_2")?,
            redis_url: env::var("REDIS_URL")?,
        })
    }
}
