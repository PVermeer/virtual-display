mod arguments;
mod cli;
mod commands;

use anyhow::Result;
use arguments::Cli;
use clap::Parser;
use common::logging::init_logging;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    init_logging();

    let arguments = Cli::parse();
    cli::run(arguments.command).await?;

    Ok(())
}
