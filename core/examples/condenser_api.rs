use alvearium_core::condenser_api::broadcast_transaction;
use alvearium_core::condenser_api::operation::Operation;
use alvearium_core::condenser_api::transaction::BlockchainMode;
use alvearium_core::condenser_api::transaction::UnsignedTransaction;
use alvearium_core::crypto::private_key::PrivateKey;
use alvearium_core::crypto::FromWif;
use alvearium_core::database_api::{get_tx_sign_properties, TxSignProperties};
use alvearium_core::operation::{AccountCreate, Asset, AssetSymbol, AuthorityType, CustomJson};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use rand::rngs::OsRng;
use secp256k1::{All, Secp256k1};
use tracing_subscriber::util::SubscriberInitExt;

async fn submit_custom_json(client: &HttpClient, props: &TxSignProperties) -> anyhow::Result<()> {
    let private_key =
        PrivateKey::from_wif(std::env::var("PRIV_KEY").expect("PRIV_KEY env var missing")).unwrap();
    let hive_name = std::env::var("HIVE_NAME").expect("HIVE_NAME env var missing");

    let trx = UnsignedTransaction::new(
        &props,
        vec![Operation::CustomJson(CustomJson {
            id: "alvearium-core-test".to_owned(),
            required_auths: vec![hive_name.to_string()],
            required_posting_auths: vec![],
            json: "{}".to_owned(),
        })],
    )
    .unwrap();

    let signed = trx.sign(&private_key, None);

    match broadcast_transaction(&client, &vec![signed]).await {
        Ok(res) => println!("{:?}", res),
        Err(e) => eprintln!("{:?}", e),
    }

    Ok(())
}

fn key_to_authority_type(key: PrivateKey) -> AuthorityType {
    AuthorityType {
        weight_threshold: 1,
        account_auths: vec![],
        key_auths: vec![(key.create_public(None), 1)],
    }
}

fn create_key(secp: &Secp256k1<All>) -> PrivateKey {
    let (key, _) = secp.generate_keypair(&mut OsRng);
    PrivateKey::from_key(key, None)
}

async fn submit_account_create(
    client: &HttpClient,
    props: &TxSignProperties,
) -> anyhow::Result<()> {
    let private_key =
        PrivateKey::from_wif(std::env::var("PRIV_KEY").expect("PRIV_KEY env var missing")).unwrap();
    let hive_name = std::env::var("HIVE_NAME").expect("HIVE_NAME env var missing");

    let secp = Secp256k1::new();

    let trx = UnsignedTransaction::new(
        &props,
        vec![Operation::AccountCreate(AccountCreate {
            creator: hive_name.to_string(),
            owner: key_to_authority_type(create_key(&secp)),
            active: key_to_authority_type(create_key(&secp)),
            posting: key_to_authority_type(create_key(&secp)),
            memo_key: create_key(&secp).create_public(None),
            new_account_name: "orillion50".to_string(),
            fee: Asset::new(100, AssetSymbol::HIVE),
            json_metadata: "{}".to_string(),
        })],
    )
    .unwrap();

    let signed = trx.sign(&private_key, None);

    match broadcast_transaction(&client, &vec![signed]).await {
        Ok(res) => println!("{:?}", res),
        Err(e) => eprintln!("{:?}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
            //submit_custom_json(&client, &props).await.unwrap();
            submit_account_create(&client, &props).await.unwrap();
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}
