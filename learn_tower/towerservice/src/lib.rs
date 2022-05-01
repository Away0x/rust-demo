use std::future::Future;
use std::io::Error;
use tokio::net;

#[derive(Debug)]
pub struct HttpRequest;

impl HttpRequest {
    pub fn path(&self) -> &str {
        "/"
    }
}

#[derive(Debug)]
pub struct HttpResponse;

impl HttpResponse {
    pub fn ok(_body: impl AsRef<str>) -> Self {
        HttpResponse
    }

    pub fn not_found() -> Self {
        HttpResponse
    }

    pub fn set_header(&mut self, _name: &str, _value: &str) {
        // todo!()
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub async fn new(addr: impl Into<String>) -> Self {
        Server { addr: addr.into() }
    }

    pub async fn run<T>(self, mut handler: T) -> Result<(), Error>
    where
        T: Handler<HttpRequest, Response = HttpResponse, Error = Error>,
    {
        let listener = net::TcpListener::bind(self.addr).await?;

        loop {
            let (mut stream, _addr) = listener.accept().await?;
            let request = read_http_request(&mut stream).await?;

            // Call the handler provided by the user
            match handler.call(request).await {
                Ok(_response) => write_http_response(&mut stream).await?,
                Err(error) => handle_error_somehow(error, &mut stream),
            }
        }
    }
}

async fn read_http_request(_stream: &mut net::TcpStream) -> Result<HttpRequest, Error> {
    Ok(HttpRequest)
}

async fn write_http_response(_stream: &mut net::TcpStream) -> Result<(), Error> {
    Ok(())
}

fn handle_error_somehow(error: Error, _stream: &mut net::TcpStream) {
    println!("{}", error);
}

pub trait Handler<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: Request) -> Self::Future;
}
