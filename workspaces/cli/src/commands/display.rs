use crate::{
    arguments::{Command, EnableArgs},
    socket::send_request,
};
use anyhow::{Result, bail};
use common::api::{Enable, Request, Response};
use tracing::debug;

pub async fn run_display_command(command: &Command) -> Result<()> {
    match command {
        Command::Enable(arguments) => enable_display(arguments).await,
        Command::Disable => todo!(),
        Command::Daemon(_) => bail!("Command should not end up here!"),
    }
}

async fn enable_display(arguments: &EnableArgs) -> Result<()> {
    debug!(?arguments, "Enabling display");

    let request = Request::Enable(Enable {
        connector: arguments.connector.clone(),
    });

    let response = send_request(request).await?;

    match response {
        Response::Ok(message) => println!("{message}"),
        Response::Error(error) => eprintln!("error: {error}"),
    }

    Ok(())
}
