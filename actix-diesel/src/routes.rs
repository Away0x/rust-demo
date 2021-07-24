use crate::Pool;
use crate::services;
use crate::models::ProductJson;

use actix_web::{post, get, delete, HttpResponse, Error, web};


#[post("/add_product")]
pub async fn add_product(
    pool: web::Data<Pool>,
    item: web::Json<ProductJson>
) -> anyhow::Result<HttpResponse, Error> {
    Ok(
        services::add_single_product(pool, item)
            .await
            .map(|product| HttpResponse::Created().json(product))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[get("/get_all_product")]
pub async fn get_all_product(
    pool: web::Data<Pool>
) -> anyhow::Result<HttpResponse, Error> {
    Ok(
        services::get_all_product(pool)
            .await
            .map(|product| HttpResponse::Ok().json(product))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

#[delete("/del_product/{id}")]
pub async fn del_product(
    pool: web::Data<Pool>,
    path: web::Path<String>
) -> anyhow::Result<HttpResponse, Error> {
    Ok(
        services::delete_product(pool, path)
            .await
            .map(|product| HttpResponse::Ok().json(product))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}