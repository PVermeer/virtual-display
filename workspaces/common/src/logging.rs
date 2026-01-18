use anyhow::Context;
use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::{FmtSubscriber, util::SubscriberInitExt};

fn get_log_level() -> Option<Level> {
    std::env::var("VD_LOG")
        .with_context(|| {
            let info = "No LOG environment variable set";
            println!("{info}");
            info
        })
        .and_then(|level_str| {
            Level::from_str(&level_str).with_context(|| {
                let error = format!("Invalid LOG environment variable set, using '{level_str}'");
                eprintln!("{error:?}");
                error
            })
        })
        .ok()
}

pub fn init_logging() {
    /* Logging */
    let mut log_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };
    log_level = get_log_level().unwrap_or(log_level);
    // Disable > info logging for external crates
    let filter = format!("{}={log_level},common={log_level}", "virtual_display");

    let logger = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_env_filter(filter)
        .finish();
    logger.init();
}
