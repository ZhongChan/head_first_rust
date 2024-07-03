use actix_web::HttpResponse;

pub async fn get_data_from_mysql_1() -> HttpResponse {
    // 从 MySQL 数据库 1 中获取数据的代码
    HttpResponse::Ok().body("Data from MySQL Database 1")
}

pub async fn get_data_from_mysql_2() -> HttpResponse {
    // 从 MySQL 数据库 2 中获取数据的代码
    HttpResponse::Ok().body("Data from MySQL Database 2")
}
