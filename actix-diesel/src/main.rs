#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};

use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;

mod models;
mod routes;
mod schema;
mod services;
mod webscoket;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); // 加载环境变量

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            // restful api
            .service(routes::add_product)
            .service(routes::get_all_product)
            .service(routes::del_product)
            // webscoket
            .service(web::resource("/ws").to(webscoket::ws_handle))
            // 静态文件服务
            .service(
                actix_files::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            // 首页
            .service(web::resource("/").to(routes::home))
    })
    .bind("127.0.0.1:8886")?
    .run()
    .await
}
