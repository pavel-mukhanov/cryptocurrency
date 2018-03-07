extern crate exonum;
extern crate exonum_cryptocurrency as cryptocurrency;
extern crate exonum_time;
extern crate timestamp_service;

use std::sync::Arc;
use exonum::blockchain::{ApiContext, ExecutionResult, GenesisConfig, Service, Transaction,
                         ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::messages::Message;
use exonum::storage::{MapIndex, MemoryDB, Snapshot};
use exonum::crypto::PublicKey;
use exonum::helpers::fabric::NodeBuilder;
use cryptocurrency::{CurrencyService, CurrencyServiceFactory};
use exonum_time::{TimeService, TimeServiceFactory};
use timestamp_service::TimestampService;

fn node_config() -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = "0.0.0.0:8000".parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        ..Default::default()
    };

    let peer_address = "0.0.0.0:2000".parse().unwrap();

    NodeConfig {
        listen_address: peer_address,
        peers: vec![],
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: None,
        network: Default::default(),
        whitelist: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
    }
}

fn main() {
    exonum::helpers::init_logger().unwrap();
    let node = Node::new(
        MemoryDB::new(),
        vec![
            Box::new(CurrencyService),
            Box::new(TimeService::new()),
            Box::new(TimestampService),
        ],
        node_config(),
    );
    println!("Starting a single node...");
    println!("Blockchain is ready for transactions!");
    node.run().unwrap();
}
