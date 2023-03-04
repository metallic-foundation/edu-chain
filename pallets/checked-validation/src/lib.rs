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

		type UniversityProvider: pallet_provider_traits::UniversityProvider;
		type StudentProvider: pallet_provider_traits::StudentProvider;
		type ProfessorProvider: pallet_provider_traits::ProfessorProvider;
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
		pub fn validate_university(
			origin: OriginFor<T>,
			validator_id: (),
			validated_kyc: (),
		) -> DispatchResult {
			// provide a mechanism to have a list of validator
			// and the edu-chain validatior will be assigned validator_id = 0
			// this provides a mechanism to validate the kyc of university
			// and this validation can be other than edu-chain itself
			// it's upto client to trust which validator

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_validator(
			origin: OriginFor<T>,
			validator_info: (),
			validated_kyc: (),
		) -> DispatchResult {
			// extrinsic to add validator source
			// the validator of edu-chain will be added from genesis-config

			Ok(())
		}
	}
}

impl<T: Config> traits::pallet_provider::ValidationProvider for Pallet<T> {
	fn is_verified_university() -> bool {
		todo!()
	}
}
