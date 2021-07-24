use argon2::{self, Config};
use chrono::Utc;
use serde_json::json;
use sqlx::{query, query_as, PgPool, Pool};
use tide::{Body, Request, Response, Server, StatusCode};
use uuid::Uuid;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod entity;

#[derive(Debug, Clone)]
struct State {
    db_pool: PgPool,
}

async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    info!("{}", db_url);
    Pool::connect(&db_url).await.unwrap()
}

async fn make_server() -> tide::Result<Server<State>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    
    let db_pool = make_db_pool().await;
    let mut app = Server::with_state(State { db_pool });

    app.at("/users")
        .get(|req: Request<State>| async move {
            // 获取到数据库连接池
            let db_pool: &PgPool = &req.state().db_pool;
            // 获取数据
            let users = query_as!(entity::user::User, "select id, username from users")
                .fetch_all(db_pool)
                .await?;

            let mut res = Response::new(StatusCode::Ok);
            res.set_body(Body::from_json(&users)?);

            Ok(res)
        })
        .post(|mut req: Request<State>| async move {
            let db_pool: PgPool = req.state().db_pool.clone();
            // 获取参数
            let create_user = req.body_json::<entity::user::CreateUser>().await?;
            // 密码加密
            let salt = std::env::var("HASH_SALT").expect("env error");
            let config = Config::default();
            let hashed_password =
                argon2::hash_encoded(create_user.password.as_bytes(), salt.as_bytes(), &config)
                    .expect("hashed error");

            let row = query!(
                r#"
                    insert into users (id, username, hashed_password, created_at, updated_at)
                    values ($1, $2, $3, $4, $5)
                "#,
                Uuid::new_v4(),
                create_user.username,
                hashed_password,
                Utc::now(),
                Utc::now(),
            )
            .execute(&db_pool)
            .await
            .expect("db error");

            let mut res = Response::new(StatusCode::Created);
            res.set_body(json!({
                "affected row": row.rows_affected(),
            }));

            Ok(res)
        });

    app.at("/login").post(|mut req: Request<State>| async move {
        let db_pool: PgPool = req.state().db_pool.clone();
        let login_user = req.body_json::<entity::user::LoginUser>().await?;

        let password = query_as!(
            entity::user::Password,
            "select hashed_password from users where username=$1",
            login_user.username,
        )
        .fetch_one(&db_pool)
        .await
        .expect("db error");

        let matches =
            argon2::verify_encoded(&password.hashed_password, login_user.password.as_bytes())
                .expect("verify error");

        let token = if matches { "access token" } else { "" };

        let mut res = Response::new(StatusCode::Ok);
        res.set_body(json!({ "token": token, "status": matches }));

        Ok(res)
    });

    Ok(app)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = make_server().await?;
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
