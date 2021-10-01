use crate::models::{PostProduct, Product, ProductJson};
use crate::Pool;

use actix_web::web;
use diesel::delete;
use diesel::dsl::insert_into;
use diesel::prelude::*;

/**
 * add product
 * 查询 name 如果有则直接获取并返回
 * 如果没有则添加
 */
pub async fn add_single_product(
    pool: web::Data<Pool>,
    item: web::Json<ProductJson>,
) -> anyhow::Result<Product, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();

    match product
        .filter(name.eq(&item.name))
        .first::<Product>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            // 添加
            let new_product = PostProduct {
                name: &item.name,
                title: &item.title,
                data_created: &format!("{}", chrono::Local::now().naive_local()),
            };

            insert_into(product)
                .values(&new_product)
                .execute(&db_connection)
                .expect("Error saving new product");

            let result = product.order(id.desc()).first(&db_connection).unwrap();

            Ok(result)
        }
    }
}

pub async fn get_all_product(
    pool: web::Data<Pool>,
) -> anyhow::Result<Vec<Product>, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();

    let result = product.load::<Product>(&db_connection)?;
    Ok(result)
}

pub async fn delete_product(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> anyhow::Result<usize, diesel::result::Error> {
    use crate::schema::product::dsl::*;
    let db_connection = pool.get().unwrap();

    let id_string = &path.0;
    let i: i32 = id_string.parse().unwrap();

    let result = delete(product.filter(id.eq(i))).execute(&db_connection)?;
    Ok(result)
}
