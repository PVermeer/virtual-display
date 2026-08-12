mod daemon;
mod requests;
mod state;
mod status;
mod virtual_display;

use anyhow::{Result, bail};
use common::logging;
use tracing::error;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    match sudo::with_env(&["VD_LOG"]) {
        Ok(_) => (),
        Err(error) => {
            error!(error);
            bail!(error.to_string())
        }
    }

    if cfg!(debug_assertions) {
        println!("======== Running debug build ========");
    }

    logging::init();
    state::init()?;

    daemon::run_daemon().await?;

    Ok(())
}
