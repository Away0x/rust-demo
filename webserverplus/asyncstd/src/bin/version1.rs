use async_std::channel::unbounded;
use async_std::net::{TcpListener, TcpStream};
use asyncstd::{handle_connection, RequestResult};

#[async_std::main]
async fn main() -> async_std::io::Result<()> {
    let (dispatch_sender, dispatch_receiver) = unbounded::<DispatchMessage>();

    let local_host = "127.0.0.1";
    let port = 8003;
    let listener = TcpListener::bind((local_host, port)).await?;
    let dispatch_sender1 = dispatch_sender.clone();

    let _accept_loop = async_std::task::spawn(async move {
        while let Ok((stream, _addr)) = listener.accept().await {
            dispatch_sender1
                .send(DispatchMessage::Connected(stream))
                .await
                .unwrap();
        }
    });

    println!(
        "server started at http://{}:{}/ serving files in {:?}",
        local_host,
        port,
        std::env::current_dir().unwrap_or_default()
    );

    while let Ok(dispatch_message) = dispatch_receiver.recv().await {
        match dispatch_message {
            DispatchMessage::Connected(stream) => {
                let dispatch_sender = dispatch_sender.clone();
                async_std::task::spawn(async move {
                    if let Ok(RequestResult::Quit) = handle_connection(stream).await {
                        dispatch_sender.send(DispatchMessage::Quit).await.unwrap();
                    }
                });
            }
            DispatchMessage::Quit => {
                break;
            }
        }
    }

    // accept_loop.await?;
    Ok(())
}

#[derive(Debug)]
enum DispatchMessage {
    Connected(TcpStream),
    Quit,
}
