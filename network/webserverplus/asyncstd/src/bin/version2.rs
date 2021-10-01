use async_std::channel::unbounded;
use async_std::net::TcpListener;
use asyncstd::{handle_connection, RequestResult};

#[async_std::main]
async fn main() -> async_std::io::Result<()> {
    let (kill_switch, kill_switch_receiver) = unbounded::<()>();

    let local_host = "127.0.0.1";
    let port = 8003;
    let listener = TcpListener::bind((local_host, port)).await?;

    let accept_loop = async_std::task::spawn(async move {
        while let Ok((stream, _addr)) = listener.accept().await {
            let kill_switch = kill_switch.clone();
            async_std::task::spawn(async move {
                if let Ok(RequestResult::Quit) = handle_connection(stream).await {
                    kill_switch.send(()).await.unwrap();
                }
            });
        }
    });

    println!(
        "server started at http://{}:{}/ serving files in {:?}",
        local_host,
        port,
        std::env::current_dir().unwrap_or_default()
    );

    kill_switch_receiver.recv().await.unwrap();
    accept_loop.cancel().await;
    Ok(())
}
