pub async fn hello(_: tide::Request<()>) -> tide::Result {
    Ok("hello".into())
}

pub async fn world(_: tide::Request<()>) -> tide::Result {
    Ok("world".into())
}
