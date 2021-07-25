use rocket::serde::{Deserialize, Serialize};

use crate::schema::products;

#[derive(Queryable, Deserialize, Serialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

// 添加 product 请求体的类型
#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub description: String,
}
