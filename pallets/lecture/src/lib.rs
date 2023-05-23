#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use traits::pallet_provider as pallet_provider_traits;
    use types::{professor::ProfessorId, university::UniversityId};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type UniversityProvider: pallet_provider_traits::UniversityProvider;
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
        pub fn create_lecture(
            origin: OriginFor<T>,
            university: UniversityId,
            faculty: (),
            professor: ProfessorId,
            lecture_id: (),
            lecture_details: (),
        ) -> DispatchResult {
            // start a new lecture of this faculty of this university
            // lecture details (might) be the ipfs link to permanent lecture details
            // or onchain details like time start, time end, number of credit carries
            // etc..

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn student_attendance(
            origin: OriginFor<T>,
            lecture_id: (),
            student: (),
        ) -> DispatchResult {
            // way for a professor or university or dean to mark this student had attended
            // lecture_id
            // this will later be used to calculate the credit a student might get for attendance

            Ok(())
        }
    }
}

impl<T: Config> traits::pallet_provider::LectureProvider for Pallet<T> {
    fn get_lecture_info() {
        todo!()
    }
}
