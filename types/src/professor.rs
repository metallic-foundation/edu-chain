/// Identifier type to uniquely represent a professor
pub type ProfessorId = crate::primitives::UniqId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// Required paramater to register this professor
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewProfessorParam {}

/// Information of a professor
pub struct Professor {}
