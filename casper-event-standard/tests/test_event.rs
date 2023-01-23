use casper_event_standard::{try_full_name_from_bytes, Event, EventInstance, Schema};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, Key, U256,
};

#[derive(Event, Debug, PartialEq)]
struct Transfer {
    amount: U256,
    from: Key,
    to: Key,
}

fn mock_transfer() -> Transfer {
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

#[test]
fn test_event_serialization() {
    let expected = mock_transfer();
    let bytes = expected.to_bytes().unwrap();
    let (result, bytes) = Transfer::from_bytes(&bytes).unwrap();
    assert!(bytes.is_empty());
    assert_eq!(result, expected);
}

#[test]
fn test_event_name() {
    let transfer = mock_transfer();
    assert_eq!(Transfer::name(), "Transfer");
    let bytes = transfer.to_bytes().unwrap();
    let full_name = try_full_name_from_bytes(&bytes).unwrap();
    assert_eq!(&full_name, "event_Transfer");
}

#[test]
fn test_event_schema() {
    let mut expected_schema = Schema::new();
    expected_schema.with_elem("amount", U256::cl_type());
    expected_schema.with_elem("from", Key::cl_type());
    expected_schema.with_elem("to", Key::cl_type());
    assert_eq!(Transfer::schema(), expected_schema);
}
