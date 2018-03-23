extern crate exonum;
extern crate exonum_cryptocurrency as cryptocurrency;
extern crate exonum_time;
extern crate iron;
extern crate router;

use exonum::blockchain::{Service, GenesisConfig, ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::{MemoryDB};
use cryptocurrency::CurrencyService;
use exonum_time::{TimeService};
use exonum::helpers::fabric::{self, Context, NodeBuilder};

mod network;

use network::NameServiceFactory;

fn main() {
    exonum::helpers::init_logger().unwrap();
    NodeBuilder::new()
        .with_service(Box::new(ServiceFactory))
        .with_service(Box::new(NameServiceFactory))
        .run();
}

pub struct ServiceFactory;

impl fabric::ServiceFactory for ServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(CurrencyService)
    }
}