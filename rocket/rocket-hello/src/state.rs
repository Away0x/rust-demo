use std::collections::HashMap;

use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::{delete, get, post, put, routes, State};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    id: usize,
    name: String,
}

type Users = Mutex<HashMap<usize, User>>;
type Messages<'r> = &'r State<Users>;

#[get("/")]
async fn user_list(messages: Messages<'_>) -> Option<Json<Vec<User>>> {
    let user_map = messages.lock().await.clone();
    let mut result: Vec<User> = vec![];

    for user in  user_map.values() {
        result.push(user.clone());
    }

    Some(Json(result))
}

#[get("/<id>")]
async fn user_detail(id: usize, messages: Messages<'_>) -> Option<Json<User>> {
    let user_map = messages.lock().await;
    let none_user = User {
        id: 0,
        name: "_".to_string(),
    };

    if id == 0 {
        Some(Json(none_user))
    } else {
        match user_map.get(&id) {
            None => Some(Json(none_user)),
            Some(u) => Some(Json(u.to_owned())),
        }
    }
}

#[post("/", format = "json", data = "<user>")]
async fn new_user(user: Json<User>, messages: Messages<'_>) -> Value {
    let mut user_map = messages.lock().await;
    let new_user = user.into_inner();

    if user_map.contains_key(&new_user.id) {
        json!({"status": false, "message": "数据已存在"})
    } else {
        user_map.insert(new_user.id, new_user);
        json!({"status": true, "message": "创建成功"})
    }
}

#[put("/<id>", format = "json", data = "<user>")]
async fn update_user(id: usize, user: Json<User>, messages: Messages<'_>) -> Value {
    let mut user_map = messages.lock().await;
    let update_user = user.into_inner();

    if user_map.contains_key(&id) {
        user_map.insert(update_user.id, update_user);
        json!({"status": true, "message": "更新成功"})
    } else {
        json!({"status": false, "message": "数据不存在"})
    }
}

#[delete("/<id>")]
async fn delete_user(id: usize, messages: Messages<'_>) -> Value {
    let mut user_map = messages.lock().await;

    if user_map.contains_key(&id) {
        user_map.remove(&id);
        json!({"status": true, "message": "删除成功"})
    } else {
        json!({"status": false, "message": "数据不存在"})
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("STATE", |rocket| async {
        rocket
            .mount(
                "/users_state",
                routes![user_list, user_detail, new_user, update_user, delete_user],
            )
            .manage(Users::new(HashMap::new()))
    })
}
