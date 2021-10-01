use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub title: String,
    pub data_created: String,
}

#[derive(Debug, Insertable)]
#[table_name = "product"]
pub struct PostProduct<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub data_created: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductJson {
    pub name: String,
    pub title: String,
}
