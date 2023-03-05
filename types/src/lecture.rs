/// Identifier type to uniquely represent a lecture
pub type LectureId = crate::primitives::UniqId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

/// Required paramater to register this lecture
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewLectureParam {}

/// Information of a lecture
pub struct Lecture {}
