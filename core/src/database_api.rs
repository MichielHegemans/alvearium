use crate::block_api::get_block;
use crate::condenser_api::transaction::BlockchainMode;
use crate::params::EmptyObjectParams;
use crate::types::DynamicGlobalProperties;
use chrono::{DateTime, Utc};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClient;

pub struct TxSignProperties {
    pub time: DateTime<Utc>,
    pub ref_block_num: u32,
    pub ref_block_prefix: String,
}

pub async fn get_dynamic_global_properties(
    client: &HttpClient,
) -> anyhow::Result<DynamicGlobalProperties> {
    let response: DynamicGlobalProperties = client
        .request(
            "database_api.get_dynamic_global_properties",
            EmptyObjectParams,
        )
        .await?;

    Ok(response)
}

pub async fn get_tx_sign_properties(
    client: &HttpClient,
    mode: BlockchainMode,
) -> anyhow::Result<TxSignProperties> {
    let properties = get_dynamic_global_properties(client).await?;

    match mode {
        BlockchainMode::Irreversible => {
            let irreversible_block =
                get_block(client, properties.last_irreversible_block_num).await?;
            Ok(TxSignProperties {
                time: properties.time,
                ref_block_num: properties.last_irreversible_block_num,
                ref_block_prefix: irreversible_block.block_id,
            })
        }
        BlockchainMode::Reversible => Ok(TxSignProperties {
            time: properties.time,
            ref_block_num: properties.head_block_number,
            ref_block_prefix: properties.head_block_id,
        }),
    }
}
