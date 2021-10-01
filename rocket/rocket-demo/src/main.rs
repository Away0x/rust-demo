#[macro_use]
extern crate diesel;

use rocket::serde::json::{serde_json::json, Value};
use rocket::{catch, catchers, get, routes};

mod auth;
mod product;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket_sync_db_pools::database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![index])
        .attach(product::handlers::state())
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await?;

    Ok(())
}
