#[macro_use]
extern crate rocket;

mod database;
mod models;
mod routes;

use routes::{articles, profiles, tags, users};

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![
            users::register,
            users::login,
            users::current_user,
            users::update_user,
        ],
    )
}
