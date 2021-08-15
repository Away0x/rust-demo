#[macro_use]
extern crate rocket;

use std::io::Cursor;

use rocket::{fairing::AdHoc, fs::FileServer, Build, Rocket};

mod config;
mod fairings;
mod handlers;
#[cfg(test)]
mod tests;

pub fn build_rocket_app() -> Rocket<Build> {
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
                handlers::get_file,
                handlers::upload_with_tempfile,
            ],
        )
        .attach(AdHoc::config::<config::Config>())
        // attach 中间件 struct
        .attach(fairings::Counter::default())
        // attach 中间件函数
        .attach(AdHoc::on_request("req ok", |req, _| {
            Box::pin(async move {
                println!(" => Incomming");
                if req.uri() == "/ok" {
                    println!("ok");
                }
            })
        }))
        .attach(AdHoc::on_response("res ok", |req, res| {
            Box::pin(async move {
                // 请求 "/ok" 时响应 "Hello ok"
                if req.uri().path() == "/ok" {
                    res.set_sized_body(None, Cursor::new("Hello ok"))
                }
            })
        }))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取自定义配置
    config::read_config()?;

    build_rocket_app().launch().await?;

    Ok(())
}
