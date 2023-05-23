#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{DispatchResult, *},
		Twox64Concat,
	};
	use frame_system::pallet_prelude::*;
	use pallet_provider_traits::StudentProvider;
	pub(super) use traits::pallet_provider as pallet_provider_traits;
	pub(super) use types::{
		primitives::{AccountIdOf, StdIpfsLink as IpfsLink},
		professor::{NewProfessorParam, ProfessorId},
		student::StudentId,
		university::*,
	};

	pub(super) type UniversityInfoFor<T> = University<AccountIdOf<T>>;
	type NewUniversityParamFor<T> = NewUniversityParam<AccountIdOf<T>>;
	type ProfessorIdFor<T> = <<T as Config>::ProfessorProvider as pallet_provider_traits::ProfessorProvider>::ProfessorId;

	pub(super) type StudentApplicationIdFor<T> =
		<<T as Config>::StudentProvider as StudentProvider>::ApplicationId;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type ProfessorProvider: pallet_provider_traits::ProfessorProvider<ProfessorId = ProfessorId>;
		type StudentProvider: StudentProvider<
			ApplicationId = types::student::ApplicationId,
			ApplicationInfo = types::student::Application<types::AccountIdOf<Self>>,
		>;
		type LectureProvider: pallet_provider_traits::LectureProvider;
		type ExamProvider: pallet_provider_traits::ExamProvider;
		type ScholarshipProvider: pallet_provider_traits::ScholarshipProvider;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_university)]
	pub type Universities<T> =
		StorageMap<_, Twox64Concat, UniversityId, UniversityInfoFor<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_student)]
	pub type Students<T> =
		StorageDoubleMap<_, Twox64Concat, UniversityId, Twox64Concat, StudentId, ()>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Application have been accepted
		ApplicationAccepted(StudentApplicationIdFor<T>),
		/// UniversityRegistered
		NewUniversity(UniversityId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Origin cannot perform this action
		InsufficientPermission,
		/// No university with given id exists
		NoUniversity,
		/// University with this id already exists
		UniversityExists,
		/// Professor does not exists
		NoProfessor,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn register_university(
			origin: OriginFor<T>,
			university_id: UniversityId,
			info: NewUniversityParamFor<T>,
		) -> DispatchResult {
			// regitser this university with given info
			// info might be something like:
			// Info {
			//      permanent: Hash(IpfsLink),
			//
			//  }

			let signer = ensure_signed(origin)?;
			Self::verify_new_id(&university_id)?;

			let NewUniversityParam { admin, permanent_info } = info;
			let admin = admin.unwrap_or(signer);

			let university = UniversityInfoFor::<T> { admin, permanent_info };

			<Universities<T>>::insert(&university_id, university);
			Self::deposit_event(Event::<T>::NewUniversity(university_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_professor(
			origin: OriginFor<T>,
			university_id: UniversityId,
			professor_id: ProfessorIdFor<T>,
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

			Self::ensure_university_admin(origin, &university_id)?;
			<<T as Config>::ProfessorProvider as pallet_provider_traits::ProfessorProvider>
                ::professor_info(&professor_id).ok_or(Error::<T>::NoProfessor)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn accept_student_application(
			origin: OriginFor<T>,
			application_id: StudentApplicationIdFor<T>,
		) -> DispatchResult {
			// accept the application of university to enroll in this course
			// while sending the application, student also specify on which intake they want to join
			// after that intake duration has passed, the application will be removed ( unless
			// specified by student )
			let signer = ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;
			let application_info = T::StudentProvider::application_info(&application_id)?;
			Self::verify_university_admin(signer, &application_info.university)?;

			// - Do something to signify acceptance of enrollment application
			// - delete the application

			Self::deposit_event(Event::<T>::ApplicationAccepted(application_id));

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

	impl<T: Config> Pallet<T> {
		pub fn verify_new_id(university_id: &UniversityId) -> DispatchResult {
			ensure!(!Universities::<T>::contains_key(university_id), Error::<T>::UniversityExists,);
			Ok(())
		}

		pub fn ensure_university_admin(
			origin: OriginFor<T>,
			university_id: &UniversityId,
		) -> DispatchResult {
			Self::verify_university_admin(
				ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?,
				university_id,
			)
		}

		pub fn verify_university_admin(
			signer: types::AccountIdOf<T>,
			university_id: &UniversityId,
		) -> DispatchResult {
			ensure!(
				Self::get_university(university_id)
					.map(|info| signer == info.admin)
					.ok_or(Error::<T>::NoUniversity)?,
				Error::<T>::InsufficientPermission
			);
			Ok(())
		}
	}
}

impl<T: Config> pallet_provider_traits::UniversityProvider for Pallet<T> {
	type UniversityId = crate::UniversityId;
	type UniversityInfo = crate::UniversityInfoFor<T>;
	type FrameConfig = T;

	fn university_info(university_id: &Self::UniversityId) -> Option<Self::UniversityInfo> {
		crate::Pallet::<T>::get_university(university_id)
	}

	fn university_admin(
		university_id: &Self::UniversityId,
	) -> Option<types::AccountIdOf<Self::FrameConfig>> {
		Self::university_info(university_id).map(|info| info.admin)
	}
}
