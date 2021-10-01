use rocket::serde::json::{serde_json::json, Value};
use rocket::{catch, catchers, get, routes};

mod state;
mod user;

#[get("/")]
async fn hello() -> String {
    "hello world".to_string()
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![hello])
        .attach(user::stage())
        .attach(state::stage())
        .register("/", catchers![not_found])
        .launch()
        .await?;

    Ok(())
}
