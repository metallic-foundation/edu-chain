pub trait StudentProvider {
	fn get_student_info();
}

pub trait UniversityProvider {
	fn get_university_info();
}

pub trait ProfessorProvider {
	fn get_staff_info();
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
