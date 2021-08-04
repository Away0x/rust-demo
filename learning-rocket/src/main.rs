#[macro_use]
extern crate rocket;

use rocket::{fairing::AdHoc, fs::FileServer};

mod config;
mod handlers;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取自定义配置
    config::read_config()?;

    rocket::build()
        .mount("/public", FileServer::from("public/")) // 文件服务器
        .mount(
            "/",
            routes![
                handlers::index,
                handlers::dynamic_paths,
                handlers::path_buf,
                handlers::get_config,
                handlers::get_request_guard,
                handlers::test_cookies,
                handlers::body_json,
                handlers::body_form,
            ],
        )
        .attach(AdHoc::config::<config::Config>())
        .launch()
        .await?;

    Ok(())
}
