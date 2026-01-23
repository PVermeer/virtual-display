use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the daemon commands
    Daemon(Daemon),

    /// Get daemon status
    Status,

    /// Reload configuration
    Reload,

    /// Ask daemon to stop
    Stop,
}

#[derive(Parser)]
pub struct Daemon {
    #[command(subcommand)]
    pub subcommand: DeamonCommands,
}

#[derive(Subcommand)]
pub enum DeamonCommands {
    /// Start the daemon (systemd)
    Start,
}
