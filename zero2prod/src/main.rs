use std::net::TcpListener;

use zero2prod::{configrations::get_config, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_config().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
