pub mod articles;
pub mod profiles;
pub mod tags;
pub mod users;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.append(&mut users::get_routes());
    routes.append(&mut articles::get_routes());
    routes.append(&mut profiles::get_routes());
    routes.append(&mut tags::get_routes());
    routes
}
