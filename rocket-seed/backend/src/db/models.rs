use rocket::serde::Serialize;

use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub status: i32,
}

#[derive(AsChangeset)]
#[table_name = "task"]
pub struct UpdateTaskStatus {
    pub status: i32,
}
