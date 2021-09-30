/// 访问 localhost:7000 会代理到 crates.io 去

use anyhow::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Server};
use std::net::SocketAddr;
use std::sync::Arc;

fn proxy_crate(req: &mut Request<Body>) -> Result<()> {
    for key in &[
        "content-length",
        "accept-encoding",
        "content-encoding",
        "transfer-encoding",
    ] {
        req.headers_mut().remove(*key);
    }
    let uri = req.uri();
    let uri_string = match uri.query() {
        Some(query_item) => format!("https://crates.io/{}?{}", uri.path(), query_item),
        None => format!("https://crates.io/{}", uri.path()),
    };

    *req.uri_mut() = uri_string.parse().context("Parsing URI Error")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let client = Arc::new(client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 7000));
    let make_svc = make_service_fn(move |_| {
        let client = Arc::clone(&client);
        async move {
            Ok::<_, Error>(service_fn(move |mut req| {
                let client = Arc::clone(&client);
                async move {
                    println!("proxy: {}", req.uri().path());
                    proxy_crate(&mut req)?;
                    client.request(req).await.context("proxy request")
                }
            }))
        }
    });

    let _server = Server::bind(&addr).serve(make_svc).await.context("Run server");

    Ok::<(), anyhow::Error>(())
}
