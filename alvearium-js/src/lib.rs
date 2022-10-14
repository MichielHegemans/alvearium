#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use alvearium_core::create_default_client;
use napi::bindgen_prelude::{Error, Result, Status};

#[napi(object)]
pub struct DynamicGlobalProperties {
    pub head_block_id: String,
}

impl From<alvearium_core::types::DynamicGlobalProperties> for DynamicGlobalProperties {
    fn from(dgp: alvearium_core::types::DynamicGlobalProperties) -> Self {
        Self {
            head_block_id: dgp.head_block_id,
        }
    }
}

#[napi]
pub async fn get_global_dynamic_properties() -> Result<DynamicGlobalProperties> {
    let client = create_default_client("https://api.hive.blog:443").map_err(|_| {
        Error::new(
            Status::GenericFailure,
            format!("Failed to create HttpClient"),
        )
    })?;

    let properties = alvearium_core::database_api::get_dynamic_global_properties(&client)
        .await
        .map_err(|_| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to get Dynamic Global Properties"),
            )
        })?;

    Ok(properties.into())
}
