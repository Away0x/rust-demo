use serde::Serialize;
use serde_json::json;
use sqlx::postgres::Postgres;
use sqlx::Acquire;
use tide_sqlx::SQLxRequestExt;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct User {
    name: String,
    dept: String,
}

pub async fn users(req: tide::Request<()>) -> tide::Result {
    let mut pg_conn = req.sqlx_conn::<Postgres>().await;

    let user = sqlx::query_as::<Postgres, User>("select name, dept from users")
        .fetch_all(pg_conn.acquire().await?)
        .await?;

    Ok(json!(user).into())
}
