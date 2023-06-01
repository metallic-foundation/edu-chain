/// Identifier type to uniquely represent a exam
pub type ExamId = crate::primitives::UniqId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewExamParam {}

/// information stored of a exam
pub struct Exam {}
