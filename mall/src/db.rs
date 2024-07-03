use actix::Addr;
use actix_redis::RedisActor;
use sqlx::mysql::MySqlPool;

pub async fn init_mysql_pool(database_url: &str) -> MySqlPool {
    MySqlPool::connect(database_url)
        .await
        .expect("Failed to create MySQL pool.")
}

pub async fn init_redis(redis_url: &str) -> Addr<RedisActor> {
    RedisActor::start(redis_url)
}
