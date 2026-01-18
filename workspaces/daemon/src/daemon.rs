use std::{
    fs::{self},
    os::unix::fs::PermissionsExt,
};

use anyhow::{Context, Result};
use common::api::{Request, Response, SOCKET_PATH};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
    signal::{
        ctrl_c,
        unix::{SignalKind, signal},
    },
};
use tracing::info;

async fn handle_client(mut stream: UnixStream) -> Result<()> {
    let mut buffer = vec![0; 1024];
    let n = stream
        .read(&mut buffer)
        .await
        .context("Failed to read from stream")?;
    let request: Request =
        serde_json::from_slice(&buffer[..n]).context("Failed to parse request")?;

    let response = match request {
        Request::Status => Response::Ok("Running".into()),
        Request::Reload => Response::Ok("Reloaded".into()),
        Request::Stop => Response::Ok("Stopping".into()),
    };

    let data = serde_json::to_vec(&response)?;
    stream.write_all(&data).await?;

    Ok(())
}

async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = ctrl_c() => {}
        _ = sigterm.recv() => {}
    }
}

pub async fn run_daemon() -> anyhow::Result<()> {
    let _ = std::fs::remove_file(SOCKET_PATH);

    let listener = UnixListener::bind(SOCKET_PATH).context("Failed to connect to socket")?;
    let mut permissions = fs::metadata(SOCKET_PATH).unwrap().permissions();
    permissions.set_mode(0o666);
    fs::set_permissions(SOCKET_PATH, permissions).unwrap();

    info!("Daemon listening on {}", SOCKET_PATH);

    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                tokio::spawn(handle_client(stream));
            }
            () = shutdown_signal() => {
                info!("Shutdown requested");
                break;
            }
        }
    }

    let _ = std::fs::remove_file(SOCKET_PATH);

    Ok(())
}
