use core_derive::HiveEncode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct VoteOperation {
    pub voter: String,
    pub author: String,
    pub permlink: String,
    pub weight: i32,
}

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct CustomJson {
    pub required_auths: Vec<String>,
    pub required_posting_auths: Vec<String>,
    pub id: String,
    pub json: String,
}

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct Custom {
    pub required_auths: Vec<String>,
    pub id: u32,
    pub data: String,
}
