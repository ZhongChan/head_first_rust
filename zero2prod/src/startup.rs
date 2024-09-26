use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection_pool = web::Data::new(db_pool);

    // Capture `connnection` from the surrounding enviroment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
