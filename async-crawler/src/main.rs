use futures::future::join_all;

async fn fetch_path(path: String) -> Result<String, reqwest::Error> {
    let mut result = String::new();

    match reqwest::get(&path).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("{:?}", text);
                println!("len {}", text.len());
                result = format!("url: {}, len: {}", path, text.len());
            }
            Err(_) => {
                println!("error while reading {}", path);
            }
        },
        Err(_) => {
            println!("error while scraping from {}", path)
        }
    }

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let paths = vec![
        "http://www.httpbin.org/ip".to_string(),
        "http://www.httpbin.org/get".to_string(),
    ];

    let result = join_all(paths.into_iter().map(|path| fetch_path(path)))
        .await
        .into_iter()
        .map(|r| {
            if r.is_ok() {
                r.unwrap()
            } else {
                "error".to_string()
            }
        })
        .collect::<Vec<String>>();

    println!("{:?}", result);

    Ok(())
}
