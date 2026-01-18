mod daemon;
mod gpu_info;

use anyhow::{Result, bail};
use common::logging;
use gpu_info::get_gpu_info;
use std::{fs, path::Path};
use tracing::{debug, error};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    match sudo::escalate_if_needed() {
        Ok(_) => (),
        Err(error) => {
            error!(error);
            bail!(error.to_string())
        }
    }

    if cfg!(debug_assertions) {
        println!("======== Running debug build ========");
    }

    logging::init_logging();

    daemon::run_daemon().await?;

    // enable_virtual_display()?;

    println!("========DONE");

    Ok(())
}

fn enable_virtual_display() -> Result<()> {
    let gpu_info = get_gpu_info()?;

    let Some(empty_connector) = gpu_info
        .iter()
        .find(|gpu_connector| !gpu_connector.connected)
    else {
        let message = "No empty connector on gpu found";
        error!(message);
        bail!(message);
    };

    debug!(connector = empty_connector.connector, "Connecting");

    let debug_dri_dir = Path::new("/")
        .join("sys")
        .join("kernel")
        .join("debug")
        .join("dri")
        .join(empty_connector.device_minor.to_string())
        .join(&empty_connector.connector);
    let edid_override_path = debug_dri_dir.join("edid_override");
    let force_on_path = debug_dri_dir.join("force");
    let trigger_hot_plug_path = debug_dri_dir.join("trigger_hotplug");
    let edid_path = Path::new("edids").join("HDR4k_120.bin");

    let Ok(edid) = fs::read(&edid_path) else {
        let message = "Failed to read edid";
        error!(path = ?edid_path, message);
        bail!(message);
    };

    fs::write(edid_override_path, edid)?;
    fs::write(force_on_path, "on")?;
    fs::write(trigger_hot_plug_path, "1")?;

    Ok(())
}
