// Identifier type to uniquely represent a scholarship
pub type ScholarshipId = crate::primitives::UniqId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// Required paramater to register this scholarship
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewScholarshipParam {}

/// Information of a scholarship
pub struct Scholarship {}
