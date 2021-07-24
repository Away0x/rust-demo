use actix_web::{post, HttpResponse, Responder};

#[post("/add_product")]
pub async fn add_product() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
