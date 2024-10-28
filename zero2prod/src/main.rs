use zero2prod::startup::Application;
use zero2prod::{
    configrations::get_config,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".to_string(), "info".to_string(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_config().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await.expect("Failed to build application");
    application.run_until_stopped().await?;
    Ok(())
}
