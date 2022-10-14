pub mod block_api;
pub mod condenser_api;
pub mod crypto;
pub mod database_api;
pub mod transaction;
pub mod types;

mod de;
pub mod enc;
pub mod operation;
mod params;
mod ser;

use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
pub use enc::HiveEncode;
pub use enc::HiveEncoder;

pub fn create_default_client(target: impl AsRef<str>) -> Result<HttpClient, jsonrpsee::core::Error> {
    HttpClientBuilder::default().build(target)
}