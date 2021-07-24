use serde::Serialize;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, Postgres};
use sqlx::{Acquire, ConnectOptions};
use tide_sqlx::SQLxMiddleware;
use tide_sqlx::SQLxRequestExt;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Record {
    name: String,
    dept: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    // db
    let mut connect_opts = PgConnectOptions::new(); // 会读取 PGHOST 等环境变量
    connect_opts.log_statements(log::LevelFilter::Info);

    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_opts)
        .await?;

    // server
    let mut app = tide::new();
    app.with(SQLxMiddleware::from(pg_pool));

    app.at("/").get(|req: tide::Request<()>| async move {
        let mut pg_conn = req.sqlx_conn::<Postgres>().await;

        let companies = sqlx::query_as::<Postgres, Record>("select name, dept from company")
            .fetch_all(pg_conn.acquire().await?)
            .await?;

        Ok(json!(companies))
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
