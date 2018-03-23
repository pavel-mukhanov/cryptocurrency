
use exonum::blockchain::{ApiContext, Blockchain, ExecutionError, ExecutionResult, Service,
                         Transaction, TransactionSet};
use exonum::encoding::serialize::FromHex;
use exonum::node::{ApiSender, TransactionSend};
use exonum::messages::{Message, RawTransaction};
use exonum::storage::{Fork, MapIndex, Snapshot};
use exonum::helpers::fabric::{self, Context, NodeBuilder};
use exonum::crypto::{Hash, PublicKey};
use exonum::encoding;
use exonum::api::{Api, ApiError};
use iron::prelude::*;
use iron::Handler;
use iron::status::{self, Status};
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;

const SERVICE_ID: u16 = 2;

pub struct NameService;

impl Service for NameService {
    fn service_name(&self) -> &'static str {
        "name"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    // Implement a method to deserialize transactions coming to the node.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        Err(encoding::Error::from("uniplemented"))
    }

    // Hashes for the service tables that will be included into the state hash.
    // To simplify things, we don't have [Merkelized tables][merkle] in the service storage
    // for now, so we return an empty vector.
    //
    // [merkle]: https://exonum.com/doc/architecture/storage/#merklized-indices
    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    // Create a REST `Handler` to process web requests to the node.
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = NetworkApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }
}

pub struct NameServiceFactory;

impl fabric::ServiceFactory for NameServiceFactory {
    fn make_service(&mut self, _: &Context) -> Box<Service> {
        Box::new(NameService)
    }
}

#[derive(Clone)]
struct NetworkApi {
    blockchain:Blockchain,
    channel:ApiSender
}

impl NetworkApi {
    // add code here
    fn get_name(&self, req: &mut Request) -> IronResult<Response> {
        let response = format!("{:?}", self.blockchain);
        Ok(Response::with((status::Ok, response)))    
    }
}

impl Api for NetworkApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let get_name = move |req: &mut Request| self_.get_name(req);
       
        router.get("/v1/get_name", get_name,  "get_name");
    }
}