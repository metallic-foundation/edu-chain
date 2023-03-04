/// Identifier type to uniquely represent a university
pub type UniversityId = crate::UniqId;

use codec::Decode;
use codec::Encode;
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewUniversityParam {

}

/// information stored of a university
pub struct University {

}
