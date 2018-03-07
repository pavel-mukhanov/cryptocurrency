// Import crates with necessary types into a new project.

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum;
extern crate router;
extern crate bodyparser;
extern crate iron;
extern crate exonum_time;

// Import necessary types from crates.

use exonum::blockchain::{Blockchain, Service, Transaction, ApiContext, ExecutionResult,
                         TransactionSet};
use exonum::encoding::serialize::FromHex;
use exonum::node::{TransactionSend, ApiSender};
use exonum::messages::{RawTransaction};
use exonum::storage::{Fork, MapIndex, Snapshot};
use exonum::crypto::*;
use exonum::encoding;
use exonum::api::{Api, ApiError};
use iron::prelude::*;
use iron::Handler;
use iron::status::Status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;
use std::time::{UNIX_EPOCH};

// // // // // // // // // // CONSTANTS // // // // // // // // // //

/// Service ID for the `Service` trait.
const SERVICE_ID: u16 = 2;

// // // // // // // // // // PERSISTENT DATA // // // // // // // // // //

encoding_struct! {

    struct Timestamp {

        file_hash: &Hash,

        time: u64,
    }
}

// // // // // // // // // // DATA LAYOUT // // // // // // // // // //

pub struct TimestampSchema<T> {
    view: T,
}

impl<T: AsRef<Snapshot>> TimestampSchema<T> {
    /// Creates a new schema instance.
    pub fn new(view: T) -> Self {
        TimestampSchema { view }
    }

    /// Returns an immutable version of the timestamps table.
    pub fn timestamps(&self) -> MapIndex<&Snapshot, Hash, Timestamp> {
        MapIndex::new("timestamps", self.view.as_ref())
    }

    /// Gets a specific timestamp from the storage.
    pub fn timestamp(&self, pub_key: &Hash) -> Option<Timestamp> {
        self.timestamps().get(pub_key)
    }
}

/// A mutable version of the schema with an additional method to persist timestamps
/// to the storage.
impl<'a> TimestampSchema<&'a mut Fork> {
    /// Returns a mutable version of the timestamps table.
    pub fn timestamps_mut(&mut self) -> MapIndex<&mut Fork, Hash, Timestamp> {
        MapIndex::new("timestamps", &mut self.view)
    }
}

// // // // // // // // // // TRANSACTIONS // // // // // // // // // //

transactions! {
    TimestampTransactions {
        const SERVICE_ID = SERVICE_ID;

        struct TxCreateTimestamp {
            data: &str,
        }
    }
}

// // // // // // // // // // CONTRACTS // // // // // // // // // //

impl Transaction for TxCreateTimestamp {
    /// Verifies integrity of the transaction by checking the transaction
    /// signature.
    fn verify(&self) -> bool {
        //self.verify_signature(self.pub_key())
        true
    }

    /// If a wallet with the specified public key is not registered, then creates a new wallet
    /// with the specified public key and name, and an initial balance of 100.
    /// Otherwise, performs no op.
    fn execute(&self, view: &mut Fork) -> ExecutionResult {


        let time = {
            let time_schema = exonum_time::TimeSchema::new(view.as_ref());
            time_schema.time().get()
        };

        // The time in the transaction should be less than in the blockchain.
        match time {
            Some(current_time) => {
                println!("time -- {:?}", current_time);
                let mut schema = TimestampSchema::new(view);

                let data_hash = hash(&self.data().as_bytes());


                if schema.timestamp(&data_hash).is_none() {

//                    let start = SystemTime::now();
                    let since_the_epoch = current_time.duration_since(UNIX_EPOCH).unwrap();
                    let in_ms = since_the_epoch.as_secs() * 1000 +
                        since_the_epoch.subsec_nanos() as u64 / 1_000_000;



                   let timestamp = Timestamp::new(&data_hash, in_ms);
//
//                    println!("Create timestamp: {:?}", timestamp);
                    schema.timestamps_mut().put(&data_hash, timestamp);
                }
                // Execute transaction business logic.
            }
            _ => {}
        }

        Ok(())
    }
}

// // // // // // // // // // REST API // // // // // // // // // //

/// Container for the service API.
#[derive(Clone)]
struct TimestampApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    /// Hash of the transaction.
    pub tx_hash: Hash,
}

impl TimestampApi {
    fn get_timestamp(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let timestamp_key = path.last().unwrap();
        let data_hash = Hash::from_hex(timestamp_key).map_err(|e| {
            IronError::new(e, (
                Status::BadRequest,
                Header(ContentType::json()),
                "\"Invalid request param: `data_hash`\"",
            ))
        })?;

        let get_timestamp = {
            let snapshot = self.blockchain.snapshot();
            let schema = TimestampSchema::new(snapshot);
            schema.timestamp(&data_hash)
        };

        if let Some(timestamp) = get_timestamp {
            self.ok_response(&serde_json::to_value(timestamp).unwrap())
        } else {
            self.not_found_response(
                &serde_json::to_value("Timestamp not found").unwrap(),
            )
        }
    }

    fn get_all_timestamps(&self, _: &mut Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = TimestampSchema::new(snapshot);
        let idx = schema.timestamps();
        let timestamps: Vec<Timestamp> = idx.values().collect();
        self.ok_response(&serde_json::to_value(&timestamps).unwrap())
    }

    fn post_transaction<T>(&self, req: &mut Request) -> IronResult<Response>{
        match req.get::<bodyparser::Struct<TimestampTransactions>>() {
            Ok(Some(transaction)) => {
                let transaction: Box<Transaction> = transaction.into();
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;
                let json = TransactionResponse { tx_hash };
                self.ok_response(&serde_json::to_value(&json).unwrap())
            }
            Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        }
    }
}

/// `Api` trait implementation.
impl Api for TimestampApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let post_create_timestamp = move |req: &mut Request| {
            self_.post_transaction::<TxCreateTimestamp>(req)
        };

        let self_ = self.clone();
        let get_all_timestamps = move |req: &mut Request| self_.get_all_timestamps(req);
        let self_ = self.clone();
        let get_timestamp = move |req: &mut Request| self_.get_timestamp(req);

        // Bind handlers to specific routes.
        router.post("/v1/timestamp", post_create_timestamp, "post_create_timestamp");
        router.get("/v1/timestamp/all", get_all_timestamps, "get_all_timestamps");
        router.get("/v1/timestamp/:data_hash", get_timestamp, "get_timestamp");
    }
}

// // // // // // // // // // SERVICE DECLARATION // // // // // // // // // //

/// Timestamp service.
pub struct TimestampService;

impl Service for TimestampService {
    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn service_name(&self) -> &'static str {
        "timestamp_service"
    }

    // Hashes for the service tables that will be included into the state hash.
    // To simplify things, we don't have [Merkelized tables][merkle] in the service storage
    // for now, so we return an empty vector.
    //
    // [merkle]: https://exonum.com/doc/architecture/storage/#merklized-indices
    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    // Implement a method to deserialize transactions coming to the node.
    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let tx = TimestampTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    // Create a REST `Handler` to process web requests to the node.
    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = TimestampApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }
}