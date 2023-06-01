use crate::primitives::StdIpfsLink;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

// Uniquely identify a intake
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct IntakeId<UniversityId> {
    pub university_id: UniversityId,
    pub intake_index: u32,
    // todo:
    // pub itake_course_id: CourseId,
}

/// Required paramater to register this lecture
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub struct NewIntakeParam<BlockNumber> {
    pub application_opens: BlockNumber,
    pub application_closes: BlockNumber,
    pub max_applicants: u32,
    pub max_accepted: u32,
}

/// Information of a lecture
#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct IntakeInfo<BlockNumber> {
    pub application_opens: BlockNumber,
    pub application_closes: BlockNumber,
    pub max_applicants: u32,
    pub max_accepted: u32,
    pub status: IntakeStatus,
}

#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen)]
pub enum IntakeStatus {
    /// Intake is pending to be open
    IntakePending,
    /// Intake is open and ongoing
    IntakeOngoing,
    /// Intake is closed and is being processed
    IntakeClosed,
    /// Intake is closed and finalised
    IntakeFinalised,
}

#[derive(Decode, Encode, TypeInfo, Clone, PartialEq, Eq, Debug, MaxEncodedLen)]
pub struct IntakeApplication<BlockNumber> {
    /// block number when application was made
    pub applied_on: BlockNumber,
    /// application details
    pub info: StdIpfsLink,
}

/// limit on how many application can be accepted at once
pub type AcceptApplicationLimit = u32;
