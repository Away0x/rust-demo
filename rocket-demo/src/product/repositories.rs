use diesel::prelude::*;
use diesel::result::QueryResult;

use super::models::*;
use crate::schema::*;

pub struct ProductRepository;

impl ProductRepository {
    pub fn all(c: &SqliteConnection) -> QueryResult<Vec<Product>> {
        products::table.limit(100).load::<Product>(c)
    }

    pub fn get(c: &SqliteConnection, product_id: i32) -> QueryResult<Product> {
        products::table.find(product_id).get_result::<Product>(c)
    }

    pub fn create(c: &SqliteConnection, product: NewProduct) -> QueryResult<Product> {
        diesel::insert_into(products::table)
            .values(product)
            .execute(c)?;

        let lid = Self::last_id(c)?;

        Self::get(c, lid)
    }

    pub fn update(c: &SqliteConnection, new_product: Product) -> QueryResult<Product> {
        diesel::update(products::table.find(new_product.id))
            .set((
                products::name.eq(new_product.name.to_owned()),
                products::description.eq(new_product.description.to_owned()),
            ))
            .execute(c)?;

        Self::get(c, new_product.id)
    }

    pub fn delete(c: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(products::table.find(id)).execute(c)
    }

    fn last_id(c: &SqliteConnection) -> QueryResult<i32> {
        products::table
            .select(products::id)
            .order(products::id.desc())
            .first(c)
    }
}
