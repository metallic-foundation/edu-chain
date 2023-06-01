#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use traits::pallet_provider as pallet_provider_traits;
    use types::{exam::*, professor::ProfessorId, university::UniversityId};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type UniversityProvider: pallet_provider_traits::UniversityProvider;
        type ProfessorProvider: pallet_provider_traits::ProfessorProvider;
        type StudentProvider: pallet_provider_traits::StudentProvider;
        type LectureProvider: pallet_provider_traits::LectureProvider;
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
        pub fn create_exam(
            origin: OriginFor<T>,
            university: UniversityId,
            faculty: (),
            professor: ProfessorId,
            exam_id: ExamId,
            exam_details: NewExamParam,
        ) -> DispatchResult {
            // create an exam which all students under this universitry under
            // this faculty have to give
            // professor is the exam inviligidator

            // TODO:
            // find a way to submit the questionarries but hide it till the
            // exam starts

            Ok(())
        }
    }
}

impl<T: Config> traits::pallet_provider::ExamProvider for Pallet<T> {
    fn get_exam_info() {
        todo!()
    }
}
