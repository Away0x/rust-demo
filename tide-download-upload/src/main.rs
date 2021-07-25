use std::io::Error as IoError;
use std::path::Path;
use std::sync::Arc;

use async_std::{fs::OpenOptions, io};
use tempfile::TempDir;
use tide::prelude::*;
use tide::{Body, Request, Response, StatusCode};
use tide_fluent_routes::prelude::*;

#[derive(Clone)]
pub struct TempDirState {
    tempdir: Arc<TempDir>,
}

impl TempDirState {
    fn try_new() -> Result<Self, IoError> {
        Ok(Self {
            tempdir: Arc::new(tempfile::tempdir()?),
        })
    }

    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

#[async_std::main]
async fn main() -> Result<(), IoError> {
    tide::log::start();

    let mut app = tide::with_state(TempDirState::try_new()?);

    app.register(root().at("api/file/:file", |route| route.put(upload).get(download)));

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn upload(req: Request<TempDirState>) -> tide::Result {
    let path = req.param("file")?;
    let fs_path = req.state().path().join(path);

    // 创建文件
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&fs_path)
        .await?;
    // 写入文件内容
    let bytes_written = io::copy(req, file).await?;

    tide::log::debug!("file written", {
        bytes_written: bytes_written,
        path: fs_path.canonicalize()?.to_str(),
    });

    Ok(json!({
        "bytes_written": bytes_written,
        "path": fs_path.canonicalize()?.to_str(),
    })
    .into())
}

async fn download(req: Request<TempDirState>) -> tide::Result {
    let path = req.param("file")?;
    let fs_path = req.state().path().join(path);

    if let Ok(res) = Body::from_file(fs_path).await {
        Ok(res.into()) // 返回文件
    } else {
        Ok(Response::new(StatusCode::NotFound))
    }
}
