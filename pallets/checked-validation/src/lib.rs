#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, Twox64Concat};
	use frame_system::pallet_prelude::{OriginFor, *};
	use pallet_provider_traits::UniversityProvider;
	pub(super) use traits::pallet_provider as pallet_provider_traits;
	use traits::pallet_provider::ProfessorProvider;
	use types::validator::{
		ValidatedProfessorInfo, ValidatedUniversityInfo, ValidatorId, ValidatorInfoFor, *,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type UniversityProvider: UniversityProvider<
			UniversityId = types::university::UniversityId,
			UniversityInfo = types::university::University<types::AccountIdOf<Self>>,
		>;
		type StudentProvider: pallet_provider_traits::StudentProvider;
		type ProfessorProvider: ProfessorProvider<ProfessorId = types::professor::ProfessorId>;
	}

	type UniversityIdOf<T> =
		<<T as Config>::UniversityProvider as UniversityProvider>::UniversityId;
	type ProfessorIdOf<T> = <<T as Config>::ProfessorProvider as ProfessorProvider>::ProfessorId;
	type NewValidatedUniversityInfoFor<T> = ValidateUniversityParam<UniversityIdOf<T>>;
	type NewValidatedProfessorInfoFor<T> = ValidateProfessorParam<ProfessorIdOf<T>>;

	/// Storage that keeps track of validators
	/// key in ValidatorId hashed by Twox64Concat and value is ValidatorInfo
	#[pallet::storage]
	#[pallet::getter(fn get_validator)]
	pub type Validators<T> = StorageMap<_, Twox64Concat, ValidatorId, ValidatorInfoFor<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_university)]
	pub type Universities<T> = StorageDoubleMap<
		_,
		Twox64Concat,
		ValidatorId,
		Twox64Concat,
		UniversityIdOf<T>,
		ValidatedUniversityInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_professor)]
	pub type Professors<T> = StorageDoubleMap<
		_,
		Twox64Concat,
		ValidatorId,
		Twox64Concat,
		ProfessorIdOf<T>,
		ValidatedProfessorInfo,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New validator have been added
		ValidatorAdded(ValidatorId),
		/// Validator have been removed
		ValidatorRemoved(ValidatorId),
		/// University have been validated
		UniversityValidated { validated_by: ValidatorId, university: UniversityIdOf<T> },
		/// Professor have been validated
		ProfessorValidated { validated_by: ValidatorId, professor: ProfessorIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Caller have mismatched permission
		InsufficientPermission,
		/// Validator with given Id already exists
		ValidatorAlreadyExists,
		/// No validator with given Id exists
		ValidatorDoesNotExist,
		/// UniversityAlreadyValidated
		UniversityAlreadyValidated,
		/// ProfessorAlreadyValidated
		ProfessorAlreadyValidated,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a validator
		/// Can be called by any signed user
		/// need to pass ValidatorId as paramater and insert that into Validators storage
		/// weight is 10_000
		#[pallet::weight(10_000)]
		pub fn register_validator(
			origin: OriginFor<T>,
			validator_id: ValidatorId,
			validator_info: NewValidatorParam,
		) -> DispatchResult {
			let admin = ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)?;
			Self::verify_new_id(&validator_id)?;

			let NewValidatorParam { document } = validator_info;
			let validator_info = ValidatorInfo { document, admin };

			Validators::<T>::insert(&validator_id, validator_info);

			Self::deposit_event(Event::ValidatorAdded(validator_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn validate_university(
			origin: OriginFor<T>,
			validator_id: ValidatorId,
			validated_info: NewValidatedUniversityInfoFor<T>,
		) -> DispatchResult {
			// provide a mechanism to have a list of validator
			// and the edu-chain validatior will be assigned validator_id = 0
			// this provides a mechanism to validate the kyc of university
			// and this validation can be other than edu-chain itself
			// it's upto client to trust which validator

			Self::ensure_validator_admin(origin, &validator_id)?;
			ensure!(
				Universities::<T>::contains_key(&validator_id, &validated_info.university),
				Error::<T>::UniversityAlreadyValidated
			);

			let NewValidatedUniversityInfoFor::<T> { university: university_id } = validated_info;

			let validated_info = ValidatedUniversityInfo {};

			Universities::<T>::insert(&validator_id, &university_id, validated_info);

			Self::deposit_event(Event::<T>::UniversityValidated {
				validated_by: validator_id,
				university: university_id,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn validate_professor(
			origin: OriginFor<T>,
			validator_id: ValidatorId,
			validated_info: NewValidatedProfessorInfoFor<T>,
		) -> DispatchResult {
			Self::ensure_validator_admin(origin, &validator_id)?;
			ensure!(
				Professors::<T>::contains_key(&validator_id, &validated_info.professor),
				Error::<T>::ProfessorAlreadyValidated
			);

			let NewValidatedProfessorInfoFor::<T> { professor: professor_id } = validated_info;

			let validated_info = ValidatedProfessorInfo {};

			Professors::<T>::insert(&validator_id, &professor_id, validated_info);
			Self::deposit_event(Event::<T>::ProfessorValidated {
				validated_by: validator_id,
				professor: professor_id,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn ensure_validator_admin(
			origin: OriginFor<T>,
			validator_id: &ValidatorId,
		) -> DispatchResult {
			ensure!(
				ensure_signed(origin).map_err(|_| Error::<T>::InsufficientPermission)? ==
					Validators::<T>::get(validator_id)
						.ok_or(Error::<T>::ValidatorDoesNotExist)?
						.admin,
				Error::<T>::InsufficientPermission,
			);

			Ok(())
		}

		pub fn verify_new_id(validator_id: &ValidatorId) -> DispatchResult {
			ensure!(
				Validators::<T>::contains_key(validator_id),
				Error::<T>::ValidatorAlreadyExists,
			);

			Ok(())
		}
	}
	// #[pallet::genesis_config]
	// pub struct GenesisConfig<T: Config> {
	// 	pub validators: Vec<(ValidatorId, ValidatorInfoFor<T>)>,
	// 	pub universities: Vec<(ValidatorId, UniversityIdOf<T>, ValidatedUniversityInfo)>,
	// 	pub professors: Vec<(ValidatorId, ProfessorIdOf<T>, ValidatedProfessorInfo)>,
	// }

	// #[cfg(feature = "std")]
	// impl<T: Config> Default for GenesisConfig<T> {
	// 	fn default() -> Self {
	// 		Self {
	// 			validators: vec![],
	// 			universities: vec![],
	// 			professors: vec![],
	// 		}
	// 	}
	// }

	// #[pallet::genesis_build]
	// impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	// 	fn build(&self) {
	// 		for (validator_id, validator_info) in &self.validators {
	// 			Validators::<T>::insert(validator_id, validator_info);
	// 		}

	// 		for (validator_id, university_id, validated_info) in &self.universities {
	// 			Universities::<T>::insert(validator_id, university_id, validated_info);
	// 		}

	// 		for (validator_id, professor_id, validated_info) in &self.professors {
	// 			Professors::<T>::insert(validator_id, professor_id, validated_info);
	// 		}
	// 	}
	// }
}

impl<T: crate::Config> crate::pallet_provider_traits::ValidationProvider for Pallet<T> {
	fn is_verified_university() -> bool {
		todo!()
	}
}
