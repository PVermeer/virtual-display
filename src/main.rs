mod gpu_info;

use anyhow::{Context, Result, bail};
use gpu_info::get_gpu_info;
use std::{fs, path::Path, str::FromStr};
use tracing::{Level, debug, error};
use tracing_subscriber::{FmtSubscriber, util::SubscriberInitExt};

fn get_log_level() -> Option<Level> {
    std::env::var("VD_LOG")
        .with_context(|| {
            let info = "No LOG environment variable set";
            println!("{info}");
            info
        })
        .and_then(|level_str| {
            Level::from_str(&level_str).with_context(|| {
                let error = format!("Invalid LOG environment variable set, using '{level_str}'");
                eprintln!("{error:?}");
                error
            })
        })
        .ok()
}

fn main() -> Result<()> {
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

    /* Logging */
    let mut log_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };
    log_level = get_log_level().unwrap_or(log_level);
    // Disable > info logging for external crates
    let filter = format!("{}={log_level},common={log_level}", "virtual_display");

    let logger = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter(filter)
        .finish();
    logger.init();

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

    println!("========DONE");

    Ok(())
}
