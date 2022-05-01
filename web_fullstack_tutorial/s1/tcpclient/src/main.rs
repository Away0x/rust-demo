use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    // 发送消息给 server
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    // 读取 server 发送的消息
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        str::from_utf8(&buffer).unwrap(),
    );
}
