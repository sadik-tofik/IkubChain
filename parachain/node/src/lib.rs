//! IkubChain node library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

pub mod chain_spec;
pub mod cli;
pub mod command;
pub mod rpc;
pub mod service;

/// The node version information.
pub const VERSION: sc_cli::VersionInfo = sc_cli::VersionInfo {
    name: "IkubChain Node",
    author: "IkubChain Team",
    description: "IkubChain Node Implementation",
    support_url: "https://github.com/ikubchain/ikubchain/issues",
    copyright_start_year: 2023,
    executable_name: "ikubchain",
    executable_description: "IkubChain Node",
    implementation_name: "ikubchain-node",
    spec_name: "ikubchain",
    spec_version: 1,
    impl_version: 1,
    authoring_version: 1,
    transaction_version: 1,
    state_version: 1,
};

/// The chain specification type.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

/// The extensions for the chain spec.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sc_chain_spec::ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

/// The chain specification for the local testnet.
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    chain_spec::local_testnet_config()
}

/// The chain specification for the development network.
pub fn development_config() -> Result<ChainSpec, String> {
    chain_spec::development_config()
}

/// The chain specification for the main network.
pub fn mainnet_config() -> Result<ChainSpec, String> {
    chain_spec::ikub_chain_config()
}
