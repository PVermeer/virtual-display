use crate::gpu_info::gpu_info;

use super::daemon::stop_daemon;
use anyhow::{Context, Result};
use common::api::{Request, Response};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
    sync::broadcast::Sender,
};
use tracing::{debug, info};

async fn read_request(stream: &mut UnixStream) -> Result<Request> {
    debug!("Parsing request");
    let mut buffer = vec![0; 1024];
    let n = stream
        .read(&mut buffer)
        .await
        .context("Failed to read from stream")?;
    let request: Request =
        serde_json::from_slice(&buffer[..n]).context("Failed to parse request")?;

    info!(?request, "Received request");

    Ok(request)
}

async fn write_response(stream: &mut UnixStream, response: &Response) -> Result<()> {
    debug!("Sending response");
    let data = serde_json::to_vec(response)?;
    stream
        .write_all(&data)
        .await
        .context("Failed to write data to stream")?;

    info!(?response, "Send response");

    Ok(())
}

/// # Errors
/// On failed to read or write data to a stream
pub async fn handle_requests(mut stream: UnixStream, shutdown_tx: Sender<()>) -> Result<()> {
    let request = read_request(&mut stream).await?;

    let response = match request {
        Request::Info => gpu_info()?,
        Request::Enable(_arguments) => Response::Ok("Enable".into()),
        Request::Disable => Response::Ok("Disable".into()),
        Request::Stop => stop_daemon(&shutdown_tx)?,
    };

    write_response(&mut stream, &response).await?;
    Ok(())
}
