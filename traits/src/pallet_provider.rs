use frame_support::pallet_prelude::*;

pub trait StudentProvider {
	type StudentId;
	type ApplicationId;
	type ApplicationInfo;

	fn get_student_info();
	fn application_info(
		application_id: &Self::ApplicationId,
	) -> Result<Self::ApplicationInfo, DispatchError>;
}

pub trait UniversityProvider {
	type UniversityId;
	type UniversityInfo;

	fn university_info(university_id: &Self::UniversityId) -> Option<Self::UniversityInfo>;
}

pub trait ProfessorProvider {
	type ProfessorId; // types::professor::ProfessorIdDef;
	type ProfessorInfo;

	fn professor_info(professor_id: &Self::ProfessorId) -> Option<Self::ProfessorInfo>;
}

pub trait IntakeProvider {
	type IntakeId;
	type IntakeInfo;

	fn intake_info(intake_id: &Self::IntakeId) -> Option<Self::IntakeInfo>;
}

pub trait LectureProvider {
	fn get_lecture_info();
}

pub trait ExamProvider {
	fn get_exam_info();
}

pub trait ScholarshipProvider {
	fn get_scholarship_info();
}

pub trait ValidationProvider {
	fn is_verified_university() -> bool;
}
