mod client;
mod config;
mod endpoint;
mod token;

use crate::client::AuthenticatedClient;

fn main() -> anyhow::Result<()> {
    #[allow(unused_mut, unused_variables)]
    let mut client = AuthenticatedClient::new()?;
    println!("✔  Authenticated");
    Ok(())
}
