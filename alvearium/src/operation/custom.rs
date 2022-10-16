use alvearium_derive::HiveEncode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct Custom {
    pub required_auths: Vec<String>,
    pub id: u32,
    pub data: String,
}
