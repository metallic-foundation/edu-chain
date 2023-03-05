use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum Scaler<Bound> {
	Limited(Bound),
	Unlimited,
}

impl<Bound> Scaler<Bound> {
	pub fn is_limited(&self) -> bool {
		matches!(self, Scaler::Limited(..))
	}

	pub fn is_unlimited(&self) -> bool {
		matches!(self, Scaler::Unlimited)
	}

	pub fn limit(&self) -> Option<&Bound> {
		match self {
			Scaler::Limited(bound) => Some(bound),
			Scaler::Unlimited => None,
		}
	}
}

/// type to uniquely represent any item
/// this better to be a had bytes
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone)]
pub struct UniqId {}

/// Structure to represent the IPFS link
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone)]
pub struct IpfsLink {}

// common types alias
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
