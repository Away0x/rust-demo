use diesel::result::Error;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{delete, get, http::Status, post, put, response::status, routes};

use super::models::*;
use super::repositories::*;
use crate::auth::BasicAuth;
use crate::DbConn;

fn db_error(e: Error) -> status::Custom<Value> {
    status::Custom(Status::InternalServerError, json!(e.to_string()))
}

#[get("/")]
async fn product_list(conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        ProductRepository::all(c)
            .map(|p| json!(p))
            .map_err(|e| db_error(e))
    })
    .await
}

#[get("/<id>")]
async fn product_detail(id: i32, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(move |c| {
        ProductRepository::get(c, id)
            .map(|p| json!(p))
            .map_err(|e| db_error(e))
    })
    .await
}

// 参数有 BasicAuth 的需要认证后才可访问
#[post("/", format = "json", data = "<new_product>")]
async fn new_product(
    _auth: BasicAuth,
    new_product: Json<NewProduct>,
    conn: DbConn,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        ProductRepository::create(c, new_product.into_inner())
            .map(|p| json!(p))
            .map_err(|e| db_error(e))
    })
    .await
}

#[put("/", format = "json", data = "<product>")]
async fn update_product(
    _auth: BasicAuth,
    conn: DbConn,
    product: Json<Product>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(|c| {
        ProductRepository::update(c, product.into_inner())
            .map(|p| json!(p))
            .map_err(|e| db_error(e))
    })
    .await
}

#[delete("/<id>")]
async fn delete_product(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<status::NoContent, status::Custom<Value>> {
    conn.run(move |c| {
        ProductRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| db_error(e))
    })
    .await
}

pub fn state() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("product", |rocket| async {
        rocket.mount(
            "/products",
            routes![
                product_list,
                product_detail,
                new_product,
                update_product,
                delete_product
            ],
        )
    })
}
