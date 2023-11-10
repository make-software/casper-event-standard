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
use casper_types::{bytesrepr, CLType, CLTyped};

#[cfg(not(target_arch = "wasm32"))]
use casper_types::bytesrepr::Bytes;

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
pub use contract::{emit, emit_bytes, init};

#[cfg(not(target_arch = "wasm32"))]
pub fn init(_schemas: Schemas) {
    panic!("Init can be used only in wasm32.")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn emit<T>(_event: T) {
    panic!("Emit can be used only in wasm32.")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn emit_bytes(_event: Bytes) {
    panic!("Emit can be used only in wasm32.")
}

/// The key under which the events are stored.
pub const EVENTS_DICT: &str = "__events";
/// The key under which the events length is stored.
pub const EVENTS_LENGTH: &str = "__events_length";
/// The key under which the event schemas are stored.
pub const EVENTS_SCHEMA: &str = "__events_schema";
/// The key under which the ces version is stored.
pub const CES_VERSION_KEY: &str = "__events_ces_version";
/// The version of CES implemented in this library.
pub const CES_VERSION: &str = "1.1";

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

/// Make sure the type of a value is not [`CLType::Any`](casper_types::CLType::Any).
pub fn validate_type<T: CLTyped>(_: &T) -> Result<(), bytesrepr::Error> {
    if has_any(&T::cl_type()) {
        Err(bytesrepr::Error::Formatting)
    } else {
        Ok(())
    }
}

fn has_any(ty: &CLType) -> bool {
    match ty {
        // Positive.
        CLType::Any => true,

        // Negative.
        CLType::Bool
        | CLType::I32
        | CLType::I64
        | CLType::U8
        | CLType::U32
        | CLType::U64
        | CLType::U128
        | CLType::U256
        | CLType::U512
        | CLType::Unit
        | CLType::String
        | CLType::Key
        | CLType::URef
        | CLType::PublicKey
        | CLType::ByteArray(_) => false,

        // Need recursive check.
        CLType::Option(ty) => has_any(ty),
        CLType::List(ty) => has_any(ty),
        CLType::Result { ok, err } => has_any(ok) || has_any(err),
        CLType::Map { key, value } => has_any(key) || has_any(value),
        CLType::Tuple1([ty]) => has_any(ty),
        CLType::Tuple2([ty1, ty2]) => has_any(ty1) || has_any(ty2),
        CLType::Tuple3([ty1, ty2, ty3]) => has_any(ty1) || has_any(ty2) || has_any(ty3),
    }
}
