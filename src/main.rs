use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::BlockTransactionsKind;
use alloy::eips::BlockNumberOrTag;
use eyre::Result;
use futures_util::{stream, StreamExt};
mod chains;
mod provider;
use crate::chains::chains::CHAINS;

#[tokio::main]
async fn main() -> Result<()> {
    let mut threads = vec![];
    for chain in CHAINS.into_iter() {
        let thread_handle = tokio::spawn(async move {
            let rpc_url = chain.rpc.parse().unwrap();
            let provider = ProviderBuilder::new().on_http(rpc_url);
            let block = provider.get_block_number().await;
        
            match block {
                Ok(val) => {
                    println!("block: {val}");
                }
                Err(e) => {
                    println!("{e}");
                }
            }

            chain.name
        });
        threads.push(thread_handle);
    }

    for t in threads {
        match t.await {
            Ok(result) => {
                println!("thread result: {:?}", result);
            }
            Err(err) => {
                println!("thread error {:?}", err);
            }
        }
    }

    // Create a provider.
    let ws = WsConnect::new("wss://bsc-rpc.publicnode.com");
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to blocks.
    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream(); //.take(2);

    while let Some(header) = stream.next().await {
        let full_block = provider.get_block_by_number(BlockNumberOrTag::Latest, BlockTransactionsKind::Full).await;
        println!("full block: {:?}", full_block);
    }

    // Poll for block headers.
    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    while let Some(block_hash) = stream.next().await {
        println!("block header: {block_hash:?}");
    }

    Ok(())
}
