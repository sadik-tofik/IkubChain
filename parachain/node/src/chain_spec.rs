//! Chain specifications for the IkubChain node.

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public, sr25519};
use sp_runtime::traits::{IdentifyAccount, Verify};

use ikub_chain_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
    Signature, SudoConfig, SystemConfig, WASM_BINARY,
};

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(
    Default, Clone, Debug, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension,
)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPair: Pair>(seed: &str) -> <TPair as Pair>::Public {
    TPair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPair: Pair>(seed: &str) -> AccountId where
    AccountPublic: From<<TPair as Pair>::Public>
{
    AccountPublic::from(get_from_seed::<TPair>(seed)).into_account()
}

/// Generate the session keys from individual elements.
pub fn get_authority_keys_from_seed(
    seed: &str,
) -> (AccountId, AccountId, GrandpaId) {
    (
        get_account_id_from_seed::<sr25519::Pair>(&format!("{}", seed)),
        get_account_id_from_seed::<sr25519::Pair>(seed),
        get_from_seed::<GrandpaPair>(seed),
    )
}

/// Development chain spec for IkubChain.
pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "IkubChain Development",
        "ikub_chain_dev",
        ChainType::Development,
        move || testnet_genesis(
            wasm_binary,
            vec![
                get_authority_keys_from_seed("Alice"),
            ],
            get_account_id_from_seed::<sr25519::Pair>("Alice"),
            vec![
                get_account_id_from_seed::<sr25519::Pair>("Alice"),
                get_account_id_from_seed::<sr25519::Pair>("Bob"),
                get_account_id_from_seed::<sr25519::Pair>("Charlie"),
                get_account_id_from_seed::<sr25519::Pair>("Dave"),
                get_account_id_from_seed::<sr25519::Pair>("Eve"),
                get_account_id_from_seed::<sr25519::Pair>("Ferdie"),
            ],
            true,
        ),
        vec![],
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 1000,
        },
    ))
}

/// Local testnet chain spec for IkubChain.
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "IkubChain Local Testnet",
        "ikub_chain_local_testnet",
        ChainType::Local,
        move || testnet_genesis(
            wasm_binary,
            vec![
                get_authority_keys_from_seed("Alice"),
                get_authority_keys_from_seed("Bob"),
            ],
            get_account_id_from_seed::<sr25519::Pair>("Alice"),
            vec![
                get_account_id_from_seed::<sr25519::Pair>("Alice"),
                get_account_id_from_seed::<sr25519::Pair>("Bob"),
                get_account_id_from_seed::<sr25519::Pair>("Charlie"),
                get_account_id_from_seed::<sr25519::Pair>("Dave"),
                get_account_id_from_seed::<sr25519::Pair>("Eve"),
                get_account_id_from_seed::<sr25519::Pair>("Ferdie"),
                get_account_id_from_seed::<sr25519::Pair>("Alice//stash"),
                get_account_id_from_seed::<sr25519::Pair>("Bob//stash"),
                get_account_id_from_seed::<sr25519::Pair>("Charlie//stash"),
                get_account_id_from_seed::<sr25519::Pair>("Dave//stash"),
                get_account_id_from_seed::<sr25519::Pair>("Eve//stash"),
                get_account_id_from_seed::<sr25519::Pair>("Ferdie//stash"),
            ],
            true,
        ),
        vec![],
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: 1000,
        },
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            _config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| x.0.clone()).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
            _config: Default::default(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        transaction_payment: Default::default(),
    }
}
