#![allow(clippy::wildcard_imports)]

use rocket_seed::{JsonApiResponse, Task};
use seed::{prelude::*, *};

struct Model {
    tasks: Vec<Task>,
}

#[derive(Debug)]
enum Msg {
    FetchedTasks(fetch::Result<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Ok(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(Err(err)) => {
            log!("Fetch err: {}", err);
            orders.skip();
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| {
            li![{
                format!(
                    "({}) {}: {}",
                    t.id,
                    t.title.clone(),
                    if t.status == 0 { "Not done" } else { "Done" }
                )
            }]
        })
        .collect();

    div![h1!["Tasks"], ul![tasks,],]
}

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async {
        let response = fetch("http://localhost:8000/tasks2")
            .await
            .expect("HTTP request failed");

        let data = response
            .check_status() // ensure we've got 2xx status
            .expect("status check failed")
            .json::<JsonApiResponse>()
            .await;

        Msg::FetchedTasks(data)
    });

    Model { tasks: vec![] }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
