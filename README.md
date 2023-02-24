# Casper Event Standard

The Casper Event Standard is a Rust library that provides a simple and standardized way for smart contracts on the Casper Network to emit events. Events are an important tool for decentralized applications, as they allow contracts to communicate with external services in a transparent and decentralized way.

With the Casper Event Standard, developers can easily define custom event types and register them in the contract. The library provides a convenient API for emitting events. This makes it easy to build complex applications that rely on event-driven architecture.

Whether you're building a decentralized exchange, a prediction market, or any other type of blockchain application, the Casper Event Standard is an essential tool for creating reliable and scalable smart contracts. So why wait? Start using the Casper Event Standard today and take your decentralized application to the next level!

## Usage

```rust
use casper_event_standard::Event;

// Turn a struct into an event.
#[derive(Event)]
struct Transfer {
    amount: U256,
    from: Key,
    to: Key
}

// Register event schemas.
fn init_events() {
    let schemas = Schemas::new()
        .with::<Transfer>();
    casper_event_standard::init(schemas);
}

// Emit event.
fn emit_transfer(transfer: Transfer) {
    casper_event_standard::emit(transfer);
}
```

## Tests

To test the code run:

```bash
$ just test
```
