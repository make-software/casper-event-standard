# Casper Event Standard

The smart contract level events for Casper.

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
