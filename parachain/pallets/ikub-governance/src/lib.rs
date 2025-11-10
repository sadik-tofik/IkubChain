#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, Saturating};
    use scale_info::TypeInfo;
    use codec::{Decode, Encode, MaxEncodedLen};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// The currency type for deposits and fees
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        
        /// Maximum number of proposals per club
        #[pallet::constant]
        type MaxProposalsPerClub: Get<u32>;
        
        /// Maximum voting duration in blocks
        #[pallet::constant]
        type MaxVotingDuration: Get<BlockNumberFor<Self>>;
        
        /// Minimum proposal deposit
        #[pallet::constant]
        type MinProposalDeposit: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Type alias for balance
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    
    /// Type alias for club ID
    pub type ClubId = u64;
    
    /// Type alias for proposal ID
    pub type ProposalId = u64;

    /// Proposal types
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub enum ProposalType {
        /// Investment proposal requiring due diligence
        Investment,
        /// Operational proposal for club management
        Operational,
        /// Emergency proposal for rapid response
        Emergency,
        /// Constitutional amendment for fundamental rule changes
        Constitutional,
    }

    /// Voting mechanism types
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub enum VotingMechanism {
        /// Simple majority voting
        SimpleMajority,
        /// Quadratic voting to prevent whale dominance
        Quadratic,
        /// Conviction voting for long-term alignment
        Conviction,
        /// Delegated voting to experts
        Delegated,
    }

    /// Vote choice
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub enum VoteChoice {
        /// Vote in favor
        Aye,
        /// Vote against
        Nay,
        /// Abstain
        Abstain,
    }

    /// Proposal status
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub enum ProposalStatus {
        /// Proposal is active and accepting votes
        Active,
        /// Proposal has passed
        Passed,
        /// Proposal has been rejected
        Rejected,
        /// Proposal has expired
        Expired,
        /// Proposal has been cancelled
        Cancelled,
    }

    /// Proposal structure
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Proposal<T: Config> {
        /// Unique proposal ID
        pub id: ProposalId,
        /// Club this proposal belongs to
        pub club_id: ClubId,
        /// Proposal creator
        pub proposer: T::AccountId,
        /// Proposal type
        pub proposal_type: ProposalType,
        /// Voting mechanism to use
        pub voting_mechanism: VotingMechanism,
        /// Proposal title
        pub title: BoundedVec<u8, ConstU32<256>>,
        /// Proposal description
        pub description: BoundedVec<u8, ConstU32<4096>>,
        /// Deposit amount
        pub deposit: BalanceOf<T>,
        /// Block number when proposal was created
        pub created_at: BlockNumberFor<T>,
        /// Block number when voting ends
        pub voting_end: BlockNumberFor<T>,
        /// Current status
        pub status: ProposalStatus,
        /// Aye votes (weighted)
        pub aye_votes: BalanceOf<T>,
        /// Nay votes (weighted)
        pub nay_votes: BalanceOf<T>,
        /// Abstain votes (weighted)
        pub abstain_votes: BalanceOf<T>,
        /// Approval threshold (as percentage)
        pub approval_threshold: u8,
    }

    /// Vote record
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Vote<T: Config> {
        /// Voter account
        pub voter: T::AccountId,
        /// Vote choice
        pub choice: VoteChoice,
        /// Voting power (weighted)
        pub power: BalanceOf<T>,
        /// Block number when vote was cast
        pub cast_at: BlockNumberFor<T>,
    }

    /// Storage: Active proposals by club
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ClubId,
        Blake2_128Concat,
        ProposalId,
        Proposal<T>,
        OptionQuery,
    >;

    /// Storage: Votes for each proposal
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, ClubId>,
            NMapKey<Blake2_128Concat, ProposalId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
        ),
        Vote<T>,
        OptionQuery,
    >;

    /// Storage: Proposal counter per club
    #[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub type ProposalCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        ProposalId,
        ValueQuery,
    >;

    /// Storage: Active proposal IDs per club
    #[pallet::storage]
    #[pallet::getter(fn active_proposals)]
    pub type ActiveProposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        BoundedVec<ProposalId, T::MaxProposalsPerClub>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new proposal was created
        ProposalCreated {
            club_id: ClubId,
            proposal_id: ProposalId,
            proposer: T::AccountId,
            proposal_type: ProposalType,
        },
        /// A vote was cast
        VoteCast {
            club_id: ClubId,
            proposal_id: ProposalId,
            voter: T::AccountId,
            choice: VoteChoice,
            power: BalanceOf<T>,
        },
        /// A proposal passed
        ProposalPassed {
            club_id: ClubId,
            proposal_id: ProposalId,
        },
        /// A proposal was rejected
        ProposalRejected {
            club_id: ClubId,
            proposal_id: ProposalId,
        },
        /// A proposal expired
        ProposalExpired {
            club_id: ClubId,
            proposal_id: ProposalId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Proposal not found
        ProposalNotFound,
        /// Proposal is not in active state
        ProposalNotActive,
        /// Voting period has ended
        VotingPeriodEnded,
        /// Insufficient deposit
        InsufficientDeposit,
        /// Maximum proposals per club exceeded
        MaxProposalsExceeded,
        /// Invalid approval threshold
        InvalidApprovalThreshold,
        /// Already voted
        AlreadyVoted,
        /// Invalid voting duration
        InvalidVotingDuration,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Check for expired proposals
            // This would be implemented to check and expire proposals
            Weight::zero()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new proposal
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(0)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            club_id: ClubId,
            proposal_type: ProposalType,
            voting_mechanism: VotingMechanism,
            title: Vec<u8>,
            description: Vec<u8>,
            voting_duration: BlockNumberFor<T>,
            approval_threshold: u8,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;
            
            // Validate voting duration
            ensure!(
                voting_duration <= T::MaxVotingDuration::get(),
                Error::<T>::InvalidVotingDuration
            );
            
            // Validate approval threshold
            ensure!(
                approval_threshold > 0 && approval_threshold <= 100,
                Error::<T>::InvalidApprovalThreshold
            );
            
            // Check deposit requirement
            let deposit = T::MinProposalDeposit::get();
            T::Currency::reserve(&proposer, deposit)?;
            
            // Get next proposal ID
            let proposal_id = Self::proposal_count(club_id);
            let new_count = proposal_id.saturating_add(1);
            ProposalCount::<T>::insert(club_id, new_count);
            
            // Check max proposals limit
            let mut active = Self::active_proposals(club_id);
            ensure!(
                active.len() < T::MaxProposalsPerClub::get() as usize,
                Error::<T>::MaxProposalsExceeded
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            let voting_end = now.saturating_add(voting_duration);
            
            let proposal = Proposal {
                id: proposal_id,
                club_id,
                proposer: proposer.clone(),
                proposal_type: proposal_type.clone(),
                voting_mechanism,
                title: BoundedVec::try_from(title)
                    .map_err(|_| Error::<T>::MaxProposalsExceeded)?,
                description: BoundedVec::try_from(description)
                    .map_err(|_| Error::<T>::MaxProposalsExceeded)?,
                deposit,
                created_at: now,
                voting_end,
                status: ProposalStatus::Active,
                aye_votes: Zero::zero(),
                nay_votes: Zero::zero(),
                abstain_votes: Zero::zero(),
                approval_threshold,
            };
            
            Proposals::<T>::insert(club_id, proposal_id, &proposal);
            active.try_push(proposal_id)
                .map_err(|_| Error::<T>::MaxProposalsExceeded)?;
            ActiveProposals::<T>::insert(club_id, active);
            
            Self::deposit_event(Event::ProposalCreated {
                club_id,
                proposal_id,
                proposer,
                proposal_type,
            });
            
            Ok(())
        }

        /// Cast a vote on a proposal
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(1)]
        pub fn vote(
            origin: OriginFor<T>,
            club_id: ClubId,
            proposal_id: ProposalId,
            choice: VoteChoice,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;
            
            let mut proposal = Proposals::<T>::get(club_id, proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;
            
            ensure!(
                proposal.status == ProposalStatus::Active,
                Error::<T>::ProposalNotActive
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(
                now <= proposal.voting_end,
                Error::<T>::VotingPeriodEnded
            );
            
            // Check if already voted
            ensure!(
                !Votes::<T>::contains_key((club_id, proposal_id, &voter)),
                Error::<T>::AlreadyVoted
            );
            
            // Get voting power (simplified - would integrate with members pallet)
            let voting_power = T::Currency::free_balance(&voter);
            
            // Apply voting mechanism (simplified - full implementation would handle quadratic, conviction, etc.)
            let weighted_power = match proposal.voting_mechanism {
                VotingMechanism::SimpleMajority => voting_power,
                VotingMechanism::Quadratic => {
                    // Simplified quadratic voting - sqrt of balance
                    // Full implementation would use fixed point math
                    voting_power
                },
                VotingMechanism::Conviction => voting_power,
                VotingMechanism::Delegated => voting_power,
            };
            
            let vote = Vote {
                voter: voter.clone(),
                choice: choice.clone(),
                power: weighted_power,
                cast_at: now,
            };
            
            Votes::<T>::insert((club_id, proposal_id, &voter), &vote);
            
            // Update proposal vote counts
            match choice {
                VoteChoice::Aye => proposal.aye_votes = proposal.aye_votes.saturating_add(weighted_power),
                VoteChoice::Nay => proposal.nay_votes = proposal.nay_votes.saturating_add(weighted_power),
                VoteChoice::Abstain => proposal.abstain_votes = proposal.abstain_votes.saturating_add(weighted_power),
            }
            
            Proposals::<T>::insert(club_id, proposal_id, &proposal);
            
            Self::deposit_event(Event::VoteCast {
                club_id,
                proposal_id,
                voter,
                choice,
                power: weighted_power,
            });
            
            Ok(())
        }

        /// Finalize a proposal and check if it passed
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(2)]
        pub fn finalize_proposal(
            origin: OriginFor<T>,
            club_id: ClubId,
            proposal_id: ProposalId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            let mut proposal = Proposals::<T>::get(club_id, proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;
            
            ensure!(
                proposal.status == ProposalStatus::Active,
                Error::<T>::ProposalNotActive
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(
                now > proposal.voting_end,
                Error::<T>::VotingPeriodEnded
            );
            
            let total_votes = proposal.aye_votes
                .saturating_add(proposal.nay_votes)
                .saturating_add(proposal.abstain_votes);
            
            if total_votes.is_zero() {
                proposal.status = ProposalStatus::Expired;
                Proposals::<T>::insert(club_id, proposal_id, &proposal);
                Self::deposit_event(Event::ProposalExpired { club_id, proposal_id });
                return Ok(());
            }
            
            // Calculate approval percentage
            // Simplified calculation - full implementation would use proper fixed point math
            // For now, we compare aye_votes to total_votes (excluding abstains)
            let voting_votes = proposal.aye_votes.saturating_add(proposal.nay_votes);
            let passed = if voting_votes.is_zero() {
                false
            } else {
                // Check if aye votes are greater than nay votes and meet threshold
                // This is a simplified check - full implementation would calculate percentage properly
                proposal.aye_votes > proposal.nay_votes
            };
            
            // Check if proposal passed
            if passed {
                proposal.status = ProposalStatus::Passed;
                Proposals::<T>::insert(club_id, proposal_id, &proposal);
                Self::deposit_event(Event::ProposalPassed { club_id, proposal_id });
            } else {
                proposal.status = ProposalStatus::Rejected;
                Proposals::<T>::insert(club_id, proposal_id, &proposal);
                Self::deposit_event(Event::ProposalRejected { club_id, proposal_id });
            }
            
            // Remove from active proposals
            let mut active = Self::active_proposals(club_id);
            active.retain(|&id| id != proposal_id);
            ActiveProposals::<T>::insert(club_id, active);
            
            Ok(())
        }
    }
}

