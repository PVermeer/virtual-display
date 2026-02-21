#![allow(unused)]

use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};

const STATE_DIR: &str = "/run/virtual-display/state";

pub fn init() -> Result<()> {
    fs::create_dir_all(STATE_DIR)?;
    Ok(())
}

pub fn set_state<T>(key: &str, value: &T) -> Result<()>
where
    T: ?Sized + Serialize,
{
    let json_string = serde_json::to_string(value)?;
    let state_file = Path::new(STATE_DIR).join(key);
    fs::write(state_file, json_string)?;

    Ok(())
}

pub fn get_state<T>(key: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let state_file = Path::new(STATE_DIR).join(key);
    dbg!(&state_file, "Get state file");
    let json_string = fs::read_to_string(state_file)?;
    let value: T = serde_json::from_str(&json_string)?;

    Ok(value)
}

pub fn remove_state(key: &str) -> Result<()> {
    let state_file = Path::new(STATE_DIR).join(key);
    fs::remove_file(state_file)?;

    Ok(())
}

pub fn get_all_state_by_type<T>() -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let mut states = Vec::new();

    for entry in fs::read_dir(STATE_DIR)? {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Ok(file_string) = fs::read_to_string(path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<T>(&file_string) else {
            continue;
        };

        states.push(value);
    }

    Ok(states)
}

pub fn get_all_state() -> Result<Vec<String>> {
    let mut states = Vec::new();

    for entry in fs::read_dir(STATE_DIR)? {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Ok(file_string) = fs::read_to_string(path) else {
            continue;
        };

        states.push(file_string);
    }

    Ok(states)
}
