use crate::{
    arguments::{EnableArgs, InfoArgs},
    socket::{handle_response, send_request},
};
use anyhow::Result;
use common::api::{Enable, GpuInfoVec, Request, Response};
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
                let display_info: GpuInfoVec = serde_json::from_str(&result)?;

                println!("Connectors:\n");
                for info in display_info {
                    let connector_status = if info.connected {
                        "connected"
                    } else {
                        "available"
                    };
                    println!("{}: {connector_status}", info.connector);
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

    let request = Request::Enable(Enable {
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
