use alvearium_core::block_api::get_block_range;
use jsonrpsee::http_client::HttpClientBuilder;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = tracing_subscriber::EnvFilter::default().add_directive(
        "jsonrpsee[method_call{name = \"block_api.get_block_range\"}]=trace".parse()?,
    );
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(filter)
        .finish()
        .try_init()?;

    match HttpClientBuilder::default().build("https://api.hive.blog:443") {
        Ok(client) => match get_block_range(&client, 68_700_000..68_700_020).await {
            Ok(block_range) => println!("{:?}", block_range),
            Err(e) => eprintln!("{:?}", e),
        },
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

    Ok(())
}
