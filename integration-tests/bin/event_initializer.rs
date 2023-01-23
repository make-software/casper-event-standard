#![no_std]
#![no_main]

use casper_event_standard::Schemas;
use integration_tests::{Mint, Transfer};

#[no_mangle]
fn call() {
    let mut schemas = Schemas::new();
    schemas.add::<Transfer>();
    schemas.add::<Mint>();
    casper_event_standard::init(schemas);
}
