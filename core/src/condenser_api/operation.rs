use core::option::Option::Some;
use core::result::Result;
use core::result::Result::Ok;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use crate::{HiveEncode, HiveEncoder};
use crate::enc::{EncodeError, LEB128};
use crate::operation::{Custom, CustomJson};

#[derive(Debug)]
pub enum Operation {
    Custom(Custom),
    CustomJson(CustomJson),
}

impl Serialize for Operation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut arr = serializer.serialize_seq(Some(2))?;

        match self {
            Operation::Custom(custom) => {
                arr.serialize_element("custom_operation")?;
                arr.serialize_element(custom)?;
            }
            Operation::CustomJson(custom_json) => {
                arr.serialize_element("custom_json_operation")?;
                arr.serialize_element(custom_json)?;
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
        }

        Ok(())
    }
}
