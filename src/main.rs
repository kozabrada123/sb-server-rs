#[macro_use]
extern crate rocket;

pub mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/auth", routes![routes::auth::auth_login])
}
