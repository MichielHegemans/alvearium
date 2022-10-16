pub mod operation;
pub mod transaction;
mod version;

use crate::params::EmptyArrayParams;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClient;
use serde_json::Value;
use transaction::Transaction;
pub use version::Version;

pub async fn broadcast_transaction(
    client: &HttpClient,
    transactions: &[Transaction],
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let response: Value = client
        .request("condenser_api.broadcast_transaction", transactions)
        .await?;

    Ok(response)
}

pub async fn get_version(client: &HttpClient) -> anyhow::Result<Version> {
    let response: Version = client
        .request("condenser_api.get_version", EmptyArrayParams)
        .await?;

    Ok(response)
}
