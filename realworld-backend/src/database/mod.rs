use rocket_sync_db_pools::database;

#[database("diesel_postgres_pool")]
pub struct Db(diesel::PgConnection);
