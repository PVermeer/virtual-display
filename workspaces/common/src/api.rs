use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Request {
    Status,
    Reload,
    Stop,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok(String),
    Error(String),
}

pub const SOCKET_PATH: &str = "/run/virtual-display-daemon.sock";
