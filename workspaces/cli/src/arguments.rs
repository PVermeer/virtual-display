use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the daemon (systemd)
    Run,

    /// Get daemon status
    Status,

    /// Reload configuration
    Reload,

    /// Ask daemon to stop
    Stop,
}
