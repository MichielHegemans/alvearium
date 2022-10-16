use alvearium::condenser_api::get_version;
use jsonrpsee::http_client::HttpClientBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HttpClientBuilder::default().build("https://api.hive.blog:443")?;

    let version = get_version(&client).await?;

    println!("Condenser Api Version: {:?}", version);

    Ok(())
}
