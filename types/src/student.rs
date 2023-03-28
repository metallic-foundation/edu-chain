/// Identifier type to uniquely represent a student
pub type StudentId = crate::primitives::UniqId;

pub type ApplicationId = crate::primitives::UniqId;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

use crate::StdIpfsLink;

/// parameter required to register as student
#[derive(Decode, Encode, TypeInfo, Clone, Debug, PartialEq, Eq, MaxEncodedLen)]
pub struct NewStudentParam {}

#[derive(Decode, Encode, TypeInfo, Clone, Debug, PartialEq, Eq, MaxEncodedLen)]
pub struct NewApplicationParam {
	pub university: crate::university::UniversityId,
	pub application: StdIpfsLink,
}

#[derive(Decode, Encode, TypeInfo, Clone, Debug, PartialEq, Eq, MaxEncodedLen)]
pub struct Application<AccountId> {
	pub applicant: AccountId,
	pub university: crate::university::UniversityId,
	pub application: StdIpfsLink,
}

/// information stored of a student
#[derive(Decode, Encode, TypeInfo, Clone, Debug, PartialEq, Eq, MaxEncodedLen)]
pub struct Student {}

pub type ApplicationInfoFor<T> = Application<crate::AccountIdOf<T>>;
