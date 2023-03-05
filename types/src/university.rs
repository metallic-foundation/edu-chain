/// Identifier type to uniquely represent a university
pub type UniversityId = crate::primitives::UniqId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewUniversityParam {}

/// information stored of a university
pub struct University {}
