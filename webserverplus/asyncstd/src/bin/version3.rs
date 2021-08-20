use async_std::channel::{unbounded as channel, Sender};
use async_std::fs::File;
use async_std::io::{copy, stdin, BufReader, Error, ErrorKind, Result};
use async_std::net::{TcpListener, TcpStream};
use async_std::path::PathBuf;
use async_std::task::sleep;
use async_std::task::spawn;
use async_std::task::JoinHandle;
use std::time::Duration;

use async_std::io::prelude::*;

/// start 启动 
/// stop 停止 
/// restart 重启 
/// quit 退出 
/// port 设置监听端口 
/// dir 设置响应文件根目录
/// stop 和 quit 可由 http 请求控制
#[async_std::main]
async fn main() -> Result<()> {
    let (cmd_sender, cmd_receiver) = channel::<Command>();

    let local_host = "127.0.0.1";
    let mut port = 20083;
    let mut dir = PathBuf::from(std::env::current_dir().unwrap_or_default());
    let mut server = Err(Error::from(ErrorKind::Other));

    cmd_sender.send(Command::Start).await.unwrap();
    let cmd_input_loop = spawn(start_cmd_input_loop(cmd_sender.clone()));

    while let Ok(cmd) = cmd_receiver.recv().await {
        match cmd {
            Command::Unknown => {}
            Command::Start => match server {
                Ok(_) => {
                    println!("started already");
                }
                Err(_) => {
                    println!("starting server");
                    server =
                        start_http_server(local_host, port, dir.clone(), cmd_sender.clone()).await;
                    match server {
                        Ok(_) => {
                            println!(
                                "server started at http://{}:{}/ serving files in {}",
                                local_host,
                                port,
                                dir.to_string_lossy()
                            );
                        }
                        Err(ref err) => {
                            println!("start server failed {}", err);
                        }
                    }
                }
            },
            Command::Stop => match server {
                Ok(accept_loop_handle) => {
                    println!("stopping");
                    accept_loop_handle.cancel().await;
                    server = Err(Error::from(ErrorKind::Other));
                    println!("stopped");
                }
                Err(_) => {
                    println!("stopped already");
                }
            },
            Command::Quit => {
                match server {
                    Ok(accept_loop_handle) => {
                        println!("stopping");
                        accept_loop_handle.cancel().await;
                        // server = Err(Error::from(ErrorKind::Other));
                        println!("stopped");
                    }
                    Err(_) => {}
                }
                println!("quitting");
                break;
            }
            Command::Port(new_port) => {
                let old_port = port;
                port = new_port;
                println!("port changed from {} to {}", old_port, new_port);
            }
            Command::Dir(new_dir) => {
                let old_dir = dir;
                dir = PathBuf::from(new_dir);
                println!(
                    "dir changed from {:?} to {:?}",
                    old_dir.to_string_lossy(),
                    dir.to_string_lossy()
                );
            }
        }
    }

    cmd_input_loop.cancel().await;
    Ok(())
}

enum Command {
    Unknown,
    Start,
    Stop,
    Quit,
    Port(u16),
    Dir(String),
}

impl From<&str> for Command {
    fn from(src: &str) -> Self {
        if src == "start" {
            Command::Start
        } else if src == "stop" {
            Command::Stop
        } else if src == "quit" {
            Command::Quit
        } else if src == "port" {
            Command::Port(0)
        } else if src == "dir" {
            Command::Dir(String::new())
        } else {
            Command::Unknown
        }
    }
}

async fn start_cmd_input_loop(cmd_sender: Sender<Command>) -> Result<()> {
    let stdin = stdin();
    loop {
        let mut line = String::new();
        //println!("cmd>");
        stdin
            .read_line(&mut line)
            .await
            .expect("Failed to read command");

        let parts = line.trim().split_whitespace().collect::<Vec<_>>();
        if parts.len() >= 1 {
            let cmd = Command::from(parts[0]);
            match cmd {
                Command::Port(_) => {
                    if parts.len() >= 2 {
                        match parts[1].parse() {
                            Ok(num) => {
                                cmd_sender.send(Command::Port(num)).await.unwrap();
                            }
                            Err(_) => {
                                println!("port command need an argument of 0~65535");
                            }
                        };
                    } else {
                        println!("port command need an argument of 0~65535");
                    }
                }
                Command::Dir(_) => {
                    if parts.len() >= 2 {
                        cmd_sender.send(Command::Dir(String::from(parts[1]))).await.unwrap();
                    } else {
                        println!("dir command need an argument of absolute path");
                    }
                }
                Command::Unknown => {
                    if (parts[0] == "restart") {
                        cmd_sender.send(Command::Stop).await.unwrap();
                        cmd_sender.send(Command::Start).await.unwrap();
                    } else {
                        println!("unknown command");
                        println!("start\nstop\nrestart\nquit\nport [num]\ndir [dir]\n");
                    }
                }
                cmd => {
                    cmd_sender.send(cmd).await.unwrap();
                }
            }
        }
    }
}

async fn start_http_server(
    host: &str,
    port: u16,
    dir: PathBuf,
    cmd_sender: Sender<Command>,
) -> Result<JoinHandle<()>> {
    let listener = TcpListener::bind((host, port)).await?;
    let accept_loop = spawn(async move {
        while let Ok((stream, _addr)) = listener.accept().await {
            let dir = dir.clone();
            let cmd_sender = cmd_sender.clone();
            spawn(async move {
                if let Ok(cmd) = handle_connection(stream, dir).await {
                    cmd_sender.send(cmd).await.unwrap();
                }
            });
        }
    });
    Ok(accept_loop)
}

async fn handle_connection(mut stream: TcpStream, dir: PathBuf) -> Result<Command> {
    let mut str = String::new();
    BufReader::new(&mut stream).read_line(&mut str).await?;

    let strsubs: Vec<_> = str.split(" ").collect();
    if strsubs.len() < 3 {
        return Err(Error::from(ErrorKind::InvalidInput));
    }
    // let method = strsubs[0];
    let path = strsubs[1];

    let (path, query) = match path.find("?") {
        Some(pos) => (&path[..pos], &path[(pos + 1)..]),
        None => (path, ""),
    };

    if query == "sleep" {
        sleep(Duration::new(4, 0)).await;
    }

    if path == "/" {
        stream
            .write("HTTP/1.1 200 OK\r\n\r\n<html><body>Welcome</body></html>".as_bytes())
            .await?;
    } else {
        let relative_path = match path.strip_prefix("/") {
            Some(p) => p,
            None => path,
        };
        match File::open(dir.join(relative_path)).await {
            Ok(mut f) => {
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).await?;
                copy(&mut f, &mut stream).await?;
            }
            Err(err) => {
                stream
                    .write(
                        format!(
                            "HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body>Not Found {} {}</body></html>",
                            path, err
                        )
                        .as_bytes(),
                    )
                    .await?;
            }
        }
    }
    stream.flush().await?;

    if query == "quit" {
        return Ok(Command::Quit);
    }
    if query == "stop" {
        return Ok(Command::Stop);
    }
    return Ok(Command::Unknown);
}
