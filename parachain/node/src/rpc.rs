//! RPC interface for the IkubChain node.

use std::sync::Arc;

use jsonrpc_core::{Error as RpcError, ErrorCode, Result as JsonResult};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use ikub_chain_runtime::{
    opaque::Block,
    AccountId,
    Balance,
    Index,
};

/// RPC interface for the IkubChain node.
#[rpc]
pub trait IkubChainApi<BlockHash> {
    /// Get the current balance of an account.
    #[rpc(name = "ikubchain_getBalance")]
    fn get_balance(&self, account: AccountId, at: Option<BlockHash>) -> JsonResult<Balance>;

    /// Get the current nonce of an account.
    #[rpc(name = "ikubchain_getNonce")]
    fn get_nonce(&self, account: AccountId, at: Option<BlockHash>) -> JsonResult<Index>;
}

/// Implementation of the RPC API for IkubChain.
pub struct IkubChain<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> IkubChain<C, M> {
    /// Create a new instance of the IkubChain RPC API.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block> IkubChainApi<<Block as BlockT>::Hash> for IkubChain<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ikub_chain_runtime::RuntimeApiCollection<Block>,
{
    fn get_balance(
        &self,
        _account: AccountId,
        _at: Option<<Block as BlockT>::Hash>,
    ) -> JsonResult<Balance> {
        // Implementation would go here
        Ok(0)
    }

    fn get_nonce(
        &self,
        _account: AccountId,
        _at: Option<<Block as BlockT>::Hash>,
    ) -> JsonResult<Index> {
        // Implementation would go here
        Ok(0)
    }
}

/// Create a new RPC API instance.
pub fn create<C, Block>(
    client: Arc<C>,
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> 
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ikub_chain_runtime::RuntimeApiCollection<Block>,
{
    let mut io = jsonrpc_core::IoHandler::default();
    let rpc = IkubChain::<C, Block>::new(client);
    
    io.extend_with(rpc.to_delegate(IkubChainApi::to_delegate()));
    
    io
}
