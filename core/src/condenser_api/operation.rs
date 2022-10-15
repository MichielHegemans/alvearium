use crate::enc::{EncodeError, LEB128};
use crate::operation::{AccountCreate, AccountCreateWithDelegation, Custom, CustomJson};
use crate::{HiveEncode, HiveEncoder};
use core::option::Option::Some;
use core::result::Result;
use core::result::Result::Ok;
use serde::ser::{Serialize, SerializeSeq, Serializer};

#[derive(Debug)]
pub enum Operation {
    Custom(Custom),
    CustomJson(CustomJson),
    AccountCreate(AccountCreate),
    AccountCreateWithDelegation(AccountCreateWithDelegation),
}

impl Serialize for Operation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut arr = serializer.serialize_seq(Some(2))?;

        match self {
            Operation::Custom(custom) => {
                arr.serialize_element("custom")?;
                arr.serialize_element(custom)?;
            }
            Operation::CustomJson(custom_json) => {
                arr.serialize_element("custom_json")?;
                arr.serialize_element(custom_json)?;
            }
            Operation::AccountCreate(account_create) => {
                arr.serialize_element("account_create")?;
                arr.serialize_element(account_create)?;
            }
            Operation::AccountCreateWithDelegation(account_create_with_delegation) => {
                arr.serialize_element("account_create_with_delegation")?;
                arr.serialize_element(account_create_with_delegation)?;
            }
        }

        arr.end()
    }
}

impl HiveEncode for Operation {
    fn encode<E: HiveEncoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Operation::Custom(custom) => {
                HiveEncode::encode(&LEB128::from(15u8), encoder)?;
                HiveEncode::encode(&custom, encoder)?;
            }
            Operation::CustomJson(custom_json) => {
                HiveEncode::encode(&LEB128::from(18u8), encoder)?;
                HiveEncode::encode(&custom_json, encoder)?;
            }
            Operation::AccountCreate(account_create) => {
                HiveEncode::encode(&LEB128::from(9u8), encoder)?;
                HiveEncode::encode(&account_create, encoder)?;
            }
            Operation::AccountCreateWithDelegation(account_create_with_delegation) => {
                HiveEncode::encode(&LEB128::from(41u8), encoder)?;
                HiveEncode::encode(&account_create_with_delegation, encoder)?;
            }
        }

        Ok(())
    }
}
