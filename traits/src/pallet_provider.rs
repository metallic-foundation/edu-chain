pub trait StudentProvider {
	fn get_student_info();
}

pub trait UniversityProvider {
	fn get_university_info();
}

pub trait StaffProvider {
	fn get_staff_info();
}

pub trait ValidationProvider {
	fn is_verified_university() -> bool;
}
