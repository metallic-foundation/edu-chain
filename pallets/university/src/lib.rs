#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use traits::pallet_provider as pallet_provider_traits;
	use types::{
		professor::{NewProfessorParam, ProfessorId},
		student::StudentId,
		university::*,
		IpfsLink,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type ProfessorProvider: pallet_provider_traits::ProfessorProvider;
		type StudentProvider: pallet_provider_traits::StudentProvider;
		type LectureProvider: pallet_provider_traits::LectureProvider;
		type ExamProvider: pallet_provider_traits::ExamProvider;
		type ScholarshipProvider: pallet_provider_traits::ScholarshipProvider;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn register_university(
			origin: OriginFor<T>,
			university_id: UniversityId,
			info: NewUniversityParam,
		) -> DispatchResult {
			// regitser this university with given info
			// info might be something like:
			// Info {
			//      permanent: Hash(IpfsLink),
			//
			//  }

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_professor(
			origin: OriginFor<T>,
			university_id: UniversityId,
			professor_id: ProfessorId,
			info: NewProfessorParam,
		) -> DispatchResult {
			// sign a contract by university to add the already registered professor to this
			// university note: professor_id is independent to university_id. i.e professor can
			// exists without affiliated to any university
			//
			// also this addition has also to be signed by professor to validate the addition
			// contract the unsigned contract has the death period of X block number i.e after X
			// block this contract will be invilated and shall be re-issued if required. so
			// professor is required to sign the contract beforehand that also depend the fee on
			// value of X block number. the longer the temporary contract period larger the fee for
			// university
			//
			// add into map like:
			// UniversityProfessors: [university_id, professor_id] -> faculty_info / other info
			// also in ProfessorProvider it makes sense to have another storage synced to this
			// Professors: [professor_id] -> Info { university: vec![university_id] }

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn accept_student_application(
			origin: OriginFor<T>,
			application_id: (),
		) -> DispatchResult {
			// accept the application of university to enroll in this course
			// while sending the application, student also specify on which intake they want to join
			// after that intake duration has passed, the application will be removed ( unless
			// specified by student )

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn issue_certificate(
			origin: OriginFor<T>,
			student_id: StudentId,
			certificate: IpfsLink,
		) -> DispatchResult {
			// issue a certificate to student student_id
			// cerificate is the ipfs link to certificate docoument
			// or certificate can be a detailed struct stored in chain
			// and making a presentable certificate camn be offloaded to front-end side
			// we will receive a html file ( with no js and no external stylesheet )
			// approve it and then pass these details to make a certofocate accorsing to the
			// university design

			Ok(())
		}
	}
}

impl<T: Config> traits::pallet_provider::UniversityProvider for Pallet<T> {
	fn get_university_info() {
		todo!()
	}
}
