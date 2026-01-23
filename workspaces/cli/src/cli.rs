use super::{arguments::Command, commands::run_daemon_command};
use anyhow::{Context, Result};
use common::api::{Request, Response, SOCKET_PATH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

pub async fn run(command: Command) -> Result<()> {
    match command {
        Command::Daemon(subcommand) => run_daemon_command(&subcommand.subcommand),
        _ => run_display_commands(command).await,
    }
}

async fn run_display_commands(cmd: Command) -> Result<()> {
    let req = match cmd {
        Command::Status => Request::Status,
        Command::Reload => Request::Reload,
        Command::Stop => Request::Stop,
        _ => unreachable!(),
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
