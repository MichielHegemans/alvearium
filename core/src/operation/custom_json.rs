use core_derive::HiveEncode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct CustomJson {
    pub required_auths: Vec<String>,
    pub required_posting_auths: Vec<String>,
    pub id: String,
    pub json: String,
}
