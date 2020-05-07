use crate::network::NetworkConfig;
use libp2p::identity::Keypair;
use sled::{Error, Tree};
use std::path::Path;

pub struct Config {
    pub tree: Tree,
    pub network: NetworkConfig,
}

impl Config {
    pub fn new(tree: Tree, keypair: Keypair) -> Self {
        Self {
            tree,
            network: NetworkConfig::new(keypair),
        }
    }

    pub fn from_tree(tree: Tree) -> Self {
        Self::new(tree, Keypair::generate_ed25519())
    }

    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let db = sled::open(path)?;
        let tree = db.open_tree("ipfs_tree")?;
        Ok(Self::from_tree(tree))
    }
}
