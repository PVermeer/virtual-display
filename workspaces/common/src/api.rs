use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Stop,
    Info,
    Enable(Enable),
    Disable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Enable {
    pub connector: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(String),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuInfo {
    pub connector: String,
    pub connected: bool,
    pub device_minor: i32,
}
pub type GpuInfoVec = Vec<GpuInfo>;

pub const SOCKET_PATH: &str = "/run/virtual-display/virtual-display-daemon.sock";
pub const SYSTEMD_UNIT: &str = "virtual-display-daemon.service";
pub const DAEMON_BIN: &str = "virtual-display-daemon";
