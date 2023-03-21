#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec as SpStdVec;
use types::university::*;

sp_api::decl_runtime_apis! {
	pub trait PalletUniversityApi
	{
		fn universities_keys() -> SpStdVec<UniversityId>;
		fn university_by_id(id: UniversityId) -> Option<University>;
	}
}
