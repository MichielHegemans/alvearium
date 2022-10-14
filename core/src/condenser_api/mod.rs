pub mod operation;
pub mod transaction;

use transaction::Transaction;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClient;
use serde_json::Value;

pub async fn broadcast_transaction(
    client: &HttpClient,
    transactions: &[Transaction],
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let response: Value = client
        .request("condenser_api.broadcast_transaction", transactions)
        .await?;

    Ok(response)
}