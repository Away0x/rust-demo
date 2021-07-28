use rocket::{catch, catchers, get, routes};
use rocket::serde::Serialize;
use rocket::serde::json::{serde_json::json, Json, Value};

use backend::db::models::Task;
use backend::db::{query_task, establish_connection};

#[rocket_sync_db_pools::database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct JsonApiResponse {
    data: Vec<Task>,
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

// 不使用 connection pool
#[get("/tasks1")]
fn tasks_get1() -> Json<JsonApiResponse> {
    let conn = establish_connection();
    let tasks = query_task(&conn, None);
    Json(JsonApiResponse { data: tasks })
}

// 使用 connection pool
#[get("/tasks2")]
async fn tasks_get2(conn: DbConn) -> Json<JsonApiResponse> {
    conn.run(|c| {
        let tasks = query_task(c, None);
        Json(JsonApiResponse { data: tasks })
    }).await
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/", routes![tasks_get1, tasks_get2])
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await?;
    Ok(())
}