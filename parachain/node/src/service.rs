//! Service and ServiceFactory implementation for the IkubChain node.

use std::sync::Arc;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sp_runtime::traits::Block as BlockT;
use sc_executor::NativeElseWasmExecutor;
use sc_consensus_aura::sr25519::AuthorityPair as AuraPair;

use crate::rpc;

/// Native executor type.
pub struct Executor;

impl sc_executor::NativeExecutionDispatch for Executor {
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        ikub_chain_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        ikub_chain_runtime::native_version()
    }
}

/// Full client type.
pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

/// Type aliases for the runtime
pub type Block = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
pub type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub type BlockNumber = u32;

/// The runtime API for this runtime.
pub type RuntimeApi = ikub_chain_runtime::RuntimeApi;

/// A handle to the client instance.
pub type Client = Arc<FullClient>;

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, grandpa_link, babe_link),
    } = new_partial(&config)?;

    // Additional service setup would go here
    
    Ok(task_manager)
}

/// Builds a new partial service.
pub fn new_partial(
    config: &Configuration,
) -> Result<sc_service::PartialComponents<FullClient, FullBackend, FullSelectChain, 
    sp_consensus::DefaultImportQueue<Block, FullClient>,
    sc_transaction_pool::FullPool<Block, FullClient>,
    (
        sc_consensus_babe::BabeBlockImport<Block, FullClient, FullSelectChain>,
        sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
        sc_consensus_babe::BabeLink<Block>,
    )
>, ServiceError> {
    // Implementation would go here
    unimplemented!()
}
