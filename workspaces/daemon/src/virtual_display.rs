use super::status::get_gpu_info;
use crate::state;
use anyhow::{Result, bail};
use common::api::{EnableArgs, GpuConnector, Response};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, error, instrument};

static EDID: &[u8] = include_bytes!("../../../edids/HDR4k_120.bin");

#[derive(Serialize, Deserialize, Debug)]
struct ConnectorState {
    connector: GpuConnector,
    enabled: bool,
}

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

pub fn get_connector_from_state() -> Result<GpuConnector> {
    let connectors = state::get_all_state_by_type::<ConnectorState>()?;
    let connector = connectors.into_iter().next();
    let Some(connector) = connector else {
        bail!("Virtual display is not enabled");
    };

    Ok(connector.connector)
}

fn set_connector_to_state(connector: GpuConnector) -> Result<()> {
    state::set_state(
        &connector.name.clone(),
        &ConnectorState {
            enabled: true,
            connector,
        },
    )?;

    Ok(())
}

fn set_virtual_display(arguments: &EnableArgs) -> Result<String> {
    if let Ok(connector) = get_connector_from_state() {
        bail!("Virtual display already enabled on: {}", connector.name);
    }

    let connector = get_connector(arguments)?;
    let connector_name_clone = connector.name.clone();

    debug!(connector = connector.name, "Connecting virtual display");

    let debug_dri_dir = get_kernel_debug_dri_path()
        .join(connector.device_minor.to_string())
        .join(&connector.name);
    let edid_override_path = debug_dri_dir.join("edid_override");
    let force_on_path = debug_dri_dir.join("force");
    let trigger_hot_plug_path = debug_dri_dir.join("trigger_hotplug");

    debug!(edid_path = %edid_override_path.display(), "Writing EDID");
    fs::write(edid_override_path, EDID)?;

    debug!(force_on_path = %force_on_path.display(), "Writing force on");
    fs::write(force_on_path, "on")?;

    debug!(hot_plug_path = %trigger_hot_plug_path.display(), "Writing hot plug");
    fs::write(trigger_hot_plug_path, "1")?;

    set_connector_to_state(connector)?;

    Ok(connector_name_clone)
}

fn unset_virtual_display() -> Result<String> {
    let connector = get_connector_from_state()?;

    debug!(connector = connector.name, "Disabling virtual display");

    let debug_dri_dir = get_kernel_debug_dri_path()
        .join(connector.device_minor.to_string())
        .join(&connector.name);
    let force_on_path = debug_dri_dir.join("force");
    let trigger_hot_plug_path = debug_dri_dir.join("trigger_hotplug");

    debug!(force_on_path = %force_on_path.display(), "Writing force off");
    fs::write(force_on_path, "off")?;

    debug!(hot_plug_path = %trigger_hot_plug_path.display(), "Writing hot plug");
    fs::write(trigger_hot_plug_path, "1")?;

    state::remove_state(&connector.name)?;

    Ok(connector.name)
}
