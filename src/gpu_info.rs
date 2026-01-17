use anyhow::{Context, Result, bail};
use std::{
    fs::{self},
    path::Path,
};
use tracing::{debug, error};

#[derive(Debug)]
pub struct GpuInfo {
    pub connector: String,
    pub connected: bool,
    pub device_minor: i32,
}

pub fn get_gpu_info() -> Result<Vec<GpuInfo>> {
    debug!("Getting GPU info");

    let mut gpu_info_vec = Vec::new();
    let drm_path = Path::new("/").join("sys").join("class").join("drm");
    let entries = fs::read_dir(drm_path)?
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().contains("card"));

    for entry in entries {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let Ok((card_name, connector_id)) =
            file_name.split_once('-').context("Failed to get card name")
        else {
            continue;
        };
        let status_path = entry.path().join("status");
        let Ok(connector_status) =
            fs::read_to_string(&status_path).map(|status| status.trim().to_string())
        else {
            debug!(path = ?status_path, "Failed to read connector status");
            continue;
        };
        if connector_status != "connected" && connector_status != "disconnected" {
            continue;
        }
        let is_connected = connector_status == "connected";
        let Ok(device_minor) = card_name
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
        else {
            let message = "Failed to parse int for device minor";
            error!(message);
            bail!(message)
        };

        let gpu_info = GpuInfo {
            connector: connector_id.to_string(),
            connected: is_connected,
            device_minor,
        };

        debug!(?gpu_info, "Found display connector");

        gpu_info_vec.push(gpu_info);
    }

    if gpu_info_vec.is_empty() {
        let message = "No display connectors found";
        error!(message);
        bail!(message)
    }

    Ok(gpu_info_vec)
}
