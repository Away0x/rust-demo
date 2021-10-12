use hyper::{
    service::{make_service_fn, service_fn},
    Request, Server,
};
use std::sync::Arc;

mod context;
mod handler;
mod router;

pub type Response = hyper::Response<hyper::Body>;
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() {
    let some_state = "state".to_string();

    let mut router = router::Router::new();
    router.get("/test", Box::new(handler::test_handler));
    router.post("/send", Box::new(handler::send_handler));
    router.get("/params/:some_param", Box::new(handler::param_handler));

    let shared_router = Arc::new(router);
    let new_service = make_service_fn(move |_| {
        let app_state = context::AppState {
            state_thing: some_state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    let addr = "0.0.0.0:8080".parse().expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    let _ = server.await;
}

async fn route(
    router: Arc<router::Router>,
    req: Request<hyper::Body>,
    app_state: context::AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(context::Context::new(app_state, req, found_handler.params))
        .await;

    Ok(resp)
}
