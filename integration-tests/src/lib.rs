#![no_std]

extern crate alloc;

use casper_event_standard::Event;
use casper_types::{Key, U256};

#[derive(Event, Debug, PartialEq)]
pub struct Transfer {
    amount: U256,
    from: Key,
    to: Key,
}

#[derive(Event, Debug, PartialEq)]
pub struct Mint {
    account: Key,
    amount: U256,
}

pub fn mock_transfer_1() -> Transfer {
    Transfer {
        amount: U256::from(123),
        from: Key::from_formatted_str(
            "hash-1111111111111111111111111111111111111111111111111111111111111111",
        )
        .unwrap(),
        to: Key::from_formatted_str(
            "hash-2222222222222222222222222222222222222222222222222222222222222222",
        )
        .unwrap(),
    }
}

pub fn mock_transfer_2() -> Transfer {
    Transfer {
        amount: U256::from(234),
        from: Key::from_formatted_str(
            "hash-3333333333333333333333333333333333333333333333333333333333333333",
        )
        .unwrap(),
        to: Key::from_formatted_str(
            "hash-4444444444444444444444444444444444444444444444444444444444444444",
        )
        .unwrap(),
    }
}

pub fn mock_mint_1() -> Mint {
    Mint {
        account: Key::from_formatted_str(
            "hash-5555555555555555555555555555555555555555555555555555555555555555",
        )
        .unwrap(),
        amount: U256::from(345),
    }
}

pub fn mock_mint_2() -> Mint {
    Mint {
        account: Key::from_formatted_str(
            "hash-6666666666666666666666666666666666666666666666666666666666666666",
        )
        .unwrap(),
        amount: U256::from(456),
    }
}
