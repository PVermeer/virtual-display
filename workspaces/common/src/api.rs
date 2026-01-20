use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Status,
    Reload,
    Stop,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Ok(String),
    Error(String),
}

pub const SOCKET_PATH: &str = "/run/virtual-display/virtual-display-daemon.sock";
