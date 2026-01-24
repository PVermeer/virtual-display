use crate::{
    arguments::Command,
    commands::{
        daemon::run_daemon_command,
        display::{disable_display, display_info, enable_display},
    },
};
use anyhow::Result;

pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::Info(arguments) => display_info(&arguments).await,
        Command::Daemon(subcommand) => run_daemon_command(&subcommand.subcommand).await,
        Command::Enable(arguments) => enable_display(&arguments).await,
        Command::Disable => disable_display().await,
    }
}
