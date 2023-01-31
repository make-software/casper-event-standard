default:
    just -l

prepare:
	rustup target add wasm32-unknown-unknown

test: test-lib test-macro test-integration

test-lib:
    cargo test -p casper-event-standard

test-macro:
    cargo test -p casper-event-standard-macro

test-integration: build-test-wasm copy-wasm-file test-integration-only

test-integration-only:
    cargo test -p integration-tests \
        --test vm_tests \
        --no-default-features \
        --features="test-support"

build-test-wasm:
    cargo build \
        --release \
        --target wasm32-unknown-unknown \
        -p integration-tests \
        --no-default-features \
        --features="contract-support"

copy-wasm-file:
    mkdir -p integration-tests/wasm
    cp target/wasm32-unknown-unknown/release/*.wasm integration-tests/wasm

clean:
    rm -rf integration-tests/wasm
    cargo clean

docs:
    cargo doc --no-deps --open \
        --target wasm32-unknown-unknown \
        -p casper-event-standard \
        -p casper-event-standard-macro

clippy:
    cargo clippy -p casper-event-standard \
        --target wasm32-unknown-unknown -- -D warnings
    cargo clippy -p casper-event-standard \
        --tests -- -D warnings
    cargo clippy -p integration-tests --target wasm32-unknown-unknown \
        --no-default-features --features contract-support -- -D warnings
    cargo clippy -p integration-tests --tests \
        --no-default-features --features test-support -- -D warnings

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

release:
    cargo publish -p casper-event-standard-macro && sleep 100
    cargo publish -p casper-event-standard
