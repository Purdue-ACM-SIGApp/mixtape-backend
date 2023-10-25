use mongodb::{options::{ClientOptions, ConnectionString}, Client};
use anyhow::Result;

pub mod models;

pub async fn connect(uri: &str) -> Result<Client> {
    let config = ClientOptions::parse_connection_string(ConnectionString::parse(uri)?)
        .await?;
    // config.tls = Some(Tls::Enabled(TlsOptions::default()));
    let client = Client::with_options(config)?;

    Ok(client)
}