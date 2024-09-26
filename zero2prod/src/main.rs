use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::{configrations::get_config, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_config().expect("Failed to read configuration.");

    let pg_address = configuration.database.connection_string();
    let connection = PgConnection::connect(&pg_address)
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}
