use asynctokio::{handle_connection, RequestResult};
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let (dispatch_sender, mut dispatch_receiver) = unbounded_channel::<DispatchMessage>();
    let (kill_switch, kill_switch_receiver) = tokio::sync::oneshot::channel::<()>(); // 只发送一次的 channel

    let local_host = "127.0.0.1";
    let port = 8002;
    let listener = TcpListener::bind((local_host, port)).await?;
    let dispatch_sender1 = dispatch_sender.clone();

    let accept_loop = tokio::task::spawn(async move {
        // 处理多个异步事件
        select! {
            _ = async {
                while let Ok((stream, _addr)) = listener.accept().await {
                    dispatch_sender1.send(DispatchMessage::Connected(stream)).unwrap();
                }
            } => {}
            _ = kill_switch_receiver => {}
        }
    });

    println!(
        "server started at http://{}:{}/ serving files in {:?}",
        local_host,
        port,
        std::env::current_dir().unwrap_or_default()
    );

    while let Some(dispatch_message) = dispatch_receiver.recv().await {
        match dispatch_message {
            DispatchMessage::Connected(stream) => {
                let dispatch_sender = dispatch_sender.clone();
                tokio::task::spawn(async move {
                    if let Ok(RequestResult::Quit) = handle_connection(stream).await {
                        dispatch_sender.send(DispatchMessage::Quit).unwrap();
                    }
                });
            }
            DispatchMessage::Quit => {
                break;
            }
        }
    }

    // 主线程结束前 用 kill_switch 发消息给 accept_loop 让其停止
    // accept_loop.await 类似于线程的 join 等待异步任务退出
    kill_switch.send(()).unwrap();
    accept_loop.await?;
    Ok(())
}

#[derive(Debug)]
enum DispatchMessage {
    Connected(TcpStream),
    Quit,
}
