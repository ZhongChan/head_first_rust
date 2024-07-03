use actix_redis::RedisSession;
use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = config::Config::from_env().unwrap();
    let mysql_pool_1 = db::init_mysql_pool(&config.mysql_url_1);
    let mysql_pool_2 = db::init_mysql_pool(&config.mysql_url_2);
    let redis_addr = db::init_redis(&config.redis_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mysql_pool_1.clone()))
            .app_data(web::Data::new(mysql_pool_2.clone()))
            .app_data(web::Data::new(redis_addr.clone()))
            .wrap(
                CookieSession::signed(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            )
            .wrap(RedisSession::new(config.redis_url.clone(), &[0; 32]))
            .configure(routes::init)
    })
    .bind(config.server_addr)?
    .run()
    .await
}
