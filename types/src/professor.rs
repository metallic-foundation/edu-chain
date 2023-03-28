/// Identifier type to uniquely represent a professor
pub type ProfessorId = crate::primitives::UniqId;

/// Identifier type to uniquely represent a offer made
pub type OfferId = crate::primitives::UniqId;

pub trait ProfessorIdDef: Clone + Decode + Encode + MaxEncodedLen + TypeInfo + Debug + Eq {}
impl ProfessorIdDef for ProfessorId {}

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

/// Offer Info
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct OfferInfo {
	pub professor: ProfessorId,
	pub university: crate::university::UniversityId,
	pub contract_file: StdIpfsLink,
}
