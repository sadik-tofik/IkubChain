#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use codec::{Decode, Encode, MaxEncodedLen};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    pub type ClubId = u64;
    pub type DisputeId = u64;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum DisputeStatus {
        Open,
        InMediation,
        InArbitration,
        Resolved,
        Closed,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub struct Dispute<T: Config> {
        pub id: DisputeId,
        pub club_id: ClubId,
        pub initiator: T::AccountId,
        pub subject: T::AccountId,
        pub description: BoundedVec<u8, ConstU32<1024>>,
        pub status: DisputeStatus,
        pub created_at: BlockNumberFor<T>,
    }

    #[pallet::storage]
    #[pallet::getter(fn disputes)]
    pub type Disputes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ClubId,
        Blake2_128Concat,
        DisputeId,
        Dispute<T>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DisputeCreated {
            club_id: ClubId,
            dispute_id: DisputeId,
            initiator: T::AccountId,
        },
        DisputeResolved {
            club_id: ClubId,
            dispute_id: DisputeId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        DisputeNotFound,
        InvalidDispute,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn create_dispute(
            origin: OriginFor<T>,
            club_id: ClubId,
            subject: T::AccountId,
            description: Vec<u8>,
        ) -> DispatchResult {
            let initiator = ensure_signed(origin)?;
            
            let dispute_id = 0; // Would be generated from counter
            
            let dispute = Dispute {
                id: dispute_id,
                club_id,
                initiator: initiator.clone(),
                subject,
                description: BoundedVec::try_from(description)
                    .map_err(|_| Error::<T>::InvalidDispute)?,
                status: DisputeStatus::Open,
                created_at: <frame_system::Pallet<T>>::block_number(),
            };
            
            Disputes::<T>::insert(club_id, dispute_id, &dispute);
            
            Self::deposit_event(Event::DisputeCreated {
                club_id,
                dispute_id,
                initiator,
            });
            
            Ok(())
        }
    }
}

