use diesel::{prelude::*, sqlite::SqliteConnection};
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./database.sqlite";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection, title: Option<&str>) -> Vec<models::Task> {
    if let Some(title) = title {
        schema::task::table
            .filter(schema::task::title.eq(title))
            .load::<models::Task>(connection)
            .expect("Error loading tasks")
    } else {
        schema::task::table
            .load::<models::Task>(connection)
            .expect("Error loading tasks")
    }
}

pub fn update_tasks(connection: &SqliteConnection, filter_title: &str) -> Vec<models::Task> {
    use schema::task::dsl::*;

    diesel::update(task.filter(title.eq(filter_title)))
        .set((status.eq(1),))
        .execute(connection)
        .expect("Error update tasks");

    query_task(connection, Some(filter_title))
}

pub fn delete_tasks(connection: &SqliteConnection, filter_title: &str) {
    use schema::task::dsl::*;

    diesel::delete(task.filter(title.eq(filter_title)))
        .execute(connection)
        .expect("Error delete tasks");
}
