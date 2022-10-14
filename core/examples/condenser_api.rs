use alvearium_core::condenser_api::broadcast_transaction;
use alvearium_core::crypto::private_key::PrivateKey;
use alvearium_core::crypto::FromWif;
use alvearium_core::database_api::get_tx_sign_properties;
use alvearium_core::operation::{CondenserOperation, CustomJson};
use alvearium_core::transaction::{BlockchainMode, UnsignedCondenserTransaction};
use jsonrpsee::http_client::HttpClientBuilder;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let private_key =
        PrivateKey::from_wif(std::env::var("PRIV_KEY").expect("PRIV_KEY env var missing")).unwrap();
    let hive_name = std::env::var("HIVE_NAME").expect("HIVE_NAME env var missing");

    let filter = tracing_subscriber::EnvFilter::default().add_directive(
        "jsonrpsee[method_call{name = \"condenser_api.broadcast_transaction\"}]=trace".parse()?,
    );
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()?;

    match HttpClientBuilder::default().build("https://api.hive.blog:443") {
        Ok(client) => {
            let props = get_tx_sign_properties(&client, BlockchainMode::Reversible).await?;
            let trx = UnsignedCondenserTransaction::new(
                &props,
                vec![CondenserOperation::CustomJson(CustomJson {
                    id: "alvearium-core-test".to_owned(),
                    required_auths: vec![hive_name.to_string()],
                    required_posting_auths: vec![],
                    json: "{}".to_owned(),
                })],
            )
            .unwrap();

            let signed = trx.sign(&private_key);

            match broadcast_transaction(&client, &vec![signed]).await {
                Ok(res) => println!("{:?}", res),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}
