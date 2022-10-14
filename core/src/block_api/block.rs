use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::block_api::transaction::Transaction;
use crate::de::deserialize_hive_time;

#[derive(Deserialize, Debug)]
pub struct BlockResponse {
    pub block: Block,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    pub previous: String,
    #[serde(deserialize_with = "deserialize_hive_time")]
    pub timestamp: DateTime<Utc>,
    pub witness: String,
    pub transaction_merkle_root: String,
    pub witness_signature: String,
    pub transactions: Vec<Transaction>,
    pub block_id: String,
    pub signing_key: String,
    pub transaction_ids: Vec<String>,
    // TODO: Extensions?
}

#[derive(Deserialize, Debug)]
pub struct BlockHeaderResponse {
    pub header: BlockHeader,
}

#[derive(Deserialize, Debug)]
pub struct BlockHeader {
    pub previous: String,
    #[serde(deserialize_with = "deserialize_hive_time")]
    pub timestamp: DateTime<Utc>,
    pub witness: String,
    pub transaction_merkle_root: String,
    // TODO: Extensions?
}

#[derive(Deserialize, Debug)]
pub struct BlockRangeResponse {
    pub blocks: Vec<BlockHeader>,
}
