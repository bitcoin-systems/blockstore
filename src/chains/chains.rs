#[derive(Debug, Clone, Copy)]
pub struct Chain {
  pub id: i64,
  pub name: &'static str,
  pub rpc: &'static str
}

impl Chain {
  pub fn new(chain: &Chain) -> Self {
    Self {
      id: chain.id,
      name: chain.name,
      rpc: chain.rpc
    }
  }
}