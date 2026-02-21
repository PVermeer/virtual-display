use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Stop,
    Status,
    Enable(EnableArgs),
    Disable,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct EnableArgs {
    /// Name of the display connector
    #[arg(short, long)]
    pub connector: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(String),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuConnector {
    pub name: String,
    pub connected: bool,
    pub device_minor: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub virtual_display_connector: Option<String>,
    pub gpu_info: Vec<GpuConnector>,
}
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Virtual display: {}",
            self.virtual_display_connector
                .clone()
                .unwrap_or("not connected".into())
        )?;
        writeln!(f)?;

        writeln!(f, "Connectors:")?;
        for info in &self.gpu_info {
            let connector_status = if info.connected {
                "connected"
            } else {
                "available"
            };
            writeln!(f, "{}: {connector_status}", info.name)?;
        }

        Ok(())
    }
}

pub const SOCKET_PATH: &str = "/run/virtual-display/virtual-display-daemon.sock";
pub const SYSTEMD_UNIT: &str = "virtual-display-daemon.service";
pub const DAEMON_BIN: &str = "virtual-display-daemon";
