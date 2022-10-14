use crate::block_api::get_block;
use crate::types::DynamicGlobalProperties;
use chrono::{DateTime, Utc};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClient;
use crate::params::EmptyObjectParams;

pub struct BlockMarker {
    pub ref_block_num: u32,
    pub ref_block_prefix: String,
}

pub struct TxSignProperties {
    pub latest: BlockMarker,
    pub irreversible: BlockMarker,
    pub time: DateTime<Utc>,
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

pub async fn get_tx_sign_properties(client: &HttpClient) -> anyhow::Result<TxSignProperties> {
    let properties = get_dynamic_global_properties(client).await?;
    let irreversible_block = get_block(client, properties.last_irreversible_block_num).await?;

    Ok(TxSignProperties {
        latest: BlockMarker {
            ref_block_num: properties.head_block_number,
            ref_block_prefix: properties.head_block_id,
        },
        irreversible: BlockMarker {
            ref_block_num: properties.last_irreversible_block_num,
            ref_block_prefix: irreversible_block.block_id,
        },
        time: properties.time,
    })
}
