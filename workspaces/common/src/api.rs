use clap::Parser;
use serde::{Deserialize, Serialize};

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
pub type GpuInfo = Vec<GpuConnector>;

pub const STATE_DIR: &str = "/run/virtual-display";
pub const SOCKET_PATH: &str = "/run/virtual-display/virtual-display-daemon.sock";
pub const SYSTEMD_UNIT: &str = "virtual-display-daemon.service";
pub const DAEMON_BIN: &str = "virtual-display-daemon";
