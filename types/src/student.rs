/// Identifier type to uniquely represent a student
pub type StudentId = crate::primitives::UniqId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// parameter required to register as student
#[derive(Decode, Encode, TypeInfo, Clone, Debug, PartialEq, Eq)]
pub struct NewStudentParam {}

/// information stored of a student
pub struct Student {}