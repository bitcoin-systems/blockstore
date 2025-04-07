mod chains;
mod provider;
use crate::chains::chains::CHAINS;

fn main() {
    // let mut threads = vec![];
    for chain in CHAINS.into_iter() {
        println!("{:?}", chain);
    }
}
