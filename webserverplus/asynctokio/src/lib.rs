use std::time;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub enum RequestResult {
    Ok,
    Quit,
}

pub async fn handle_connection(mut stream: TcpStream) -> tokio::io::Result<RequestResult> {
    let mut str = String::new();
    tokio::io::BufReader::new(&mut stream)
        .read_line(&mut str)
        .await?;

    let strsubs: Vec<_> = str.split(" ").collect();
    if strsubs.len() < 3 {
        return Err(tokio::io::Error::from(tokio::io::ErrorKind::InvalidInput));
    }

    // let method = strsubs[0];
    let path = strsubs[1];

    let (path, query) = match path.find("?") {
        Some(pos) => (&path[..pos], &path[(pos + 1)..]),
        None => (path, ""),
    };

    if query == "sleep" {
        tokio::time::sleep(time::Duration::from_secs(4)).await;
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
        match tokio::fs::File::open(relative_path).await {
            Ok(mut f) => {
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).await?;
                tokio::io::copy(&mut f, &mut stream).await?;
            }
            Err(err) => {
                stream.write(format!("HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body>Not Found {} {}</body></html>", path, err).as_bytes()).await?;
            }
        }
    }
    stream.flush().await?;

    if query == "quit" {
        return Ok(RequestResult::Quit);
    }
    return Ok(RequestResult::Ok);
}
