extern crate exonum;
extern crate exonum_cryptocurrency as cryptocurrency;
#[macro_use]
extern crate exonum_testkit;
extern crate futures;
extern crate rand;
extern crate tempdir;
extern crate vec_map;

use exonum::blockchain::{Blockchain, Transaction};
use exonum::messages::{MessageBuffer, RawMessage};
use exonum::crypto::{self, gen_keypair, PublicKey, SecretKey};
use exonum_testkit::{TestKit, TestKitBuilder};
use futures::sync::mpsc;
use std::path::Path;
use tempdir::TempDir;
use exonum::storage::{Database, MemoryDB};
use exonum::node::ApiSender;
use rand::thread_rng;
use vec_map::VecMap;
use exonum::api::ApiError;
use exonum::encoding::{Error as MessageError};
use std::error::Error;

// Import data types used in tests from the crate where the service is defined.
use cryptocurrency::{CurrencySchema, CurrencyService, TxCreateWallet, TxTransfer, Wallet};

// Imports shared test constants.
use constants::{ALICE_NAME, BOB_NAME};

mod constants;

/// Initializes testkit with `CurrencyService`.
fn init_testkit() -> TestKit {
    TestKitBuilder::validator()
        .with_service(CurrencyService)
        .create()
}

pub fn tx_info() -> Result<(), ApiError> {
    let err = 
        MessageError::IncorrectServiceId { service_id: 1 }.description();

    Err(ApiError::InternalError(
        encoding_error().unwrap_err().description().into()
    ))
}

pub fn encoding_error() -> Result<(), MessageError> {
    Err(MessageError::IncorrectServiceId { service_id: 1 })
}

#[test]
fn test_error() {
    let teskit = init_testkit();

    let dir = TempDir::new(gen_tempdir_name().as_str()).unwrap();
    let path = dir.path();
    let blockchain = create_blockchain(path);

    let buf = vec![1; 10];
    let raw = RawMessage::new(MessageBuffer::from_vec(buf));

    println!("raw message {:?}", raw);
    
    let res = blockchain.tx_from_raw(raw);


    assert_eq!("Incorrect service id", res.unwrap_err().description());

    assert!(false);
}

#[test]
fn test_tx_info() {
    println!("tx info {:?}", tx_info());
    assert!(false);
}

fn create_blockchain(_: &Path) -> Blockchain {
    let service_keypair = gen_keypair();
    let api_channel = mpsc::channel(1);
    Blockchain::new(
        MemoryDB::new(),
        vec![Box::new(CurrencyService)],
        service_keypair.0,
        service_keypair.1,
        ApiSender::new(api_channel.0),
    )
}

fn gen_tempdir_name() -> String {
    String::from("temp")
}
