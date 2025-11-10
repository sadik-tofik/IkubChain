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
        #[pallet::constant]
        type MaxMembersPerClub: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    pub type ClubId = u64;
    pub type ReputationScore = u64;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub struct MemberProfile<T: Config> {
        pub account: T::AccountId,
        pub club_id: ClubId,
        pub joined_at: BlockNumberFor<T>,
        pub reputation: ReputationScore,
        pub contribution_weight: u64,
        pub voting_participation: u64,
        pub proposal_success_rate: u8,
    }

    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub type Members<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ClubId,
        Blake2_128Concat,
        T::AccountId,
        MemberProfile<T>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn member_count)]
    pub type MemberCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        u32,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MemberJoined {
            club_id: ClubId,
            account: T::AccountId,
        },
        MemberLeft {
            club_id: ClubId,
            account: T::AccountId,
        },
        ReputationUpdated {
            club_id: ClubId,
            account: T::AccountId,
            new_reputation: ReputationScore,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyMember,
        NotMember,
        MaxMembersReached,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn join_club(
            origin: OriginFor<T>,
            club_id: ClubId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            
            ensure!(
                !Members::<T>::contains_key(club_id, &account),
                Error::<T>::AlreadyMember
            );
            
            let count = Self::member_count(club_id);
            ensure!(
                count < T::MaxMembersPerClub::get(),
                Error::<T>::MaxMembersReached
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            let profile = MemberProfile {
                account: account.clone(),
                club_id,
                joined_at: now,
                reputation: 0,
                contribution_weight: 0,
                voting_participation: 0,
                proposal_success_rate: 0,
            };
            
            Members::<T>::insert(club_id, &account, &profile);
            MemberCount::<T>::mutate(club_id, |c| *c = c.saturating_add(1));
            
            Self::deposit_event(Event::MemberJoined {
                club_id,
                account,
            });
            
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn leave_club(
            origin: OriginFor<T>,
            club_id: ClubId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            
            ensure!(
                Members::<T>::contains_key(club_id, &account),
                Error::<T>::NotMember
            );
            
            Members::<T>::remove(club_id, &account);
            MemberCount::<T>::mutate(club_id, |c| *c = c.saturating_sub(1));
            
            Self::deposit_event(Event::MemberLeft {
                club_id,
                account,
            });
            
            Ok(())
        }
    }
}

