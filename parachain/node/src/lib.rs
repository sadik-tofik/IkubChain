#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;
pub mod runtime;
pub mod service;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, NumberFor},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};

sp_api::impl_runtime_apis! {
    // Implementation of the runtime API for the runtime
}

// Implement the runtime API for the node
impl sp_api::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }

    fn execute_block(block: Block) {
        Executive::execute_block(block);
    }

    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header);
    }
}

// Include the runtime version
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("ikubchain"),
    impl_name: create_runtime_str!("ikubchain-node"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

// Include the runtime
pub use runtime::*;
