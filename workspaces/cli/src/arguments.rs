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

    /// Enable virtual display
    Enable(EnableArgs),

    /// Disable virtual display
    Disable,
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
    /// Name of the person to greet
    #[arg(short, long)]
    pub connector: Option<String>,
}
