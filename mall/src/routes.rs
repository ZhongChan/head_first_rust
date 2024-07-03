use crate::handlers::{get_data_from_mysql_1, get_data_from_mysql_2};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mysql1").route(web::get().to(get_data_from_mysql_1)));
    cfg.service(web::resource("/mysql2").route(web::get().to(get_data_from_mysql_2)));
}
