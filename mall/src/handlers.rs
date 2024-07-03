use crate::db::MysqlPool;
use actix_web::{web, HttpResponse};

pub async fn get_data_from_mysql_1(mysql_pool_1: web::Data<MysqlPool>) -> HttpResponse {
    let conn = mysql_pool_1.get().unwrap();
    // 从 MySQL 数据库 1 中获取数据的代码
    HttpResponse::Ok().body("Data from MySQL Database 1")
}

pub async fn get_data_from_mysql_2(mysql_pool_2: web::Data<MysqlPool>) -> HttpResponse {
    let conn = mysql_pool_2.get().unwrap();
    // 从 MySQL 数据库 2 中获取数据的代码
    HttpResponse::Ok().body("Data from MySQL Database 2")
}
