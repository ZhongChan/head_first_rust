pub mod configrations;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use sqlx::PgConnection;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

// subscribe
// (2) Retriving a connection from the application state!
async fn subscribe(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>, //(2)
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener, connncetion: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connncetion);

    // Capture `connnection` from the surrounding enviroment
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
