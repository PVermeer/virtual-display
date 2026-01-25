use crate::{
    arguments::InfoArgs,
    socket::{handle_response, send_request},
};
use anyhow::Result;
use common::api::{EnableArgs, GpuInfo, Request, Response};
use tracing::{debug, instrument};

#[instrument(err)]
pub async fn display_info(arguments: &InfoArgs) -> Result<()> {
    debug!("Getting display info");

    let request = Request::Info;
    let response = send_request(request).await?;
    match response {
        Response::Ok(result) => {
            if arguments.json {
                println!("{result}");
            } else {
                let display_info: GpuInfo = serde_json::from_str(&result)?;

                println!("Connectors:\n");
                for info in display_info {
                    let connector_status = if info.connected {
                        "connected"
                    } else {
                        "available"
                    };
                    println!("{}: {connector_status}", info.name);
                }
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
