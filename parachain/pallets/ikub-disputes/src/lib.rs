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
    pub enum VoteChoice {
        FavorInitiator,
        FavorSubject,
        Abstain,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Dispute<T: Config> {
        pub id: DisputeId,
        pub club_id: ClubId,
        pub initiator: T::AccountId,
        pub subject: T::AccountId,
        pub description: BoundedVec<u8, ConstU32<1024>>,
        pub status: DisputeStatus,
        pub created_at: BlockNumberFor<T>,
        pub resolved_at: Option<BlockNumberFor<T>>,
        pub favor_initiator_votes: u32,
        pub favor_subject_votes: u32,
        pub abstain_votes: u32,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Evidence<T: Config> {
        pub submitter: T::AccountId,
        pub description: BoundedVec<u8, ConstU32<2048>>,
        pub submitted_at: BlockNumberFor<T>,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct DisputeVote<T: Config> {
        pub voter: T::AccountId,
        pub choice: VoteChoice,
        pub cast_at: BlockNumberFor<T>,
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

    /// Storage: Dispute counter per club
    #[pallet::storage]
    #[pallet::getter(fn dispute_count)]
    pub type DisputeCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        DisputeId,
        ValueQuery,
    >;

    /// Storage: Evidence for disputes
    #[pallet::storage]
    #[pallet::getter(fn evidence)]
    pub type Evidence<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, ClubId>,
            NMapKey<Blake2_128Concat, DisputeId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
        ),
        Evidence<T>,
        OptionQuery,
    >;

    /// Storage: Votes on disputes
    #[pallet::storage]
    #[pallet::getter(fn dispute_votes)]
    pub type DisputeVotes<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, ClubId>,
            NMapKey<Blake2_128Concat, DisputeId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
        ),
        DisputeVote<T>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DisputeCreated {
            club_id: ClubId,
            dispute_id: DisputeId,
            initiator: T::AccountId,
            subject: T::AccountId,
        },
        EvidenceSubmitted {
            club_id: ClubId,
            dispute_id: DisputeId,
            submitter: T::AccountId,
        },
        DisputeVoted {
            club_id: ClubId,
            dispute_id: DisputeId,
            voter: T::AccountId,
            choice: VoteChoice,
        },
        DisputeResolved {
            club_id: ClubId,
            dispute_id: DisputeId,
            winner: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        DisputeNotFound,
        InvalidDispute,
        DisputeNotOpen,
        AlreadyVoted,
        EvidenceAlreadySubmitted,
        DisputeNotResolvable,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Open a new dispute
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(0)]
        pub fn open_dispute(
            origin: OriginFor<T>,
            club_id: ClubId,
            subject: T::AccountId,
            description: Vec<u8>,
        ) -> DispatchResult {
            let initiator = ensure_signed(origin)?;
            
            let dispute_id = Self::dispute_count(club_id);
            let new_count = dispute_id.saturating_add(1);
            DisputeCount::<T>::insert(club_id, new_count);
            
            let now = <frame_system::Pallet<T>>::block_number();
            let dispute = Dispute {
                id: dispute_id,
                club_id,
                initiator: initiator.clone(),
                subject: subject.clone(),
                description: BoundedVec::try_from(description)
                    .map_err(|_| Error::<T>::InvalidDispute)?,
                status: DisputeStatus::Open,
                created_at: now,
                resolved_at: None,
                favor_initiator_votes: 0,
                favor_subject_votes: 0,
                abstain_votes: 0,
            };
            
            Disputes::<T>::insert(club_id, dispute_id, &dispute);
            
            Self::deposit_event(Event::DisputeCreated {
                club_id,
                dispute_id,
                initiator,
                subject,
            });
            
            Ok(())
        }

        /// Submit evidence for a dispute
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(1)]
        pub fn submit_evidence(
            origin: OriginFor<T>,
            club_id: ClubId,
            dispute_id: DisputeId,
            evidence_description: Vec<u8>,
        ) -> DispatchResult {
            let submitter = ensure_signed(origin)?;
            
            let dispute = Self::disputes(club_id, dispute_id)
                .ok_or(Error::<T>::DisputeNotFound)?;
            
            ensure!(
                dispute.status == DisputeStatus::Open || dispute.status == DisputeStatus::InMediation,
                Error::<T>::DisputeNotOpen
            );
            
            ensure!(
                !Evidence::<T>::contains_key((club_id, dispute_id, &submitter)),
                Error::<T>::EvidenceAlreadySubmitted
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            let evidence = Evidence {
                submitter: submitter.clone(),
                description: BoundedVec::try_from(evidence_description)
                    .map_err(|_| Error::<T>::InvalidDispute)?,
                submitted_at: now,
            };
            
            Evidence::<T>::insert((club_id, dispute_id, &submitter), &evidence);
            
            Self::deposit_event(Event::EvidenceSubmitted {
                club_id,
                dispute_id,
                submitter,
            });
            
            Ok(())
        }

        /// Vote on a dispute
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(2)]
        pub fn vote_on_dispute(
            origin: OriginFor<T>,
            club_id: ClubId,
            dispute_id: DisputeId,
            choice: VoteChoice,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            
            let mut dispute = Self::disputes(club_id, dispute_id)
                .ok_or(Error::<T>::DisputeNotFound)?;
            
            ensure!(
                dispute.status == DisputeStatus::Open || dispute.status == DisputeStatus::InArbitration,
                Error::<T>::DisputeNotOpen
            );
            
            ensure!(
                !DisputeVotes::<T>::contains_key((club_id, dispute_id, &voter)),
                Error::<T>::AlreadyVoted
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            let vote = DisputeVote {
                voter: voter.clone(),
                choice: choice.clone(),
                cast_at: now,
            };
            
            DisputeVotes::<T>::insert((club_id, dispute_id, &voter), &vote);
            
            // Update vote counts
            match choice {
                VoteChoice::FavorInitiator => dispute.favor_initiator_votes += 1,
                VoteChoice::FavorSubject => dispute.favor_subject_votes += 1,
                VoteChoice::Abstain => dispute.abstain_votes += 1,
            }
            
            Disputes::<T>::insert(club_id, dispute_id, &dispute);
            
            Self::deposit_event(Event::DisputeVoted {
                club_id,
                dispute_id,
                voter,
                choice,
            });
            
            Ok(())
        }

        /// Resolve a dispute based on votes
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(3)]
        pub fn resolve_dispute(
            origin: OriginFor<T>,
            club_id: ClubId,
            dispute_id: DisputeId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            let mut dispute = Self::disputes(club_id, dispute_id)
                .ok_or(Error::<T>::DisputeNotFound)?;
            
            ensure!(
                dispute.status == DisputeStatus::Open || dispute.status == DisputeStatus::InArbitration,
                Error::<T>::DisputeNotOpen
            );
            
            // Determine winner based on votes
            let total_votes = dispute.favor_initiator_votes
                .saturating_add(dispute.favor_subject_votes)
                .saturating_add(dispute.abstain_votes);
            
            ensure!(
                total_votes > 0,
                Error::<T>::DisputeNotResolvable
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            let winner = if dispute.favor_initiator_votes > dispute.favor_subject_votes {
                dispute.initiator.clone()
            } else if dispute.favor_subject_votes > dispute.favor_initiator_votes {
                dispute.subject.clone()
            } else {
                // Tie - favor subject (could be configurable)
                dispute.subject.clone()
            };
            
            dispute.status = DisputeStatus::Resolved;
            dispute.resolved_at = Some(now);
            Disputes::<T>::insert(club_id, dispute_id, &dispute);
            
            Self::deposit_event(Event::DisputeResolved {
                club_id,
                dispute_id,
                winner,
            });
            
            Ok(())
        }
    }
}

