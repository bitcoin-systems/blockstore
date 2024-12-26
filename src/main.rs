use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::{stream, StreamExt};
mod provider;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);
    let block = provider.get_block_number().await;

    match block {
        Ok(val) => {
            println!("block: {val}");
        }
        Err(e) => {
            println!("{e}");
            println!("error");
        }
    }

    // Create a provider.
    let ws = WsConnect::new("wss://bsc-rpc.publicnode.com");
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to blocks.
    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream().take(2);

    while let Some(header) = stream.next().await {
        println!("Received block number: {}", header.number);
    }

    // Poll for block headers.
    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    while let Some(block_hash) = stream.next().await {
        println!("Polled for block header: {block_hash:?}");
    }

    Ok(())
}
