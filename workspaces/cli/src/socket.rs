use std::path::Path;

use anyhow::{Context, Result, bail};
use common::api::{Request, Response, SOCKET_PATH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::error;

pub async fn send_request(request: Request) -> Result<Response> {
    let socket_path = Path::new(SOCKET_PATH);
    if !socket_path.exists() {
        let message = "Failed to connect to socket, is the daemon running?";
        error!(message);
        bail!(message);
    }

    let mut stream = UnixStream::connect(SOCKET_PATH)
        .await
        .context("Failed to connect to socket")?;
    stream
        .write_all(&serde_json::to_vec(&request)?)
        .await
        .context("Failed to write to stream")?;

    let mut buf = vec![0; 1024];
    let n = stream
        .read(&mut buf)
        .await
        .context("Failed to read from stream")?;
    let resp: Response = serde_json::from_slice(&buf[..n]).context("Failed to parse repsonse")?;

    Ok(resp)
}
