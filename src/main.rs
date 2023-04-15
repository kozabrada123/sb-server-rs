#[macro_use]
extern crate rocket;

pub mod routes;
pub mod schema;

// Until we have config files this is how we'll set out database
// protocol can be mysql, postgres or sqlite
//"protocol://username:password@host/database";
pub const DB_PATH: &str = "protocol://username:password@host/database";

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    rocket::build().mount("/api/auth", routes![routes::auth::auth_login])
}
