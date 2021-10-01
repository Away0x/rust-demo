use crate::models::ProductJson;
use crate::services;
use crate::Pool;

use actix_web::{delete, get, post, web, Error, HttpResponse};
use tera::{Context, Tera};

pub async fn home() -> Result<HttpResponse, Error> {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let ctx = Context::new();

    let back = tera.render("index.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(back))
}

#[post("/add_product")]
pub async fn add_product(
    pool: web::Data<Pool>,
    item: web::Json<ProductJson>,
) -> anyhow::Result<HttpResponse, Error> {
    Ok(services::add_single_product(pool, item)
        .await
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/get_all_product")]
pub async fn get_all_product(pool: web::Data<Pool>) -> anyhow::Result<HttpResponse, Error> {
    Ok(services::get_all_product(pool)
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[delete("/del_product/{id}")]
pub async fn del_product(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> anyhow::Result<HttpResponse, Error> {
    Ok(services::delete_product(pool, path)
        .await
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
