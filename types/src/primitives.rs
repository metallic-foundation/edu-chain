use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	traits::{ConstU32, Get},
	BoundedVec,
};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

const IPFS_LINK_LENGTH: u32 = 300;

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
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
pub struct UniqId {}

/// Structure to represent the IPFS link
#[derive(Decode, Encode, TypeInfo, Clone, MaxEncodedLen, Debug)]
pub struct IpfsLink<S: Debug + Get<u32>>(BoundedVec<u8, S>);

impl<S: Get<u32> + Debug> Eq for IpfsLink<S> {}
impl<S> PartialEq for IpfsLink<S>
where
	S: Get<u32> + Debug,
{
	fn eq(&self, other: &Self) -> bool {
		if self.0.len() == other.0.len() {
			for at in 0_usize..S::get().try_into().expect("usize is smaller than u32") {
				if self.0[at] != other.0[at] {
					return false
				}
			}
			true
		} else {
			false
		}
	}
}

#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
pub struct IpfsLinkLength;
impl<T: From<u32>> Get<T> for IpfsLinkLength {
	fn get() -> T {
		IPFS_LINK_LENGTH.into()
	}
}

impl<S: Debug + Get<u32>> IpfsLink<S> {
	pub fn new(bytes: BoundedVec<u8, S>) -> Self {
		Self(bytes)
	}

	pub fn get(&self) -> &BoundedVec<u8, S> {
		&self.0
	}

	pub fn inner(self) -> BoundedVec<u8, S> {
		self.0
	}

	pub fn get_mut(&mut self) -> &mut BoundedVec<u8, S> {
		&mut self.0
	}

	pub fn set(&mut self, bytes: BoundedVec<u8, S>) {
		self.0 = bytes
	}
}

// common types alias
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type StdIpfsLink = IpfsLink<IpfsLinkLength>; // reasonable bounded vec to cover up a ipfs link
