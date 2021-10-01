use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::fs;
use std::io::prelude::*;
use std::time::Duration;

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // 4 个线程

    // .incoming(): 会产生流序列的迭代器 (单个流表示客户端和服务端打开的一个连接)
    // .take(2): 接收两个请求后会停机
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // 单线程版本: server 会依次处理每一个请求，意味着它在完成第一个连接的处理之前不会处理第二个连接
        // handle_connection(stream);

        // 多线程版本 server
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // 1024 字节
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n"; // localhost:7878
    let sleep = b"GET /sleep HTTP/1.1\r\n"; // localhost:7878/sleep

    // 不是访问 / 路径的话，响应 404
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        // 模拟慢请求 (休眠 5s)
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
