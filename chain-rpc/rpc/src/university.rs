use std::sync::Arc;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_api::{BlockId, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use types::university::*;

pub use chain_rpc_runtime_api::university::PalletUniversityApi as UniversityRuntimeApi;

#[rpc(client, server)]
pub trait UniversityApi<BlockHash> {
	#[method(name = "university_get_all_id")]
	fn all_university_id(&self, at: Option<BlockHash>) -> RpcResult<Vec<UniversityId>>;
}

pub struct University<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> University<C, P> {
	/// Creates a new instance of the TransactionPayment Rpc helper.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block> UniversityApiServer<<Block as BlockT>::Hash> for University<C, Block>
where
	Block: BlockT,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: UniversityRuntimeApi<Block>,
{
	fn all_university_id(&self, at: Option<Block::Hash>) -> RpcResult<Vec<UniversityId>> {
		let block_id = BlockId::Hash(at.unwrap_or_else(|| self.client.info().best_hash));
		let api = self.client.runtime_api();
		let all_id = api.universities_keys(&block_id).unwrap();

		Ok(all_id)
	}
}
