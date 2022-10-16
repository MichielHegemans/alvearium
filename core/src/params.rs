use jsonrpsee::core::traits::ToRpcParams;
use serde_json::value::RawValue;

pub struct EmptyObjectParams;
pub struct EmptyArrayParams;

impl ToRpcParams for EmptyObjectParams {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, jsonrpsee::core::Error> {
        Ok(Some(RawValue::from_string("{}".to_owned())?))
    }
}

impl ToRpcParams for EmptyArrayParams {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, jsonrpsee::core::Error> {
        Ok(Some(RawValue::from_string("[]".to_owned())?))
    }
}
