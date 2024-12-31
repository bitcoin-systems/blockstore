#[derive(Debug, Clone, Copy)]
pub struct Chain {
    pub id: i64,
    pub name: &'static str,
    pub rpc: &'static str,
}

impl Chain {
    pub fn new(chain: &Chain) -> Self {
        Self {
            id: chain.id,
            name: chain.name,
            rpc: chain.rpc,
        }
    }
}

pub const ETHEREUM: Chain = Chain {
    id: 1,
    name: "ethereum",
    rpc: "https://eth.llamarpc.com",
};

pub const POLYGON: Chain = Chain {
    id: 137,
    name: "polygon",
    rpc: "https://polygon.llamarpc.com",
};

pub const FANTOM: Chain = Chain {
    id: 250,
    name: "fantom",
    rpc: "https://rpc.ftm.tools",
};

pub const BSC: Chain = Chain {
    id: 56,
    name: "bsc",
    rpc: "https://bscrpc.com",
};

pub const GNOSIS: Chain = Chain {
    id: 100,
    name: "gnosis",
    rpc: "https://rpc.ankr.com/gnosis",
};

pub const OPTIMISM: Chain = Chain {
    id: 10,
    name: "optimism",
    rpc: "https://rpc.ankr.com/optimism",
};

pub const ARBITRUM_ONE: Chain = Chain {
    id: 42161,
    name: "arbitrum",
    rpc: "https://rpc.ankr.com/arbitrum",
};

pub const ARBITRUM_NOVA: Chain = Chain {
    id: 42170,
    name: "arbitrum-nova",
    rpc: "https://nova.arbitrum.io/rpc",
};

pub const MOONBEAM: Chain = Chain {
    id: 1284,
    name: "moonbeam",
    rpc: "https://rpc.ankr.com/moonbeam",
};

pub const AVALANCHE: Chain = Chain {
    id: 43114,
    name: "avalanche",
    rpc: "https://rpc.ankr.com/avalanche",
};

pub const BITTORRENT: Chain = Chain {
    id: 199,
    name: "bittorrent",
    rpc: "https://rpc.bittorrentchain.io",
};

pub const CELO: Chain = Chain {
    id: 42220,
    name: "celo",
    rpc: "https://rpc.ankr.com/celo",
};

pub static CHAINS: [Chain; 12] = [
    ETHEREUM,
    POLYGON,
    FANTOM,
    BSC,
    GNOSIS,
    OPTIMISM,
    ARBITRUM_ONE,
    ARBITRUM_NOVA,
    MOONBEAM,
    AVALANCHE,
    BITTORRENT,
    CELO,
];
