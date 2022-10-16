pub mod block_api;
pub mod condenser_api;
pub mod crypto;
pub mod database_api;
pub mod types;

mod de;
pub mod enc;
pub mod operation;
mod params;
mod ser;

pub use enc::HiveEncode;
pub use enc::HiveEncoder;
use jsonrpsee::http_client::HttpClientBuilder;

pub use jsonrpsee::http_client::HttpClient;

pub fn create_default_client(
    target: impl AsRef<str>,
) -> Result<HttpClient, jsonrpsee::core::Error> {
    HttpClientBuilder::default().build(target)
}
