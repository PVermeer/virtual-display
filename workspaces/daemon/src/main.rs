mod daemon;
mod gpu_info;
mod requests;
mod virtual_display;

use anyhow::{Result, bail};
use common::logging;
use gpu_info::get_gpu_info;
use std::{fs, path::Path};
use tracing::{debug, error};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    match sudo::escalate_if_needed() {
        Ok(_) => (),
        Err(error) => {
            error!(error);
            bail!(error.to_string())
        }
    }

    if cfg!(debug_assertions) {
        println!("======== Running debug build ========");
    }

    logging::init_logging();

    daemon::run_daemon().await?;

    Ok(())
}
