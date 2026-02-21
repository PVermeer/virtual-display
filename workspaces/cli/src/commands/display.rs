use crate::{
    arguments::StatusArgs,
    socket::{handle_response, send_request},
};
use anyhow::Result;
use common::api::{EnableArgs, Request, Response, Status};
use tracing::{debug, instrument};

#[instrument(err)]
pub async fn status(arguments: &StatusArgs) -> Result<()> {
    debug!("Getting status");

    let request = Request::Status;
    let response = send_request(request).await?;
    match response {
        Response::Ok(result) => {
            if arguments.json {
                println!("{result}");
            } else {
                let display_info: Status = serde_json::from_str(&result)?;
                println!("{display_info}");
            }
        }
        Response::Error(_) => {
            handle_response(response);
        }
    }

    Ok(())
}

#[instrument(err)]
pub async fn enable_display(arguments: &EnableArgs) -> Result<()> {
    debug!(?arguments, "Enabling virtual display");

    let request = Request::Enable(EnableArgs {
        connector: arguments.connector.clone(),
    });
    let response = send_request(request).await?;
    handle_response(response);

    Ok(())
}

#[instrument(err)]
pub async fn disable_display() -> Result<()> {
    debug!("Disabling virtual display");

    let request = Request::Disable;
    let response = send_request(request).await?;
    handle_response(response);

    Ok(())
}
