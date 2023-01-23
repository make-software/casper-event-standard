#![no_std]
pub extern crate alloc;

use alloc::string::String;
pub use casper_event_standard_macro::Event;
pub use casper_types;
use casper_types::bytesrepr;

mod cl_type2;
mod schema;

pub use cl_type2::CLType2;
pub use schema::{Schema, Schemas};

#[cfg(target_arch = "wasm32")]
mod contract;

#[cfg(target_arch = "wasm32")]
pub use contract::{emit, init};

/// The key under which the events are stored.
pub const EVENTS_DICT: &str = "__events";
/// The key under which the events length is stored.
pub const EVENTS_LENGTH: &str = "__events_length";
pub const EVENTS_SCHEMA: &str = "__events_schema";

pub trait EventInstance {
    fn name() -> String;
    fn schema() -> schema::Schema;
}

pub fn try_full_name_from_bytes(bytes: &[u8]) -> Result<String, bytesrepr::Error> {
    let (name, _) = bytesrepr::FromBytes::from_bytes(bytes)?;
    Ok(name)
}
