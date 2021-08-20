use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::thread;
use std::time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (dispatch_sender, dispatch_receiver) = channel::<DispatchMessage>();

    let local_host = "127.0.0.1";
    let port = 8001;
    let listener = TcpListener::bind((local_host, port))?;
    let dispatch_sender1 = dispatch_sender.clone();

    // 用独立线程的 accept_loop 处理 listener.accept 将接入连接转发给主线程的 dispatch_loop

    let _accept_loop = thread::spawn(move || {
        while let Ok((stream, _addr)) = listener.accept() {
            dispatch_sender1
                .send(DispatchMessage::Connected(stream))
                .unwrap();
        }
    });
    println!(
        "server started at http://{}:{}/ serving files in {:?}",
        local_host,
        port,
        std::env::current_dir().unwrap_or_default()
    );

    // dispatch_loop 接受并处理来自 channel 的连接消息与退出消息，收到退出消息时退出循环
    while let Ok(dispatch_message) = dispatch_receiver.recv() {
        match dispatch_message {
            DispatchMessage::Connected(stream) => {
                let dispatch_sender = dispatch_sender.clone();
                thread::spawn(move || {
                    if let Ok(RequestResult::Quit) = handle_connection(stream) {
                        dispatch_sender.send(DispatchMessage::Quit).unwrap();
                    }
                });
            }
            // 退出
            DispatchMessage::Quit => {
                break;
            }
        }
    }

    // accept_loop.join().unwrap();
    Ok(())
}

#[derive(Debug)]
enum DispatchMessage {
    Connected(TcpStream),
    Quit,
}

enum RequestResult {
    Ok,
    Quit,
}

/// 文件请求 http://127.0.0.1:20083/abc.html 发送当前目录下 abc.html 的内容
/// 简单 query string 处理 /?sleep /abc.html?sleep 暂停4秒再发送响应
/// 正确的退出 处理 /?quit /abc.html?quit 会退出程序
fn handle_connection(mut stream: TcpStream) -> std::io::Result<RequestResult> {
    let mut str = String::new();
    BufReader::new(&stream).read_line(&mut str)?;

    let strsubs: Vec<_> = str.split(" ").collect();
    if strsubs.len() < 3 {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }

    // 解析请求头
    // let method = strsubs[0];
    let path = strsubs[1];

    let (path, query) = match path.find("?") {
        Some(pos) => (&path[..pos], &path[(pos + 1)..]),
        None => (path, ""),
    };

    // 模拟慢请求
    if query == "sleep" {
        thread::sleep(time::Duration::from_secs(4))
    }

    if path == "/" {
        write!(
            stream,
            "HTTP/1.1 200 OK\r\n\r\n<html><body>Welcome</body></html>"
        )?;
    } else {
        let relative_path = match path.strip_prefix("/") {
            Some(p) => p,
            None => path,
        };

        match File::open(relative_path) {
            Ok(mut f) => {
                write!(stream, "HTTP/1.1 200 OK\r\n\r\n")?;
                std::io::copy(&mut f, &mut stream)?;
            }
            Err(err) => {
                write!(
                    stream,
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body>Not Found {} {}</body></html>",
                    path, err
                )?;
            }
        }
    }
    stream.flush()?;

    if query == "quit" {
        return Ok(RequestResult::Quit);
    }

    return Ok(RequestResult::Ok);
}
