mod arguments;

use anyhow::{Context, Result};
use arguments::{Cli, Command};
use clap::Parser;
use common::api::{Request, Response, SOCKET_PATH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

async fn run_cli(cmd: Command) -> anyhow::Result<()> {
    let req = match cmd {
        Command::Status => Request::Status,
        Command::Reload => Request::Reload,
        Command::Stop => Request::Stop,
        Command::Run => unreachable!(),
    };

    let mut stream = UnixStream::connect(SOCKET_PATH)
        .await
        .context("Failed to connect to socket")?;
    stream
        .write_all(&serde_json::to_vec(&req)?)
        .await
        .context("Failed to write to stream")?;

    let mut buf = vec![0; 1024];
    let n = stream
        .read(&mut buf)
        .await
        .context("Failed to read from stream")?;
    let resp: Response = serde_json::from_slice(&buf[..n]).context("Failed to parse repsonse")?;

    match resp {
        Response::Ok(message) => println!("{message}"),
        Response::Error(error) => eprintln!("error: {error}"),
    }

    Ok(())
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => println!("No command given"),
        Some(command) => match command {
            Command::Run => println!("Not running Daemon from here"),
            _ => run_cli(command).await?,
        },
    }

    Ok(())
}
