use crate::{schema::Schemas, EVENTS_DICT, EVENTS_LENGTH, EVENTS_SCHEMA};
use alloc::string::ToString;
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    bytesrepr::{Bytes, ToBytes},
    ApiError, URef,
};

/// Initializes events-releated named keys and stores [`Schemas`].
///
/// It should be called during the contract initialization.
///
/// [`Schemas`]: crate::Schema
pub fn init(schemas: Schemas) {
    expect_no_key(EVENTS_LENGTH);
    expect_no_key(EVENTS_SCHEMA);
    storage::new_dictionary(EVENTS_DICT).unwrap_or_revert();
    runtime::put_key(EVENTS_LENGTH, storage::new_uref(0u32).into());
    runtime::put_key(EVENTS_SCHEMA, storage::new_uref(schemas).into());
}

/// Emits an event.
pub fn emit<T: ToBytes>(event: T) {
    let length_key = runtime::get_key(EVENTS_LENGTH).unwrap_or_revert();
    let length_uref = length_key.try_into().unwrap_or_revert();
    let lenght: u32 = storage::read(length_uref)
        .unwrap_or_revert()
        .unwrap_or_revert();
    let seed = event_dict_seed();
    let event_bytes = event.to_bytes().unwrap_or_revert();
    let event_bytes: Bytes = event_bytes.into();
    storage::dictionary_put(seed, &lenght.to_string(), event_bytes);
    storage::write(length_uref, lenght + 1);
}

fn event_dict_seed() -> URef {
    runtime::get_key(EVENTS_DICT)
        .unwrap_or_revert()
        .try_into()
        .unwrap_or_revert()
}

fn expect_no_key(name: &str) {
    if runtime::has_key(name) {
        runtime::revert(ApiError::InvalidArgument);
    }
}
