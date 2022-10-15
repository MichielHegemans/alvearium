use crate::block_api::operation::Operation;
use crate::de::deserialize_hive_time;
use crate::ser::serialize_hive_time;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    pub ref_block_num: u32,
    pub ref_block_prefix: u32,
    #[serde(
        deserialize_with = "deserialize_hive_time",
        serialize_with = "serialize_hive_time"
    )]
    pub expiration: DateTime<Utc>,
    pub operations: Vec<Operation>,
    pub signatures: Vec<String>,
    pub extensions: Vec<()>,
}
