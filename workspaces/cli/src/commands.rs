use super::arguments::DeamonCommands;
use anyhow::{Context, Result, bail};
use common::api::{DAEMON_BIN, SYSTEMD_UNIT};
use std::{path::Path, process::Command};
use systemctl::SystemCtl;
use tracing::{debug, error};

pub fn run_daemon_command(command: &DeamonCommands) -> Result<()> {
    match command {
        DeamonCommands::Start => start_daemon(),
    }
}

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

    Ok(())
}
