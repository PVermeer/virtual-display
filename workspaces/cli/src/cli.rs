use super::{
    arguments::Command,
    commands::{daemon::run_daemon_command, display::run_display_command},
};
use anyhow::Result;

pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::Daemon(subcommand) => run_daemon_command(&subcommand.subcommand).await,
        Command::Enable(_) | Command::Disable => run_display_command(&command).await,
    }
}
