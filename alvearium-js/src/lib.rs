#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use alvearium_core::condenser_api::broadcast_transaction;
use alvearium_core::condenser_api::operation::Operation;
use alvearium_core::condenser_api::transaction::{
    BlockchainMode, UnsignedTransaction,
};
use alvearium_core::create_default_client;
use alvearium_core::crypto::private_key::PrivateKey;
use alvearium_core::database_api::get_tx_sign_properties;
use alvearium_core::HttpClient;
use alvearium_core::crypto::FromWif;
use napi::bindgen_prelude::{Error, Result, Status};

#[napi(js_name = "HiveClient")]
pub struct JsClient {
    client: HttpClient,
}

#[napi]
impl JsClient {
    #[napi(constructor)]
    pub fn new(target: String) -> Result<Self> {
        let client = create_default_client(&target).map_err(|_| {
            Error::new(
                Status::InvalidArg,
                format!("{} is not a valid hive node address", target),
            )
        })?;

        Ok(Self { client })
    }

    #[napi]
    pub async fn get_dynamic_global_properties(&self) -> Result<DynamicGlobalProperties> {
        let properties =
            alvearium_core::database_api::get_dynamic_global_properties(&self.client)
                .await
                .map_err(|_| {
                    Error::new(
                        Status::GenericFailure,
                        "Failed to get Dynamic Global Properties".to_owned(),
                    )
                })?;

        Ok(properties.into())
    }

    #[napi]
    pub async fn broadcast_custom_json(&self, custom_json: CustomJson, key: String) -> Result<()> {
        let sk = PrivateKey::from_wif(key).map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "Private key provided is not valid WIF format".to_owned(),
            )
        })?;

        let properties = get_tx_sign_properties(&self.client, BlockchainMode::Reversible)
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
        let transaction =
            UnsignedTransaction::new(&properties, vec![Operation::CustomJson(custom_json.into())])
                .map_err(|_| {
                    Error::new(
                        Status::GenericFailure,
                        "Failed to construct transaction".to_owned(),
                    )
                })?;

        let signed = transaction.sign(&sk, None);

        broadcast_transaction(&self.client, &vec![signed])
            .await
            .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

        Ok(())
    }
}

#[napi(object)]
pub struct CustomJson {
    pub id: String,
    pub required_auths: Vec<String>,
    pub required_posting_auths: Vec<String>,
    pub json: String,
}

impl Into<alvearium_core::operation::CustomJson> for CustomJson {
    fn into(self) -> alvearium_core::operation::CustomJson {
        alvearium_core::operation::CustomJson {
            id: self.id,
            required_auths: self.required_auths,
            required_posting_auths: self.required_posting_auths,
            json: self.json,
        }
    }
}

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
pub async fn get_dynamic_global_properties() -> Result<DynamicGlobalProperties> {
    let client = create_default_client("https://api.hive.blog:443").map_err(|_| {
        Error::new(
            Status::GenericFailure,
            "Failed to create HttpClient".to_owned(),
        )
    })?;

    let properties = alvearium_core::database_api::get_dynamic_global_properties(&client)
        .await
        .map_err(|_| {
            Error::new(
                Status::GenericFailure,
                "Failed to get Dynamic Global Properties".to_owned(),
            )
        })?;

    Ok(properties.into())
}
