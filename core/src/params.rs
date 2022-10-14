use jsonrpsee::core::traits::ToRpcParams;
use serde_json::value::RawValue;

pub struct EmptyObjectParams;

impl ToRpcParams for EmptyObjectParams {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, jsonrpsee::core::Error> {
        Ok(Some(RawValue::from_string("{}".to_owned())?))
    }
}
