#![no_std]
#![no_main]

use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::{Bytes, ToBytes};

#[no_mangle]
fn call() {
    casper_event_standard::emit(integration_tests::mock_transfer_1());
    casper_event_standard::emit(integration_tests::mock_transfer_2());
    casper_event_standard::emit(integration_tests::mock_mint_1());

    let event = integration_tests::mock_mint_2();
    let event_bytes: Bytes = event.to_bytes().unwrap_or_revert().into();
    casper_event_standard::emit_bytes(event_bytes);
}
