use crate::{
    daemon::stop_daemon,
    gpu_info::status,
    virtual_display::{disable_virtual_display, enable_virtual_display},
};
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

pub async fn handle_requests(mut stream: UnixStream, shutdown_tx: Sender<()>) -> Result<()> {
    let request = read_request(&mut stream).await?;

    let response = match request {
        Request::Status => status()?,
        Request::Enable(arguments) => enable_virtual_display(&arguments)?,
        Request::Disable => disable_virtual_display()?,
        Request::Stop => stop_daemon(&shutdown_tx)?,
    };

    write_response(&mut stream, &response).await?;
    Ok(())
}
