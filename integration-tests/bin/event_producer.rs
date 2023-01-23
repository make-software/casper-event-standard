#![no_std]
#![no_main]

#[no_mangle]
fn call() {
    casper_event_standard::emit(integration_tests::mock_transfer_1());
    casper_event_standard::emit(integration_tests::mock_transfer_2());
    casper_event_standard::emit(integration_tests::mock_mint_1());
    casper_event_standard::emit(integration_tests::mock_mint_2());
}
