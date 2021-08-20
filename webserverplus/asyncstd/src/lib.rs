use async_std::io::prelude::*;
use async_std::net::TcpStream;
use std::time;

pub enum RequestResult {
    Ok,
    Quit,
}

pub async fn handle_connection(mut stream: TcpStream) -> async_std::io::Result<RequestResult> {
    let mut str = String::new();
    async_std::io::BufReader::new(&mut stream)
        .read_line(&mut str)
        .await?;

    let strsubs: Vec<_> = str.split(" ").collect();
    if strsubs.len() < 3 {
        return Err(async_std::io::Error::from(
            async_std::io::ErrorKind::InvalidInput,
        ));
    }

    // let method = strsubs[0];
    let path = strsubs[1];

    let (path, query) = match path.find("?") {
        Some(pos) => (&path[..pos], &path[(pos + 1)..]),
        None => (path, ""),
    };

    if query == "sleep" {
        async_std::task::sleep(time::Duration::from_secs(4)).await;
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
        match async_std::fs::File::open(relative_path).await {
            Ok(mut f) => {
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).await?;
                async_std::io::copy(&mut f, &mut stream).await?;
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
