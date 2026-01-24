use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Get display info
    Info(InfoArgs),
    /// Run daemon commands
    Daemon(Daemon),
    /// Enable virtual display
    Enable(EnableArgs),
    /// Disable virtual display
    Disable,
}

#[derive(Parser, Debug)]
pub struct InfoArgs {
    /// Print json
    #[arg(long)]
    pub json: bool,
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
    /// Stop the daemon (systemd)
    Stop,
}

#[derive(Parser, Debug)]
pub struct EnableArgs {
    /// Name of the display connector
    #[arg(short, long)]
    pub connector: Option<String>,
}
