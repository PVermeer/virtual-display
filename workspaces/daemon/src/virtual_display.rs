use super::gpu_info::get_gpu_info;
use anyhow::{Result, bail};
use common::api::{EnableArgs, GpuConnector, Response, STATE_DIR};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, error, instrument};

static EDID: &[u8] = include_bytes!("../../../edids/HDR4k_120.bin");

#[instrument(err)]
pub fn enable_virtual_display(arguments: &EnableArgs) -> Result<Response> {
    match set_virtual_display(arguments) {
        Ok(connector) => {
            let message = format!("Enabled virtual display on: {connector}");
            Ok(Response::Ok(message))
        }
        Err(error) => {
            error!(?error);
            Ok(Response::Error(error.to_string()))
        }
    }
}

#[instrument(err)]
pub fn disable_virtual_display() -> Result<Response> {
    match unset_virtual_display() {
        Ok(connector) => {
            let message = format!("Disabled virtual display on: {connector}");
            Ok(Response::Ok(message))
        }
        Err(error) => {
            error!(?error);
            Ok(Response::Error(error.to_string()))
        }
    }
}

fn get_kernel_debug_dri_path() -> PathBuf {
    Path::new("/")
        .join("sys")
        .join("kernel")
        .join("debug")
        .join("dri")
}

fn get_connector(arguments: &EnableArgs) -> Result<GpuConnector> {
    let gpu_info = get_gpu_info()?;

    if let Some(connector) = &arguments.connector {
        let Some(gpu_connector) = gpu_info
            .into_iter()
            .find(|gpu_info| gpu_info.name == *connector)
        else {
            bail!("Connector not found: {connector}");
        };
        if gpu_connector.connected {
            bail!("Connector already connected: {connector}");
        }

        Ok(gpu_connector)
    } else {
        let Some(gpu_connector) = gpu_info.into_iter().find(|gpu_info| !gpu_info.connected) else {
            bail!("No usused connector found, run 'virtual-display info' for more information");
        };

        Ok(gpu_connector)
    }
}

fn get_connector_from_state() -> Result<GpuConnector> {
    let gpu_info = get_gpu_info()?;
    let mut connector = None;

    for gpu_connector in gpu_info {
        if !gpu_connector.connected {
            continue;
        }

        let state_file = Path::new(STATE_DIR).join(&gpu_connector.name);
        if state_file.exists() {
            connector = Some(gpu_connector);
            break;
        }
    }

    let Some(connector) = connector else {
        bail!("Virtual display is not enabled");
    };

    Ok(connector)
}

fn set_virtual_display(arguments: &EnableArgs) -> Result<String> {
    let connector = get_connector(arguments)?;

    debug!(connector = connector.name, "Connecting virtual display");

    let debug_dri_dir = get_kernel_debug_dri_path()
        .join(connector.device_minor.to_string())
        .join(&connector.name);
    let edid_override_path = debug_dri_dir.join("edid_override");
    let force_on_path = debug_dri_dir.join("force");
    let trigger_hot_plug_path = debug_dri_dir.join("trigger_hotplug");

    fs::write(edid_override_path, EDID)?;
    fs::write(force_on_path, "on")?;
    fs::write(trigger_hot_plug_path, "1")?;

    let state_file = Path::new(STATE_DIR).join(&connector.name);
    fs::write(state_file, "1")?;

    Ok(connector.name)
}

fn unset_virtual_display() -> Result<String> {
    let connector = get_connector_from_state()?;

    debug!(connector = connector.name, "Disabling virtual display");

    let debug_dri_dir = get_kernel_debug_dri_path()
        .join(connector.device_minor.to_string())
        .join(&connector.name);
    let force_on_path = debug_dri_dir.join("force");
    let trigger_hot_plug_path = debug_dri_dir.join("trigger_hotplug");

    fs::write(force_on_path, "off")?;
    fs::write(trigger_hot_plug_path, "1")?;

    let state_file = Path::new(STATE_DIR).join(&connector.name);
    fs::remove_file(state_file)?;

    Ok(connector.name)
}
