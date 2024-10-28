use crate::configrations::{DatabaseSettings, Settings};
use crate::{
    email_client::EmailClient,
    routes::{health_check, subscribe},
};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;


pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, Error> {
        let db_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

        // Build an `EmailClient` using `configuration`
        let sender = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");

        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!("{}:{}", configuration.application.host, configuration.application.port);
        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr()?.port();
        let server = run(listener, db_pool, email_client)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), Error> {
        self.server.await
    }
}


pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection_pool = web::Data::new(db_pool);

    // Capture `connection` from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            // Instead of `Logger::default`
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
            .app_data(email_client.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}