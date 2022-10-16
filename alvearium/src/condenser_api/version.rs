use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub blockchain_version: String,
    pub hive_revision: String,
    pub fc_revision: String,
    pub chain_id: String,
}
