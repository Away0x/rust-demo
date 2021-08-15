use super::build_rocket_app;
use rocket::http::Status;

#[rocket::async_test]
async fn test_hello() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(build_rocket_app()).await.unwrap();
    let req = client.get("/");
    // 两次请求
    let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
    assert_eq!(r1.status(), r2.status());
    assert_eq!(r1.status(), Status::Ok);

    let (s1, s2) = (r1.into_string().await, r2.into_string().await);
    assert_eq!(s1, s2);
    assert_eq!(s1.unwrap(), "Hello, world!");
}
