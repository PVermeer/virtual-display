use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Enable(Enable),
    Reload,
    Stop,
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

pub const SOCKET_PATH: &str = "/run/virtual-display/virtual-display-daemon.sock";
pub const SYSTEMD_UNIT: &str = "virtual-display-daemon.service";
pub const DAEMON_BIN: &str = "virtual-display-daemon";
