use clap::Parser;
use std::env;
use tracing::Level;

/// Q&A web service API
#[derive(Parser, Debug, PartialEq)]
#[clap(author,version,about,long_about=None)]
pub struct Config {
    #[clap(short, long, default_value = "warn")]
    pub log_level: String,

    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    // cli args: --db-user ,not --db_user
    #[clap(long, default_value = "user")]
    pub db_user: String,

    #[clap(long)]
    pub db_password: Option<String>,

    #[clap(long, default_value = "localhost")]
    pub db_host: String,

    #[clap(long, default_value = "5432")]
    pub db_port: u16,

    #[clap(long, default_value = "rustwebdev")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Self, handle_errors::Error> {
        let config = Config::parse();

        if let Err(_) = env::var("APILAYER_KEY") {
            panic!("Apilayer API key not set");
        }

        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO key not set")
        }

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(8080))
            .map_err(|e| {
                println!("port parse error");
                tracing::event!(Level::ERROR, "port parse error");
                handle_errors::Error::ParseError(e)
            })?;

        let db_user = env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();
        let db_host = env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT").unwrap_or(config.db_port.to_string());
        let db_name = env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());
        println!("db_password:{}", db_password);

        Ok(Config {
            log_level: config.log_level,
            port: port,
            db_user: db_user,
            db_password: Some(db_password),
            db_host: db_host,
            db_port: db_port.parse::<u16>().map_err(|e| {
                println!("db_port parse error");
                tracing::event!(Level::ERROR, "db_port parse error");
                handle_errors::Error::ParseError(e)
            })?,
            db_name: db_name,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use serial_test::serial;

    fn set_env() {
        env::set_var("PASETO_KEY", "yes");
        env::set_var("POSTGRES_USER", "user");
        env::set_var("POSTGRES_PASSWORD", "pass");
        env::set_var("POSTGRES_DB", "rustwebdev");
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_PORT", "5432");
        env::set_var("APILAYER_KEY", "yes");
    }

    fn unset_env() {
        env::remove_var("PASETO_KEY");
        env::remove_var("POSTGRES_USER");
        env::remove_var("POSTGRES_PASSWORD");
        env::remove_var("POSTGRES_DB");
        env::remove_var("POSTGRES_HOST");
        env::remove_var("POSTGRES_PORT");
        env::remove_var("APILAYER_KEY");
    }

    #[test]
    #[serial]
    fn unset_api_key() {
        unset_env();
        let result = std::panic::catch_unwind(|| Config::new());
        println!("{:?}", result);
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn set_api_key() {
        set_env();
        let expected = Config {
            log_level: "warn".to_string(),
            port: 8080,
            db_user: "user".to_string(),
            db_password: Some("pass".to_string()),
            db_host: "localhost".to_string(),
            db_port: 5432,
            db_name: "rustwebdev".to_string(),
        };

        let config = Config::new().unwrap();

        assert_eq!(config, expected);
    }
}
