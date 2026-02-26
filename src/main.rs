mod client;
mod config;
mod endpoint;
mod models;
mod token;

use crate::{
    client::AuthenticatedClient,
    endpoint::Endpoint,
    models::consignment::{ConsignmentRequest, ConsignmentResponse},
};

fn main() -> anyhow::Result<()> {
    let mut client = AuthenticatedClient::new()?;
    println!("✔  Authenticated");
    let consignment_request = ConsignmentRequest::test();
    let json = client.post(
        Endpoint::Consignment,
        &[serde_json::to_value(consignment_request)?],
    )?;
    println!("✔  Request Body Posted");
    let consignment_response = serde_json::from_value::<ConsignmentResponse>(json)?;
    // println!("✔ ");
    println!("✔  ConsignmentResponse Parsed");
    client.delete(Endpoint::Consignment, consignment_response.id())?;
    println!("✔  Cosnsignment Deleted");
    Ok(())
}
