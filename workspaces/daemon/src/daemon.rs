use super::requests::handle_requests;
use anyhow::{Context, Result};
use common::api::SOCKET_PATH;
use std::{
    fs::{self},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};
use tokio::{
    net::UnixListener,
    signal::{
        ctrl_c,
        unix::{SignalKind, signal},
    },
};
use tracing::{error, info};

fn connect_socket() -> Result<(UnixListener, PathBuf)> {
    info!("Setting up socket");

    let socket_path = Path::new(SOCKET_PATH);
    let socket_dir = socket_path
        .parent()
        .context("Failed to get socket directory")?;

    if !socket_dir.is_dir() {
        fs::create_dir_all(socket_dir).context("Failed to create socket dir")?;
    }
    if socket_path.try_exists().is_ok_and(|exists| exists) {
        error!(
            "Socket already exists, removing socket {}.",
            socket_path.display()
        );
        std::fs::remove_file(SOCKET_PATH).context(format!(
            "Failed to remove old socket file at {}",
            socket_path.display()
        ))?;
    }

    let listener = UnixListener::bind(socket_path).context("Failed to connect to socket")?;

    let mut permissions = fs::metadata(socket_path)
        .context("Failed to get meta data of socket")?
        .permissions();
    permissions.set_mode(0o666);

    fs::set_permissions(socket_path, permissions).context("Failed to set permissions on socket")?;

    Ok((listener, socket_path.to_path_buf()))
}

async fn stop_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigquit = signal(SignalKind::quit()).unwrap();

    tokio::select! {
        _ = ctrl_c() => {}
        _ = sigterm.recv() => {}
        _ = sigint.recv() => {}
        _ = sigquit.recv() => {}
    }
}

pub async fn run_daemon() -> Result<()> {
    info!("Starting daemon");

    let (listener, socket_path) = &connect_socket()?;
    info!("Daemon listening on {}", socket_path.display());

    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                tokio::spawn(handle_requests(stream));
            }
            () = stop_signal() => {
                info!("Stop requested");
                break;
            }
        }
    }

    match std::fs::remove_file(socket_path).context(format!(
        "Failed to remove socket file at {}",
        socket_path.display()
    )) {
        Ok(()) => (),
        Err(error) => error!("{error}"),
    }

    info!("Daemon stopped");

    Ok(())
}
