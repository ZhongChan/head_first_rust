use database::Db;
use routes::get_routes;

#[macro_use]
extern crate rocket;

mod database;
mod models;
mod routes;
mod schema;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Db::fairing())
        .mount("/api", get_routes())
}
