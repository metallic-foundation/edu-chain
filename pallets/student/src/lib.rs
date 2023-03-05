#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::{OriginFor, *};
	use traits::pallet_provider as pallet_provider_traits;
	use types::{student::*, university::UniversityId, IpfsLink};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type ProfessorProvider: pallet_provider_traits::ProfessorProvider;
		type UniversityProvider: pallet_provider_traits::UniversityProvider;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn send_application(
			origin: OriginFor<T>,
			university_id: UniversityId,
			application_id: (),
			student_info: (),
			course_id: (),
		) -> DispatchResult {
			// send application to get admission in university
			//

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn submit_thesis(
			origin: OriginFor<T>,
			thesis_id: (),
			thesis: IpfsLink,
		) -> DispatchResult {
			// way to submit the thesis
			// thesis-id will be some unique id ( pref. string hash which can be generted from
			// client side app ) thesis: will be the document link to ipfs where thesis is actually
			// written

			// also keep the status of this submission
			// which shall be approved by the professor of thesis_id
			// and always keep the submission if the thesis is rejected ( with rejected status )

			// Question: also provide a way to have a time bound on until when the thesis
			// can be submitted ( this info will be stored under thesis_id )

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn accept_scholarship(origin: OriginFor<T>, scholarship_id: ()) -> DispatchResult {
			// once the hoster of scholarship grant the scholarship to this student
			// student shall accept the scholarship to finalize the process
			//
			// TODO:
			// find a way to always keep this balance non-withdrable
			// i.e shall only be used within the university
			// and non-used credits can be re-used somehow
			// this can be done by creating a new pallet_balance type
			// interface which have one more filed like locked,free,reserved
			// i.e field grant which act as free to be used within academic and
			// act as locked when doing other transaction

			Ok(())
		}
	}
}

impl<T: Config> traits::pallet_provider::StudentProvider for Pallet<T> {
	fn get_student_info() {
		todo!()
	}
}
