#[macro_use]
extern crate diesel;
extern crate dotenv;

mod routes;
mod schema;
mod models;

use actix_web::{App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("can not find database");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .service(routes::add_product)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}