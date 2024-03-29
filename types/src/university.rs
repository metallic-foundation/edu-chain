/// Identifier type to uniquely represent a university
pub type UniversityId = crate::primitives::UniqId;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

use crate::primitives::StdIpfsLink as IpfsLink;

#[derive(Decode, Encode, TypeInfo, Clone, Debug, Eq, PartialEq)]
pub struct NewUniversityParam<AccountId> {
    pub admin: Option<AccountId>,
    pub permanent_info: IpfsLink,
}

/// information stored of a university
#[derive(Decode, Encode, TypeInfo, Clone, MaxEncodedLen, Debug, PartialEq, Eq)]
pub struct University<AccountId> {
    /// who own the right to modify this university
    // this will most probably be the multi-signature AccountId
    // to represent the group of stake_holders
    pub admin: AccountId,
    pub permanent_info: IpfsLink,
}
