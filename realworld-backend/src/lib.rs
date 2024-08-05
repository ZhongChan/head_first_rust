#[macro_use]
extern crate rocket;

mod database;
mod models;
mod routes;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/api", routes![])
}
