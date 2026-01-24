use anyhow::{Context, Result, bail};
use common::api::{DAEMON_BIN, Request, Response, SYSTEMD_UNIT};
use std::{path::Path, process::Command};
use systemctl::SystemCtl;
use tracing::{debug, error, instrument};

use crate::{arguments::DeamonCommands, socket::send_request};

pub async fn run_daemon_command(command: &DeamonCommands) -> Result<()> {
    match command {
        DeamonCommands::Start => start_daemon(),
        DeamonCommands::Stop => stop_daemon().await,
    }
}

#[instrument(err)]
fn start_daemon() -> Result<()> {
    let systemctl = SystemCtl::default();

    if cfg!(debug_assertions) {
        debug!("Starting daemon without systemd");

        let daemon_bin_path = Path::new("target")
            .join("debug")
            .join(DAEMON_BIN)
            .canonicalize()
            .unwrap();

        match Command::new(daemon_bin_path).spawn() {
            Ok(_result) => {
                println!("Started daemon");
                return Ok(());
            }
            Err(error) => {
                error!(?error, "Failed to run command");
                bail!(error)
            }
        }
    }

    let unit = systemctl
        .create_unit(SYSTEMD_UNIT)
        .context("Failed to load systemd unit")?;

    if unit.active {
        println!("Daemon is already running");
        return Ok(());
    }

    systemctl
        .restart(SYSTEMD_UNIT)
        .context("Failed to start daemon")?;

    println!("Started daemon");
    Ok(())
}

#[instrument(err)]
async fn stop_daemon() -> Result<()> {
    let systemctl = SystemCtl::default();

    if cfg!(debug_assertions) {
        debug!("Stopping daemon without systemd");

        let response = send_request(Request::Stop).await?;

        match response {
            Response::Ok(response) => println!("{response}"),
            Response::Error(error) => println!("{error}"),
        }

        return Ok(());
    }

    let unit = systemctl
        .create_unit(SYSTEMD_UNIT)
        .context("Failed to stop systemd unit")?;

    if !unit.active {
        println!("Daemon is not running");
        return Ok(());
    }

    systemctl
        .stop(SYSTEMD_UNIT)
        .context("Failed to stop daemon")?;

    println!("Stopped daemon");
    Ok(())
}
