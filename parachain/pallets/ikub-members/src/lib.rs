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

    /// Club information
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Club<T: Config> {
        pub id: ClubId,
        pub name: BoundedVec<u8, ConstU32<256>>,
        pub description: BoundedVec<u8, ConstU32<1024>>,
        pub creator: T::AccountId,
        pub created_at: BlockNumberFor<T>,
        pub is_active: bool,
    }

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

    /// Storage: Clubs
    #[pallet::storage]
    #[pallet::getter(fn clubs)]
    pub type Clubs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        Club<T>,
        OptionQuery,
    >;

    /// Storage: Club counter
    #[pallet::storage]
    #[pallet::getter(fn club_count)]
    pub type ClubCount<T: Config> = StorageValue<_, ClubId, ValueQuery>;

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
        ClubCreated {
            club_id: ClubId,
            creator: T::AccountId,
            name: BoundedVec<u8, ConstU32<256>>,
        },
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
        ClubNotFound,
        InvalidClubName,
        InvalidClubDescription,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new investment club
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(0)]
        pub fn create_club(
            origin: OriginFor<T>,
            name: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            
            ensure!(
                !name.is_empty() && name.len() <= 256,
                Error::<T>::InvalidClubName
            );
            ensure!(
                description.len() <= 1024,
                Error::<T>::InvalidClubDescription
            );
            
            let club_id = Self::club_count();
            let new_count = club_id.saturating_add(1);
            ClubCount::<T>::put(new_count);
            
            let now = <frame_system::Pallet<T>>::block_number();
            let club = Club {
                id: club_id,
                name: BoundedVec::try_from(name.clone())
                    .map_err(|_| Error::<T>::InvalidClubName)?,
                description: BoundedVec::try_from(description)
                    .map_err(|_| Error::<T>::InvalidClubDescription)?,
                creator: creator.clone(),
                created_at: now,
                is_active: true,
            };
            
            Clubs::<T>::insert(club_id, &club);
            
            // Creator automatically joins the club
            let profile = MemberProfile {
                account: creator.clone(),
                club_id,
                joined_at: now,
                reputation: 0,
                contribution_weight: 0,
                voting_participation: 0,
                proposal_success_rate: 0,
            };
            
            Members::<T>::insert(club_id, &creator, &profile);
            MemberCount::<T>::insert(club_id, 1u32);
            
            Self::deposit_event(Event::ClubCreated {
                club_id,
                creator,
                name: club.name.clone(),
            });
            
            Self::deposit_event(Event::MemberJoined {
                club_id,
                account: profile.account,
            });
            
            Ok(())
        }

        /// Join an existing club
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(1)]
        pub fn join_club(
            origin: OriginFor<T>,
            club_id: ClubId,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            
            // Check club exists
            ensure!(
                Clubs::<T>::contains_key(club_id),
                Error::<T>::ClubNotFound
            );
            
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

        /// Leave a club
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(2)]
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

        /// Update member reputation (called by other pallets)
        pub fn update_reputation(
            club_id: ClubId,
            account: &T::AccountId,
            reputation_delta: ReputationScore,
        ) -> DispatchResult {
            if let Some(mut profile) = Members::<T>::get(club_id, account) {
                profile.reputation = profile.reputation.saturating_add(reputation_delta);
                Members::<T>::insert(club_id, account, &profile);
                
                Self::deposit_event(Event::ReputationUpdated {
                    club_id,
                    account: account.clone(),
                    new_reputation: profile.reputation,
                });
            }
            Ok(())
        }
    }
}

