#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use traits::pallet_provider as pallet_provider_traits;
    use types::scholarship::*;

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
        type ExamProvider: pallet_provider_traits::ExamProvider;
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
        pub fn register_scholarship(
            origin: OriginFor<T>,
            scholarship_id: ScholarshipId,
            info: (),
        ) -> DispatchResult {
            // create a scholarship grant under scholarship_id
            //
            // info contains following and more information:
            // - no. of student to provide this grant to
            // - fund balance to have enough amount to grant that much credit
            // - optional list of university / faculty to whom to provide this scholarship

            Ok(())
        }
    }
}

impl<T: Config> traits::pallet_provider::ScholarshipProvider for Pallet<T> {
    fn get_scholarship_info() {
        todo!()
    }
}
