use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    configrations::{get_config, DatabaseSettings}
    ,
    telemetry::{get_subscriber, init_subscriber},
};

use once_cell::sync::Lazy;
use zero2prod::startup::{get_connection_pool, Application};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber_name = "test".to_string();
    let default_filter_level = "debug".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // The first time `init` is invoked the code in `TRACING` is executed.
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = get_config().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone()).await.expect("Failed to build application.");
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp { address, db_pool: get_connection_pool(&configuration.database) }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create Database
    let mut connection =
        PgConnection::connect_with(&config.without_db())
            .await
            .expect("Failed to connect to Postgres");

    sqlx::query(&format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");

    // Migrate database
    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");
    db_pool
}


