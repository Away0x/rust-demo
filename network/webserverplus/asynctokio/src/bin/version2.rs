use asynctokio::{handle_connection, RequestResult};
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let (kill_switch, mut kill_switch_receiver) = unbounded_channel::<()>();

    let local_host = "127.0.0.1";
    let port = 8002;
    let listener = TcpListener::bind((local_host, port)).await?;

    let accept_loop = tokio::task::spawn(async move {
        select! {
            _ = async {
                while let Ok((stream, _addr)) = listener.accept().await {
                    let kill_switch = kill_switch.clone();
                    tokio::task::spawn(async move {
                        if let Ok(RequestResult::Quit) = handle_connection(stream).await {
                            kill_switch.send(()).unwrap();
                        }
                    });
                }
            } => {}
            _ = kill_switch_receiver.recv() => {}
        }
    });

    println!(
        "server started at http://{}:{}/ serving files in {:?}",
        local_host,
        port,
        std::env::current_dir().unwrap_or_default()
    );

    accept_loop.await?;
    Ok(())
}
