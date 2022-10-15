mod account_create;
mod account_create_with_delegation;
mod custom;
mod custom_json;
mod vote;

pub use account_create::AccountCreate;
pub use account_create_with_delegation::AccountCreateWithDelegation;
pub use custom::Custom;
pub use custom_json::CustomJson;
pub use vote::Vote;

use crate::crypto::public_key::PublicKey;
use crate::enc::{encode_without_size, EncodeError};
use crate::{HiveEncode, HiveEncoder};
use core_derive::HiveEncode;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct AuthorityType {
    pub weight_threshold: u32,
    pub account_auths: Vec<(PublicKey, u16)>,
    pub key_auths: Vec<(PublicKey, u16)>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum AssetSymbol {
    HIVE,
    VESTS,
    HBD,
    TESTS,
    TBD,
    STEEM,
    SBD,
}

impl Display for AssetSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetSymbol::HIVE => f.write_str("HIVE")?,
            AssetSymbol::VESTS => f.write_str("VESTS")?,
            AssetSymbol::STEEM => f.write_str("STEEM")?,
            AssetSymbol::HBD => f.write_str("HBD")?,
            AssetSymbol::TESTS => f.write_str("TESTS")?,
            AssetSymbol::TBD => f.write_str("TBD")?,
            AssetSymbol::SBD => f.write_str("SBD")?,
        }

        Ok(())
    }
}

pub struct NaiError;

impl AssetSymbol {
    pub fn try_from_nai(nai: &str) -> Result<AssetSymbol, NaiError> {
        let symbol = match nai {
            "@@00000013" => AssetSymbol::HBD,
            "@@00000021" => AssetSymbol::STEEM,
            "@@00000037" => AssetSymbol::VESTS,
            _ => return Err(NaiError),
        };

        Ok(symbol)
    }
}

impl HiveEncode for AssetSymbol {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let data: [u8; 7] = match self {
            AssetSymbol::HIVE => [b'S', b'T', b'E', b'E', b'M', 0, 0],
            AssetSymbol::STEEM => [b'S', b'T', b'E', b'E', b'M', 0, 0],
            AssetSymbol::HBD => [b'S', b'B', b'D', 0, 0, 0, 0],
            AssetSymbol::SBD => [b'S', b'B', b'D', 0, 0, 0, 0],
            AssetSymbol::VESTS => [b'V', b'E', b'S', b'T', b'S', 0, 0],
            AssetSymbol::TESTS => [b'T', b'E', b'S', b'T', b'S', 0, 0],
            AssetSymbol::TBD => [b'T', b'B', b'D', 0, 0, 0, 0],
        };

        encode_without_size(&data, encoder)?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub amount: i64,
    pub symbol: AssetSymbol,
}

impl Asset {
    pub fn new(amount: i64, symbol: AssetSymbol) -> Self {
        Self { amount, symbol }
    }

    pub fn precision(&self) -> u8 {
        match self.symbol {
            AssetSymbol::VESTS => 6,
            _ => 3,
        }
    }
}

impl HiveEncode for Asset {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let amount = self.amount as f64 * 10f64.powf(self.precision() as f64);

        HiveEncode::encode(&(amount as i64), encoder)?;
        HiveEncode::encode(&self.precision(), encoder)?;
        HiveEncode::encode(&self.symbol, encoder)?;

        Ok(())
    }
}

impl Serialize for Asset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Asset price needs to be serialized as "100.000 HIVE" or "100.000000 VESTS" for example
        serializer.serialize_str(&format!(
            "{amount:.*} {}",
            self.precision() as usize,
            self.symbol,
            amount = self.amount as f64
        ))
    }
}
