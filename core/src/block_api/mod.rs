pub mod types;

use crate::block_api::types::{
    Block, BlockHeader, BlockHeaderResponse, BlockRangeResponse, BlockResponse,
};
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::core::Error;
use jsonrpsee::http_client::HttpClient;
use serde_json::json;
use serde_json::value::RawValue;
use std::ops::Range;

struct BlockRange(Range<u32>);
struct BlockNumber(u32);

impl ToRpcParams for BlockNumber {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
        Ok(Some(RawValue::from_string(
            json!({ "block_num": self.0 }).to_string(),
        )?))
    }
}

impl ToRpcParams for BlockRange {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
        Ok(Some(RawValue::from_string(
            json!({
                "starting_block_num": json!(self.0.start),
                "count": json!(self.0.end - self.0.start),
            })
            .to_string(),
        )?))
    }
}

pub async fn get_block(client: &HttpClient, n: u32) -> anyhow::Result<Block> {
    let response: BlockResponse = client
        .request("block_api.get_block", BlockNumber(n))
        .await?;

    Ok(response.block)
}

pub async fn get_block_header(client: &HttpClient, n: u32) -> anyhow::Result<BlockHeader> {
    let response: BlockHeaderResponse = client
        .request("block_api.get_block_header", BlockNumber(n))
        .await?;

    Ok(response.header)
}

pub async fn get_block_range(
    client: &HttpClient,
    r: Range<u32>,
) -> anyhow::Result<Vec<BlockHeader>> {
    let response: BlockRangeResponse = client
        .request("block_api.get_block_range", BlockRange(r))
        .await?;

    Ok(response.blocks)
}
