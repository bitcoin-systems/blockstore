mod chains;
mod provider;
use crate::chains::chains::CHAINS;
use crate::provider::rpc::rpc;

fn main() {
    // let mut threads = vec![];
    rpc();

    for chain in CHAINS.into_iter() {
        println!("{:?}", chain);

    }
}
