pub type ValidatorId = crate::primitives::UniqId;

use crate::StdIpfsLink;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

/// Information of the individual validator
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub struct ValidatorInfo<AccountId> {
    pub document: StdIpfsLink,
    pub admin: AccountId,
}

/// Information of the validated university
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
pub struct ValidateUniversityParam<UniversityId> {
    pub university: UniversityId,
}

/// Information of the validated professor
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
pub struct ValidateProfessorParam<ProfessorId> {
    pub professor: ProfessorId,
}

/// Information of the validated university
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub struct ValidatedUniversityInfo {}

/// Information of the validated professor
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub struct ValidatedProfessorInfo {}

/// Information required to register a new validator
#[derive(Decode, Encode, TypeInfo, Debug, Eq, PartialEq, Clone, MaxEncodedLen)]
pub struct NewValidatorParam {
    pub document: StdIpfsLink,
}

pub type ValidatorInfoFor<T> = ValidatorInfo<crate::AccountIdOf<T>>;
