use core_derive::HiveEncode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct Vote {
    pub voter: String,
    pub author: String,
    pub permlink: String,
    pub weight: i32,
}
