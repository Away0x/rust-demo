#![allow(clippy::wildcard_imports)]

use rocket_seed::{JsonApiResponse, Task};
use seed::{prelude::*, *};

struct Model {
    tasks: Vec<Task>,
}

#[derive(Clone, Debug)]
enum Msg {
    FetchedTasks(Option<JsonApiResponse>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedTasks(Some(mut result)) => {
            model.tasks.clear();
            model.tasks.append(&mut result.data);
        }
        Msg::FetchedTasks(None) => {
            model.tasks.clear();
            // orders.skip();
            model.tasks.append(&mut vec![
                Task {
                    id: 0,
                    title: "Error".to_string(),
                    status: 1,
                }
            ]);
        }
    }
}

fn view(model: &Model) -> impl View<Msg> {
    let tasks: Vec<Node<Msg>> = model
        .tasks
        .iter()
        .map(|t| li![{ t.title.clone() }])
        .collect();

    h1![{ "Tasks" }, ul![tasks,],]
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

        match data {
            Ok(res) => Msg::FetchedTasks(Some(res)),
            Err(err) => {
                log!(format!("Error fetching: {:?}", err));
                Msg::FetchedTasks(None)
            },
        }
    });

    Model { tasks: vec![] }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
