#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::{OriginFor, *};
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
		pub fn add_professor(
			origin: OriginFor<T>,
			university_id: (),
			professor_id: (),
			info: (),
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
		pub fn issue_certificate(
			origin: OriginFor<T>,
			student_id: (),
			certificate: (),
		) -> DispatchResult {
			// issue a certificate to student student_id
			// cerificate is the ipfs link to certificate docoument

			Ok(())
		}
	}
}

impl<T: Config> traits::pallet_provider::UniversityProvider for Pallet<T> {
	fn get_university_info() {
		todo!()
	}
}
