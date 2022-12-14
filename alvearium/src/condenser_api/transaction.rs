use crate::condenser_api::operation::Operation;
use crate::crypto::private_key::PrivateKey;
use crate::database_api::TxSignProperties;
use crate::enc::encode_to_vec;
use crate::ser::serialize_hive_time;
use alvearium_derive::HiveEncode;
use chrono::{DateTime, Duration, Utc};
use hex_literal::hex;
use serde::Serialize;
use std::num::ParseIntError;

const DEFAULT_CHAIN_ID: [u8; 32] = hex!(
    "beeab0de" "00000000"
    "00000000" "00000000"
    "00000000" "00000000"
    "00000000" "00000000"
);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BlockchainMode {
    Reversible,
    Irreversible,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TransactionCreateError {
    PrefixDecodeError(DecodeHexError),
}

impl From<DecodeHexError> for TransactionCreateError {
    fn from(e: DecodeHexError) -> Self {
        TransactionCreateError::PrefixDecodeError(e)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DecodeHexError {
    OddLength,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

/*
https://stackoverflow.com/questions/52987181/how-can-i-convert-a-hex-string-to-a-u8-slice by Sven Marnach
 */
fn decode_hex(s: &str) -> Result<Vec<u8>, DecodeHexError> {
    if s.len() % 2 != 0 {
        Err(DecodeHexError::OddLength)
    } else {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}

impl UnsignedTransaction {
    pub fn new(
        properties: &TxSignProperties,
        operations: Vec<Operation>,
    ) -> Result<Self, TransactionCreateError> {
        let ref_block_num = properties.ref_block_num;
        let prefix =
            decode_hex(&properties.ref_block_prefix).map_err(TransactionCreateError::from)?;
        let mut prefix_bytes: [u8; 4] = Default::default();
        prefix_bytes.copy_from_slice(&prefix[4..8]);
        let ref_block_prefix = u32::from_le_bytes(prefix_bytes);

        Ok(UnsignedTransaction {
            operations,
            ref_block_num: ref_block_num as u16,
            ref_block_prefix,
            expiration: properties.time + Duration::minutes(10),
            extensions: vec![],
        })
    }

    pub fn sign(self, key: &PrivateKey, chain_id: Option<[u8; 32]>) -> Transaction {
        let v = encode_to_vec(&self).unwrap();
        let signature =
            key.sign_ecdsa_canonical([chain_id.unwrap_or(DEFAULT_CHAIN_ID).as_ref(), &v].concat());
        let (recovery_id, buf) = signature.serialize_compact();
        let mut buffer: [u8; 65] = [0; 65];

        // I guess recovery_id can never be larger than a u8 the code says it needs to be 0..=3
        buffer[0] = recovery_id.to_i32() as u8 + 31;
        buffer[1..].clone_from_slice(&buf);
        let signatures = vec![hex::encode(buffer)];

        Transaction {
            ref_block_num: self.ref_block_num,
            ref_block_prefix: self.ref_block_prefix,
            signatures,
            operations: self.operations,
            expiration: self.expiration,
            extensions: self.extensions,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Transaction {
    pub ref_block_num: u16,
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

#[derive(Serialize, Debug, HiveEncode)]
#[hive_encode(crate = "crate")]
pub struct UnsignedTransaction {
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    #[serde(
        deserialize_with = "deserialize_hive_time",
        serialize_with = "serialize_hive_time"
    )]
    pub expiration: DateTime<Utc>,
    pub operations: Vec<Operation>,
    pub extensions: Vec<()>,
}
