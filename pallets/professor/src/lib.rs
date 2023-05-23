#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, Twox64Concat};
    use frame_system::pallet_prelude::*;
    use pallet_provider_traits::*;
    pub(super) use traits::pallet_provider as pallet_provider_traits;
    pub(super) use types::{
        primitives::{AccountIdOf, StdIpfsLink as IpfsLink},
        professor::*,
        university::UniversityId,
    };

    pub(super) type NewProfessorParamFor = NewProfessorParam;
    pub(super) type ProfessorInfoFor<T> = ProfessorInfo<AccountIdOf<T>>;
    pub(super) type UniversityInfoFor<T> = types::university::University<types::AccountIdOf<T>>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type UniversityProvider: UniversityProvider<
            UniversityId = UniversityId,
            UniversityInfo = UniversityInfoFor<Self>,
        >;
        type StudentProvider: StudentProvider;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_professor)]
    pub type Professors<T> = StorageMap<_, Twox64Concat, ProfessorId, ProfessorInfoFor<T>>;

    /// These are previouslt used professor's Id
    /// i.e some professor was registered with this ID
    /// and later he deregistered himself
    /// but we cannot allow this id to be taken again by anyone ( not even the same accountId of
    /// previous professor )
    ///
    /// - reason: example: there might be offer made to professorIdOne but he
    /// deregister then if someone else register as professorIdOne then he can own the offer
    /// but this is not something intentional
    ///
    /// another solution would have been to remove all data of this ProfessorId while deregistering
    /// but this is always not possible as ProfessorId might not always be the key to any storage
    /// and iteration over every value in search of this professorId would not make much sense
    ///
    /// another solution would be to use the same Professors<T>
    /// but while de-regestering instead of removing put the professor in wiped state
    /// but it would be more storage-wise cheaper to have just a map in new storage
    #[pallet::storage]
    pub type UnusableProfessorId<T> = StorageMap<_, Identity, ProfessorId, ()>;

    #[pallet::storage]
    #[pallet::getter(fn get_offer)]
    pub type Offers<T> = StorageMap<_, Twox64Concat, ProfessorId, OfferInfo>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New professor have been registered under `ProfessorId`
        NewProfessor(ProfessorId),
        /// A new offer have been made
        OfferMade(OfferId),
        /// Offer have been accepted
        OfferAccepted(OfferId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Origin cannot perform this action
        InsufficientPermission,
        ///
        ProfessorExists,
        /// No such professor
        NoProfessor,
        /// NO such univeristy
        NoUniversity,
        /// No such offer
        NoOffer,
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
            Self::ensure_professor_id_is_unique(&professor_id)?;

            let NewProfessorParam { info } = info;
            let professor_info = ProfessorInfoFor::<T> { info, professor };

            <Professors<T>>::insert(professor_id.clone(), professor_info);
            Self::deposit_event(Event::<T>::NewProfessor(professor_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn make_offer_for_uni(
            origin: OriginFor<T>,
            offer_id: OfferId,
            offer: OfferInfo,
        ) -> DispatchResult {
            let signer = ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;
            let university = T::UniversityProvider::university_info(&offer.university)
                .ok_or(Error::<T>::NoUniversity)?;
            ensure!(
                signer == university.admin,
                Error::<T>::InsufficientPermission
            );

            // TODO:
            // make some checks to see if this offer can be made. Example:
            // - limit of offer to accept by professor
            // - limit of offer to make by professor
            // - if professor is open to work

            Offers::<T>::insert(&offer_id, offer);
            Self::deposit_event(Event::<T>::OfferMade(offer_id));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn accept_offer(origin: OriginFor<T>, offer_id: OfferId) -> DispatchResult {
            // Once university calls university::add_staff
            // the staff will be temporary located in a storage
            // before that storage explires, respective staff ought to call this to approve the
            // inclusion also staff need to pass the required info to match against one added by
            // university this is same as contract signing by staff side
            //
            // also add the list of university in the professor info
            // so we can query "get all university this professor is associated with"
            // without needing to iterate all university ( which will be impractical )

            let signer = ensure_signed(origin)?;
            let offer_info = Self::get_offer(&offer_id).ok_or(Error::<T>::NoOffer)?;
            let professor =
                Self::get_professor(&offer_info.professor).ok_or(Error::<T>::NoProfessor)?;
            ensure!(
                signer == professor.professor,
                Error::<T>::InsufficientPermission
            );

            ensure!(Offers::<T>::contains_key(&offer_id), Error::<T>::NoOffer);
            // TODO:
            // do something to signify the acceptance of offer.
            // example: adding the professor to university, assigning classes etc.
            Offers::<T>::remove(&offer_id);

            Self::deposit_event(Event::<T>::OfferAccepted(offer_id));

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

    impl<T: Config> Pallet<T> {
        fn ensure_professor_id_is_unique(professor_id: &ProfessorId) -> DispatchResult {
            ensure!(
                !Professors::<T>::contains_key(professor_id),
                Error::<T>::ProfessorExists
            );
            ensure!(
                !UnusableProfessorId::<T>::contains_key(professor_id),
                Error::<T>::ProfessorExists
            );
            Ok(())
        }
    }
}

use crate::{ProfessorId, ProfessorInfoFor};
impl<T: Config> traits::pallet_provider::ProfessorProvider for Pallet<T> {
    type ProfessorId = crate::ProfessorId;
    type ProfessorInfo = crate::ProfessorInfoFor<T>;

    fn professor_info(professor_id: &Self::ProfessorId) -> Option<Self::ProfessorInfo> {
        crate::Pallet::<T>::get_professor(professor_id)
    }
}
