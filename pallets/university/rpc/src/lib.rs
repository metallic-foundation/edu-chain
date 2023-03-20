use std::{convert::TryInto, sync::Arc};

use codec::{Codec, Decode};
use jsonrpsee::{
	core::{Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorCode, ErrorObject},
};
use sp_api::{ApiExt, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};

pub use pallet_university_rpc_runtime_api::PalletUniversityApi as UniversityRuntimeApi;

#[rpc(client, server)]
pub trait UniversityApi<BlockHash> {
	#[method(name = "university_is_verified")]
	fn university_is_verified(&self) -> RpcResult<()>;
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
	fn university_is_verified(&self) -> RpcResult<()> {
		let api = self.client.runtime_api();

		Ok(())
	}
}
