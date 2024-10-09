use alloy::providers::{Provider, ProviderBuilder};
use eyre::Result;
mod provider;

#[tokio::main]
async fn main() -> Result<()> {
    // ...

    let rpc_url = "https://eth.merkle.io".parse()?;

    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().on_http(rpc_url);

    let block = provider.get_block_number().await;
    match block {
        Ok(val) => {
            println!("block: {val}");
        },
        Err(_) => {
            println!("error");
        }
    }

    Ok(())
}