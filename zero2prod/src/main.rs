use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::{configrations::get_config, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // `init` dos call `set_logger`
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_config().expect("Failed to read configuration.");

    let pg_address = configuration.database.connection_string();
    let db_pool = PgPool::connect(&pg_address)
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener, db_pool)?.await
}
