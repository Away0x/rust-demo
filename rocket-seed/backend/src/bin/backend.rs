use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{catch, catchers, get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use backend::db::{establish_connection, query_task};
use rocket_seed::JsonApiResponse;

#[rocket_sync_db_pools::database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

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
    let mut response = JsonApiResponse { data: vec![], };
    let conn = establish_connection();

    for task in query_task(&conn, None) {
        let api_task = rocket_seed::Task {
            id: task.id,
            title: task.title,
            status: task.status,
        };
        response.data.push(api_task);
    }
    
    Json(response)
}

// 使用 connection pool
#[get("/tasks2")]
async fn tasks_get2(conn: DbConn) -> Json<JsonApiResponse> {
    conn.run(|c| {
        let mut response = JsonApiResponse { data: vec![], };

        for task in query_task(&c, None) {
            let api_task = rocket_seed::Task {
                id: task.id,
                title: task.title,
                status: task.status,
            };
            response.data.push(api_task);
        }

        Json(response)
    })
    .await
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorized", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::build()
        .mount("/", routes![tasks_get1, tasks_get2])
        .register("/", catchers![not_found])
        .attach(cors)
        .attach(DbConn::fairing())
        .launch()
        .await?;
    Ok(())
}
