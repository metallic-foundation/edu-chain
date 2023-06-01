#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, Twox64Concat};
    use frame_system::pallet_prelude::*;
    use pallet_provider_traits::UniversityProvider;
    use traits::pallet_provider as pallet_provider_traits;
    pub(super) use types::intake::*;
    use types::{primitives::*, university::UniversityId, BlockNumberOf};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type ProfessorProvider: pallet_provider_traits::ProfessorProvider;
        type UniversityProvider: pallet_provider_traits::UniversityProvider<
            UniversityId = types::university::UniversityId,
            FrameConfig = Self,
        >;
    }

    type StudentIdOf<T> = AccountIdOf<T>;
    type UniversityIdOf<T> =
        <<T as Config>::UniversityProvider as UniversityProvider>::UniversityId;
    pub(crate) type IntakeIdOf<T> = IntakeId<UniversityIdOf<T>>;
    pub(crate) type IntakeApplicationOf<T> = IntakeApplication<BlockNumberFor<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New intake application announced
        NewIntakeAnnounced(IntakeIdOf<T>),
        /// New intake application submitted
        AppliedForIntake(IntakeIdOf<T>, StudentIdOf<T>),
        /// Application has been withdrawn
        ApplicationWithdrawn(IntakeIdOf<T>, StudentIdOf<T>),
        /// application has been accepted by university
        ApplicationAccepted(IntakeIdOf<T>, StudentIdOf<T>),
        /// An intake have been finalised
        IntakeFinalised(IntakeIdOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Cannot perform this action due to mismatched permission
        InsufficientPermission,
        /// Intake already exists
        IntakeExists,
        /// No such university
        NonExistentUniversity,
        /// Invalid parameter
        InvalidParamater,
        /// Intake does not exists
        NonExistentIntake,
        /// Intake closed
        IntakeClosed,
        /// Intake is still ongoing
        IntakeOngoing,
        /// application does not exsist
        NonExistentApplication,
        /// Intake is not closed
        IntakeNotClosed,
    }

    #[pallet::storage]
    #[pallet::getter(fn get_intake)]
    pub type Intakes<T> = StorageMap<_, Twox64Concat, IntakeIdOf<T>, IntakeInfo<BlockNumberOf<T>>>;

    #[pallet::storage]
    #[pallet::getter(fn get_intake_closing_date)]
    pub type IntakeClosingDateLookup<T> =
        StorageDoubleMap<_, Twox64Concat, BlockNumberOf<T>, Twox64Concat, IntakeIdOf<T>, ()>;

    #[pallet::storage]
    pub type LastUniIntake<T> = StorageMap<_, Twox64Concat, UniversityIdOf<T>, IntakeIdOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_accepted_application)]
    pub type AcceptedApplications<T> =
        StorageDoubleMap<_, Twox64Concat, IntakeIdOf<T>, Twox64Concat, StudentIdOf<T>, ()>;

    #[pallet::storage]
    pub type Applications<T> = StorageDoubleMap<
        _,
        Twox64Concat,
        IntakeIdOf<T>,
        Twox64Concat,
        StudentIdOf<T>,
        IntakeApplicationOf<T>,
    >;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn apply_for_intake(
            origin: OriginFor<T>,
            intake_id: IntakeIdOf<T>,
            mut application: IntakeApplicationOf<T>,
        ) -> DispatchResult {
            let student_id =
                ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;

            // Get intake info
            let intake_info = Self::get_intake(&intake_id).ok_or(Error::<T>::NonExistentIntake)?;
            // ensure instake status is open
            ensure!(
                intake_info.status == IntakeStatus::IntakeOngoing,
                Error::<T>::IntakeClosed,
            );
            // ensure this student has not applied for same intake before
            ensure!(
                !Applications::<T>::contains_key(&intake_id, &student_id),
                Error::<T>::InvalidParamater,
            );

            // modify the application applied date
            application.applied_on = Self::current_block_number();

            // put the application
            Applications::<T>::insert(&intake_id, &student_id, application);

            // emit event
            Self::deposit_event(Event::AppliedForIntake(intake_id, student_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn announce_intake_application(
            origin: OriginFor<T>,
            intake_id: IntakeIdOf<T>,
            university_id: UniversityId,
            intake_info: types::intake::NewIntakeParam<BlockNumberFor<T>>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;
            let university_admin = T::UniversityProvider::university_admin(&university_id)
                .ok_or(Error::<T>::NonExistentUniversity)?;

            // only university can announce new intake
            ensure!(
                signer == university_admin,
                Error::<T>::InsufficientPermission
            );
            // intake id must be unique
            ensure!(
                !Intakes::<T>::contains_key(&intake_id),
                Error::<T>::IntakeExists
            );

            let types::intake::NewIntakeParam {
                application_opens,
                application_closes,
                max_accepted,
                max_applicants,
            } = intake_info;
            // application closing date and opening date should be valid
            ensure!(
                application_closes > application_opens,
                Error::<T>::InvalidParamater
            );
            // max_accepted & max_applicants must be greater than 0
            // max_applicants must be greater than max_accepted
            ensure!(
                max_applicants > 0 && max_accepted > 0 && max_accepted <= max_applicants,
                Error::<T>::InvalidParamater
            );

            // get the intake status
            let intake_status = if Self::current_block_number() >= application_opens {
                IntakeStatus::IntakeOngoing
            } else {
                IntakeStatus::IntakePending
            };

            let intake_info = IntakeInfo {
                application_opens,
                application_closes,
                max_applicants,
                max_accepted,
                status: intake_status,
            };
            Intakes::<T>::insert(&intake_id, &intake_info);
            LastUniIntake::<T>::insert(&university_id, &intake_id);

            Self::deposit_event(Event::<T>::NewIntakeAnnounced(intake_id));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn withdraw_application(
            origin: OriginFor<T>,
            intake_id: IntakeIdOf<T>,
        ) -> DispatchResult {
            let student_id =
                ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;

            // ensure the application was applied for
            ensure!(
                Applications::<T>::contains_key(&intake_id, &student_id),
                Error::<T>::InvalidParamater,
            );

            // ensure intake exists
            let intake_info = Self::get_intake(&intake_id).ok_or(Error::<T>::NonExistentIntake)?;
            // application can be revoked only on ongoing intakes
            ensure!(
                intake_info.status == IntakeStatus::IntakeOngoing,
                Error::<T>::IntakeClosed,
            );

            // remove the application
            Applications::<T>::remove(&intake_id, &student_id);

            // emit the event
            Self::deposit_event(Event::ApplicationWithdrawn(intake_id, student_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn finalise_intake(origin: OriginFor<T>, intake_id: IntakeIdOf<T>) -> DispatchResult {
            let signer = ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;
            let university_admin =
                T::UniversityProvider::university_admin(&intake_id.university_id)
                    .ok_or(Error::<T>::NonExistentUniversity)?;
            ensure!(
                signer == university_admin,
                Error::<T>::InsufficientPermission
            );

            // ensure intake status us Closed
            let intake_info = Self::get_intake(&intake_id).ok_or(Error::<T>::NonExistentIntake)?;
            ensure!(
                intake_info.status == IntakeStatus::IntakeClosed,
                Error::<T>::IntakeNotClosed,
            );

            // update status to Finalised
            Intakes::<T>::mutate(&intake_id, |intake_info| {
                if let Some(intake_info) = intake_info {
                    intake_info.status = IntakeStatus::IntakeFinalised
                };
            });

            // remove all applications
            Applications::<T>::remove_prefix(&intake_id, None);

            // emit the event
            Self::deposit_event(Event::<T>::IntakeFinalised(intake_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn accept_application(
            origin: OriginFor<T>,
            intake_id: IntakeIdOf<T>,
            student_id: StudentIdOf<T>,
        ) -> DispatchResult {
            let signer = ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;
            let university_admin =
                T::UniversityProvider::university_admin(&intake_id.university_id)
                    .ok_or(Error::<T>::NonExistentUniversity)?;

            // only university can accept application
            ensure!(
                signer == university_admin,
                Error::<T>::InsufficientPermission
            );

            // get intake info
            let intake_info = Self::get_intake(&intake_id).ok_or(Error::<T>::NonExistentIntake)?;
            // ensre intake is closed
            ensure!(
                intake_info.status == IntakeStatus::IntakeClosed,
                Error::<T>::IntakeOngoing,
            );

            // ensure application of this student exists
            ensure!(
                Applications::<T>::contains_key(&intake_id, &student_id),
                Error::<T>::NonExistentApplication,
            );

            // ensure this student has not been accepted before
            ensure!(
                !AcceptedApplications::<T>::contains_key(&intake_id, &student_id),
                Error::<T>::InvalidParamater,
            );

            // put into accepted applications
            AcceptedApplications::<T>::insert(&intake_id, &student_id, ());

            // emit event
            Self::deposit_event(Event::ApplicationAccepted(intake_id, student_id));

            Ok(())
        }
    }

    // extrinsic helpers
    impl<T: Config> Pallet<T> {
        pub fn current_block_number() -> BlockNumberOf<T> {
            <frame_system::Pallet<T>>::block_number()
        }
    }

    // implement  a hook to delete intake info when the intake is closed
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberOf<T>> for Pallet<T> {
        fn on_initialize(current_block_number: BlockNumberOf<T>) -> Weight {
            let mut weight_consumed = Weight::zero();

            // delete intake info when the intake closing date has passed
            weight_consumed += Self::mark_close_intakes(current_block_number);

            // return weight consumed
            weight_consumed
        }
    }

    // hooks helpers
    impl<T: Config> Pallet<T> {
        /// delete intake info when the intake closing date has passed
        fn mark_close_intakes(block_number: BlockNumberOf<T>) -> Weight {
            let weight_consumed = Weight::zero();

            let to_expire_intakes = IntakeClosingDateLookup::<T>::drain_prefix(&block_number);
            for (intake_id, _) in to_expire_intakes {
                Intakes::<T>::mutate(&intake_id, |intake_info| {
                    if let Some(intake_info) = intake_info {
                        intake_info.status = IntakeStatus::IntakeClosed;
                    }
                });
            }

            weight_consumed
        }
    }
}

impl<T: Config> traits::pallet_provider::IntakeProvider for Pallet<T> {
    type IntakeId = IntakeIdOf<T>;
    type IntakeInfo = IntakeInfo<types::BlockNumberOf<T>>;

    fn intake_info(intake_id: &Self::IntakeId) -> Option<Self::IntakeInfo> {
        Intakes::<T>::get(intake_id)
    }
}
