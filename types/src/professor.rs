/// Identifier type to uniquely represent a professor
pub type ProfessorId = crate::primitives::UniqId;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

use crate::primitives::StdIpfsLink;

/// Required paramater to register this professor
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewProfessorParam {
	pub info: StdIpfsLink,
}

/// Information of a professor
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct ProfessorInfo<AccountId> {
	pub professor: AccountId,
	pub info: StdIpfsLink,
}
