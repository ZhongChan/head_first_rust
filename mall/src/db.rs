use actix::Addr;
use actix_redis::RedisActor;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type MysqlPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_mysql_pool(database_url: &str) -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create MySQL pool.")
}

pub async fn init_redis(redis_url: &str) -> Addr<RedisActor> {
    RedisActor::start(redis_url)
}
