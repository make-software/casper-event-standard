#![no_std]

//! The smart contract level events for Casper.
//!
//! ```rust
//! use casper_event_standard::Event;
//!
//! // Turn a struct into an event.
//! #[derive(Event)]
//! struct Transfer {
//!    amount: U256,
//!    from: Key,
//!    to: Key
//! }
//!
//! // Register event schemas.
//! fn init_events() {
//!     let schemas = Schemas::new()
//!         .with::<Transfer>();
//!     casper_event_standard::init(schemas);
//! }
//!
//! // Emit event.
//! fn emit_transfer(transfer: Transfer) {
//!     casper_event_standard::emit(transfer);
//! }
//! ```

#[doc(hidden)]
pub extern crate alloc;

#[doc(hidden)]
pub use casper_types;

use alloc::string::String;
use casper_types::bytesrepr;

/// Macro that derives [`CLTyped`], [`FromBytes`], [`ToBytes`] and [`EventInstance`].
///
/// [`CLTyped`]: casper_types::CLTyped
/// [`FromBytes`]: casper_types::bytesrepr::FromBytes
/// [`ToBytes`]: casper_types::bytesrepr::ToBytes
pub use casper_event_standard_macro::Event;

mod cl_type2;
mod schema;

pub use cl_type2::CLType2;
pub use schema::{Schema, Schemas};

#[cfg(target_arch = "wasm32")]
mod contract;

#[cfg(target_arch = "wasm32")]
pub use contract::{emit, init};

#[cfg(not(target_arch = "wasm32"))]
pub fn init(_schemas: Schemas) {
    panic!("Init can be used only in wasm32.")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn emit<T>(_event: T) {
    panic!("Emit can be used only in wasm32.")
}

/// The key under which the events are stored.
pub const EVENTS_DICT: &str = "__events";
/// The key under which the events length is stored.
pub const EVENTS_LENGTH: &str = "__events_length";
/// The key under which the event schemas are stored.
pub const EVENTS_SCHEMA: &str = "__events_schema";

/// Helper trait, used for the schema generation.
pub trait EventInstance {
    /// Returns the name of the event.
    fn name() -> String;
    /// Returns the [`Schema`](schema::Schema) of the event.
    fn schema() -> schema::Schema;
}

/// Extracts full name of the event including `event_` prefix.
pub fn try_full_name_from_bytes(bytes: &[u8]) -> Result<String, bytesrepr::Error> {
    let (name, _) = bytesrepr::FromBytes::from_bytes(bytes)?;
    Ok(name)
}
