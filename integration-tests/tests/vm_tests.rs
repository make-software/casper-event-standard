use casper_engine_test_support::{
    utils::create_run_genesis_request, ExecuteRequestBuilder, LmdbWasmTestBuilder,
    DEFAULT_ACCOUNT_INITIAL_BALANCE,
};
use casper_event_standard::{
    Schemas, CES_VERSION, CES_VERSION_KEY, EVENTS_DICT, EVENTS_LENGTH, EVENTS_SCHEMA,
};
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    contracts::NamedKeys,
    GenesisAccount, Key, Motes, PublicKey, RuntimeArgs, SecretKey, StoredValue, URef, U512,
};
use integration_tests::{Mint, Transfer};

struct TestEnv {
    builder: LmdbWasmTestBuilder,
    account_addr: AccountHash,
}

impl TestEnv {
    pub fn new() -> TestEnv {
        // Create keypair.
        let secret_key = SecretKey::ed25519_from_bytes([7u8; 32]).unwrap();
        let public_key = PublicKey::from(&secret_key);

        // Create an AccountHash from a public key.
        let account_addr = AccountHash::from(&public_key);
        // Create a GenesisAccount.
        let account = GenesisAccount::account(
            public_key,
            Motes::new(U512::from(DEFAULT_ACCOUNT_INITIAL_BALANCE)),
            None,
        );

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(create_run_genesis_request(vec![account]));

        TestEnv {
            builder,
            account_addr,
        }
    }

    pub fn default_account(&self) -> AccountHash {
        self.account_addr
    }

    pub fn deploy_event_producer_wasm(&mut self) {
        self.deploy_wasm("event_producer.wasm")
    }

    pub fn deploy_event_initializer_wasm(&mut self) {
        self.deploy_wasm("event_initializer.wasm")
    }

    pub fn named_keys(&self) -> NamedKeys {
        self.builder
            .get_entity_with_named_keys_by_account_hash(self.default_account())
            .unwrap()
            .named_keys()
            .clone()
    }

    pub fn schemas(&self) -> Schemas {
        let key = Key::from(self.default_account());
        self.builder
            .query(None, key, &[String::from(EVENTS_SCHEMA)])
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }

    pub fn events_length(&self) -> u32 {
        let key = Key::from(self.default_account());
        self.builder
            .query(None, key, &[String::from(EVENTS_LENGTH)])
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }

    pub fn ces_version(&self) -> String {
        let key = Key::from(self.default_account());
        self.builder
            .query(None, key, &[String::from(CES_VERSION_KEY)])
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
    }

    pub fn event_at<T: FromBytes>(&self, index: u32) -> Option<T> {
        let dictionary_seed_uref: URef = *self
            .named_keys()
            .get(EVENTS_DICT)
            .unwrap()
            .as_uref()
            .unwrap();

        let event: StoredValue = self
            .builder
            .query_dictionary_item(None, dictionary_seed_uref, &index.to_string())
            .unwrap();

        let bytes: Bytes = event.as_cl_value().unwrap().clone().into_t().unwrap();
        let (event, bytes) = T::from_bytes(&bytes).unwrap();
        assert!(bytes.is_empty());
        Some(event)
    }

    fn deploy_wasm(&mut self, name: &str) {
        let wasm_exec_request =
            ExecuteRequestBuilder::standard(self.default_account(), name, RuntimeArgs::new())
                .build();

        self.builder
            .exec(wasm_exec_request)
            .expect_success()
            .commit();
    }
}

#[test]
fn test_events_initalization() {
    let mut test_env = TestEnv::new();
    test_env.deploy_event_initializer_wasm();

    let named_keys = test_env.named_keys();
    assert!(named_keys.contains(EVENTS_DICT));
    assert!(named_keys.contains(EVENTS_LENGTH));
    assert!(named_keys.contains(EVENTS_SCHEMA));
    assert!(named_keys.contains(CES_VERSION_KEY));
    assert_eq!(test_env.events_length(), 0);
    assert_eq!(test_env.ces_version(), CES_VERSION);

    let schemas = test_env.schemas();
    let mut expected_schemas = Schemas::new();
    expected_schemas.add::<Transfer>();
    expected_schemas.add::<Mint>();
    assert_eq!(schemas, expected_schemas);
}

#[test]
fn test_events_emission() {
    let mut test_env = TestEnv::new();
    test_env.deploy_event_initializer_wasm();
    test_env.deploy_event_producer_wasm();

    assert_eq!(test_env.events_length(), 4);

    let transfer_1: Transfer = test_env.event_at(0).unwrap();
    assert_eq!(transfer_1, integration_tests::mock_transfer_1());

    let transfer_2: Transfer = test_env.event_at(1).unwrap();
    assert_eq!(transfer_2, integration_tests::mock_transfer_2());

    let mint_1: Mint = test_env.event_at(2).unwrap();
    assert_eq!(mint_1, integration_tests::mock_mint_1());

    let mint_2: Mint = test_env.event_at(3).unwrap();
    assert_eq!(mint_2, integration_tests::mock_mint_2());
}
