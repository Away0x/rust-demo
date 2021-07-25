use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post, put, routes};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: usize,
    name: String,
}

#[get("/")]
async fn user_list() -> Option<Json<Vec<User>>> {
    let list = vec![
        User {
            id: 1,
            name: "wt1".to_string(),
        },
        User {
            id: 2,
            name: "wt2".to_string(),
        },
    ];

    Some(Json(list))
}

#[get("/<id>")]
async fn user_detail(id: usize) -> Option<Json<User>> {
    Some(Json(User {
        id,
        name: "wt1".to_string(),
    }))
}

#[post("/", format = "json", data = "<user>")]
async fn new_user(user: Json<User>) -> Option<Json<User>> {
    Some(user)
}

#[put("/<id>", format = "json", data = "<user>")]
async fn update_user(id: usize, mut user: Json<User>) -> Option<Json<User>> {
    user.id = id;
    Some(user)
}

#[delete("/<id>")]
async fn delete_user(id: usize) -> Value {
    json!({"status": "ok", "id": id})
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("USER", |rocket| async {
        rocket.mount(
            "/users",
            routes![user_list, user_detail, new_user, update_user, delete_user],
        )
    })
}
