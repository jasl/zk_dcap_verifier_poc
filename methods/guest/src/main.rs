#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

extern crate alloc;
extern crate core;

use scale_codec::Decode;
use primitive_io::{Inputs, Outputs};

mod error;
use error::Error;
mod dcap;

fn main() {
    let input: Inputs = env::read();

    let now = input.now;
    let raw_quote = input.quote;
    let raw_quote_collateral = input.quote_collateral;

    let quote_collateral =
        dcap::SgxV30QuoteCollateral::decode(&mut raw_quote_collateral.as_slice()).unwrap();
    let (report_data, mr_enclave, mr_signer, isv_prod_id, isv_svn, tcb_status, advisory_ids) =
        dcap::verify(&raw_quote, &quote_collateral, now).unwrap();

    let output = Outputs {
        report_data,
        mr_enclave,
        mr_signer,
        isv_prod_id,
        isv_svn,
        tcb_status,
        advisory_ids
    };

    // write public output to the journal
    env::commit(&output);
}
