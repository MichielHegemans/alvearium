use crate::crypto::public_key::PublicKey;
use crate::operation::{Asset, AuthorityType};
use alvearium_derive::HiveEncode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct AccountCreate {
    pub fee: Asset,
    pub creator: String,
    pub new_account_name: String,
    pub owner: AuthorityType,
    pub active: AuthorityType,
    pub posting: AuthorityType,
    pub memo_key: PublicKey,
    pub json_metadata: String,
}
