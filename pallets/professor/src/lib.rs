#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use traits::pallet_provider as pallet_provider_traits;
	use types::{
		primitives::{AccountIdOf, StdIpfsLink as IpfsLink},
		professor::*,
		university::UniversityId,
	};

	type NewProfessorParamFor = NewProfessorParam;
	type ProfessorInfoFor<T> = ProfessorInfo<AccountIdOf<T>>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type UniversityProvider: pallet_provider_traits::UniversityProvider;
		type StudentProvider: pallet_provider_traits::StudentProvider;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Professors<T> = StorageMap<_, Twox64Concat, ProfessorId, ProfessorInfoFor<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New professor have been registered under `ProfessorId`
		NewProfessor(ProfessorId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Origin cannot perform this action
		InsufficientPermission,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_0000)]
		pub fn register_as_professor(
			origin: OriginFor<T>,
			professor_id: ProfessorId,
			info: NewProfessorParamFor,
		) -> DispatchResult {
			// register this user as professor under professor_id
			// identity is the permanent identity of this user which will be hash of ipfs link
			// info is changable informarmation about professor ( eg: which university is he
			// associated to )
			let professor =
				ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;

			let NewProfessorParam { info } = info;
			let professor_info = ProfessorInfoFor::<T> { info, professor };

			<Professors<T>>::insert(professor_id.clone(), professor_info);
			Self::deposit_event(Event::<T>::NewProfessor(professor_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn approve_staff_inclusion(
			origin: OriginFor<T>,
			university_id: UniversityId,
			info_to_approve: (),
		) -> DispatchResult {
			// Once university calls university::add_staff
			// the staff will be temporary located in a storage
			// before that storage explires, respective staff ought to call this to approve the
			// inclusion also staff need to pass the required info to match against one added by
			// university this is same as contract signing by staff side
			//
			// also add the list of university in the professor info
			// so we can query "get all university this professor is associated with"
			// without needing to iterate all university ( which will be impractical )

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn announce_thesis(
			origin: OriginFor<T>,
			thesis_id: (),
			faculty: (),
			thesis_requirements: (),
			info: IpfsLink,
		) -> DispatchResult {
			// announce that the thesis is due submission for all students in faculty
			// thesis_id is a unique hash that will be generated from front-end app for easy purpose
			// thesis_requirement is the link to ipfs document that describe what the thesis should
			// be about info is the external info of this thesis ( eg: submission due date, credit
			// weight )

			Ok(())
		}
	}
}

impl<T: Config> traits::pallet_provider::ProfessorProvider for Pallet<T> {
	fn get_staff_info() {
		todo!()
	}
}
