use anyhow::{Context, Result, bail};
use common::api::{GpuConnector, Response, Status};
use serde_json::json;
use std::{
    fs::{self},
    path::Path,
};
use tracing::{debug, error, instrument};

use crate::virtual_display::get_connector_from_state;

#[instrument(err)]
pub fn status() -> Result<Response> {
    match get_status() {
        Ok(status) => {
            let status_serialized = json!(status);
            Ok(Response::Ok(status_serialized.to_string()))
        }
        Err(error) => {
            error!(?error);
            Ok(Response::Error(error.to_string()))
        }
    }
}

fn get_status() -> Result<Status> {
    let gpu_info = get_gpu_info()?;
    let virtual_display_connector = get_connector_from_state()
        .ok()
        .map(|connector| connector.name);

    let status = Status {
        virtual_display_connector,
        gpu_info,
    };

    Ok(status)
}

pub fn get_gpu_info() -> Result<Vec<GpuConnector>> {
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

        let gpu_info = GpuConnector {
            name: connector_id.to_string(),
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
