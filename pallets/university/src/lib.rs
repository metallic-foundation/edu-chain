#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use traits::pallet_provider as pallet_provider_traits;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type ProfessorProvider: pallet_provider_traits::ProfessorProvider;
		type StudentProvider: pallet_provider_traits::StudentProvider;
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
		pub fn register_university(origin: OriginFor<T>, info: ()) -> DispatchResult {
			// regitser this university with given info
			// info might be something like:
			// Info {
			//      permanent: Hash(IpfsLink),
			//
			//  }

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_staff(origin: OriginFor<T>, staff_info: (), staff_role: ()) -> DispatchResult {
			// add to the multi-key map of
			// Staff: [unifersity_id, staff_id] -> StaffInfo
			// StaffByRole: [university_id, staff_role] -> staff_id

			// staff_info might contains:
			// permissions ( logic gatable enum ), name, email , contract duration.

			// the addition of staff is only valid if the staff also approve the addition
			// before that this is temporarily stored for X number of blocks
			// and fee is also dependent on how long the contract is stored

			// T::StaffProvider::new_staff()

			Ok(())
		}
	}
}

impl<T: Config> traits::pallet_provider::UniversityProvider for Pallet<T> {
	fn get_university_info() {
		todo!()
	}
}
