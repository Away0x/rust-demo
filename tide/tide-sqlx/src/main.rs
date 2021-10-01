use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxMiddleware;

mod handler;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    // db
    let mut connect_opts = PgConnectOptions::new(); // 会自动读取 PGHOST 等环境变量
    connect_opts.log_statements(log::LevelFilter::Info);

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_opts)
        .await?;

    // server
    let mut app = tide::new();
    app.with(SQLxMiddleware::from(pg_pool));

    // routes
    app.register(
        root()
            .at("users", |route| route.get(handler::user::users))
            .at("api/hello", |route| route.get(handler::api::hello))
            .at("api/world", |route| route.get(handler::api::world)),
    );

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
