use crate::de::deserialize_hive_time;
use alvearium_derive::HiveEncode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct Supply {
    pub amount: String,
    pub precision: u32,
    pub nai: String,
}

#[derive(Deserialize, Debug)]
pub struct DynamicGlobalProperties {
    pub id: u32,
    pub head_block_number: u32,
    pub head_block_id: String,
    #[serde(deserialize_with = "deserialize_hive_time")]
    pub time: DateTime<Utc>,
    pub current_witness: String,
    pub total_pow: u64,
    pub num_pow_witnesses: u64,
    pub virtual_supply: Supply,
    pub current_supply: Supply,
    pub init_hbd_supply: Supply,
    pub current_hbd_supply: Supply,
    pub total_vesting_fund_hive: Supply,
    pub total_vesting_shares: Supply,
    pub total_reward_fund_hive: Supply,
    pub total_reward_shares2: String,
    pub pending_rewarded_vesting_shares: Supply,
    pub pending_rewarded_vesting_hive: Supply,
    pub hbd_interest_rate: f64,
    pub hbd_print_rate: f64,
    pub maximum_block_size: u64,
    pub required_actions_partition_percent: u64,
    pub current_aslot: u64,
    pub recent_slots_filled: String,
    pub participation_count: u64,
    pub last_irreversible_block_num: u32,
    pub target_votes_per_period: Option<u64>,
    pub delegation_return_period: u64,
    pub reverse_auction_seconds: u64,
    pub available_account_subsidies: u64,
    pub hbd_stop_percent: u64,
    pub hbd_start_percent: u64,
    #[serde(deserialize_with = "deserialize_hive_time")]
    pub next_maintenance_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_hive_time")]
    pub last_budget_time: DateTime<Utc>,
    pub content_reward_percent: u64,
    pub vesting_reward_percent: u64,
    pub downvote_pool_percent: u64,
}
