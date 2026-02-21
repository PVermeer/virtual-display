mod arguments;
mod cli;
mod commands;
mod socket;

use anyhow::Result;
use arguments::Cli;
use clap::Parser;
use common::logging;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    logging::init();

    let arguments = Cli::parse();
    cli::run(arguments.command).await?;

    Ok(())
}
