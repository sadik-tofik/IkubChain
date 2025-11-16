#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Time},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, Saturating, Zero, SaturatedConversion};
    use scale_info::TypeInfo;
    use codec::{Decode, Encode, MaxEncodedLen};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Currency type
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        
        /// Maximum number of signers for multi-sig
        #[pallet::constant]
        type MaxSigners: Get<u32>;
        
        /// Minimum number of signatures required
        #[pallet::constant]
        type MinSignatures: Get<u32>;
        
        /// Minimum contribution amount per cycle
        #[pallet::constant]
        type MinContribution: Get<BalanceOf<Self>>;
        
        /// Default contribution period in blocks
        #[pallet::constant]
        type DefaultContributionPeriod: Get<BlockNumberFor<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Type aliases
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub type ClubId = u64;
    pub type WithdrawalId = u64;
    pub type ContributionCycleId = u64;

    /// Withdrawal status
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum WithdrawalStatus {
        Pending,
        Approved,
        Executed,
        Cancelled,
    }

    /// Withdrawal request
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct WithdrawalRequest<T: Config> {
        pub id: WithdrawalId,
        pub club_id: ClubId,
        pub recipient: T::AccountId,
        pub amount: BalanceOf<T>,
        pub unlock_at: BlockNumberFor<T>,
        pub status: WithdrawalStatus,
        pub signatures: BoundedVec<T::AccountId, T::MaxSigners>,
        pub created_at: BlockNumberFor<T>,
    }

    /// Contribution cycle status
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum CycleStatus {
        Open,
        Closed,
        Distributed,
    }

    /// Contribution cycle
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct ContributionCycle<T: Config> {
        pub id: ContributionCycleId,
        pub club_id: ClubId,
        pub start_block: BlockNumberFor<T>,
        pub end_block: BlockNumberFor<T>,
        pub total_contributions: BalanceOf<T>,
        pub returns: BalanceOf<T>,
        pub status: CycleStatus,
        pub minimum_contribution: BalanceOf<T>,
    }

    /// Individual contribution record
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Contribution<T: Config> {
        pub contributor: T::AccountId,
        pub amount: BalanceOf<T>,
        pub contributed_at: BlockNumberFor<T>,
    }

    /// Storage: Treasury balances per club
    #[pallet::storage]
    #[pallet::getter(fn treasury_balance)]
    pub type TreasuryBalances<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Storage: Withdrawal requests
    #[pallet::storage]
    #[pallet::getter(fn withdrawal_requests)]
    pub type WithdrawalRequests<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ClubId,
        Blake2_128Concat,
        WithdrawalId,
        WithdrawalRequest<T>,
        OptionQuery,
    >;

    /// Storage: Withdrawal counter
    #[pallet::storage]
    #[pallet::getter(fn withdrawal_count)]
    pub type WithdrawalCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        WithdrawalId,
        ValueQuery,
    >;

    /// Storage: Contribution cycles
    #[pallet::storage]
    #[pallet::getter(fn contribution_cycles)]
    pub type ContributionCycles<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ClubId,
        Blake2_128Concat,
        ContributionCycleId,
        ContributionCycle<T>,
        OptionQuery,
    >;

    /// Storage: Active cycle per club
    #[pallet::storage]
    #[pallet::getter(fn active_cycle)]
    pub type ActiveCycle<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        ContributionCycleId,
        OptionQuery,
    >;

    /// Storage: Cycle counter per club
    #[pallet::storage]
    #[pallet::getter(fn cycle_count)]
    pub type CycleCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        ContributionCycleId,
        ValueQuery,
    >;

    /// Storage: Contributions per cycle
    #[pallet::storage]
    #[pallet::getter(fn contributions)]
    pub type Contributions<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, ClubId>,
            NMapKey<Blake2_128Concat, ContributionCycleId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
        ),
        Contribution<T>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Funds deposited to treasury
        FundsDeposited {
            club_id: ClubId,
            account: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Withdrawal request created
        WithdrawalRequested {
            club_id: ClubId,
            withdrawal_id: WithdrawalId,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Withdrawal approved
        WithdrawalApproved {
            club_id: ClubId,
            withdrawal_id: WithdrawalId,
            approver: T::AccountId,
        },
        /// Withdrawal executed
        WithdrawalExecuted {
            club_id: ClubId,
            withdrawal_id: WithdrawalId,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Contribution cycle opened
        CycleOpened {
            club_id: ClubId,
            cycle_id: ContributionCycleId,
            end_block: BlockNumberFor<T>,
        },
        /// Contribution made to cycle
        ContributionMade {
            club_id: ClubId,
            cycle_id: ContributionCycleId,
            contributor: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Cycle closed
        CycleClosed {
            club_id: ClubId,
            cycle_id: ContributionCycleId,
            total_contributions: BalanceOf<T>,
        },
        /// Returns distributed
        ReturnsDistributed {
            club_id: ClubId,
            cycle_id: ContributionCycleId,
            total_returns: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient balance
        InsufficientBalance,
        /// Withdrawal not found
        WithdrawalNotFound,
        /// Withdrawal not pending
        WithdrawalNotPending,
        /// Already signed
        AlreadySigned,
        /// Insufficient signatures
        InsufficientSignatures,
        /// Withdrawal not ready
        WithdrawalNotReady,
        /// Cycle not found
        CycleNotFound,
        /// Cycle not open
        CycleNotOpen,
        /// Contribution below minimum
        ContributionBelowMinimum,
        /// Cycle not closed
        CycleNotClosed,
        /// No returns to distribute
        NoReturnsToDistribute,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit funds to club treasury
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(0)]
        pub fn deposit(
            origin: OriginFor<T>,
            club_id: ClubId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let account = ensure_signed(origin)?;
            
            T::Currency::transfer(&account, &Self::treasury_account_id(club_id), amount, ExistenceRequirement::KeepAlive)?;
            
            TreasuryBalances::<T>::mutate(club_id, |balance| *balance = balance.saturating_add(amount));
            
            Self::deposit_event(Event::FundsDeposited {
                club_id,
                account,
                amount,
            });
            
            Ok(())
        }

        /// Create a withdrawal request
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(1)]
        pub fn request_withdrawal(
            origin: OriginFor<T>,
            club_id: ClubId,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
            unlock_delay: BlockNumberFor<T>,
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;
            
            let balance = Self::treasury_balance(club_id);
            ensure!(balance >= amount, Error::<T>::InsufficientBalance);
            
            let withdrawal_id = Self::withdrawal_count(club_id);
            let new_count = withdrawal_id.saturating_add(1);
            WithdrawalCount::<T>::insert(club_id, new_count);
            
            let now = <frame_system::Pallet<T>>::block_number();
            let unlock_at = now.saturating_add(unlock_delay);
            
            let mut signatures = BoundedVec::new();
            signatures.try_push(requester.clone())
                .map_err(|_| Error::<T>::InsufficientSignatures)?;
            
            let withdrawal = WithdrawalRequest {
                id: withdrawal_id,
                club_id,
                recipient: recipient.clone(),
                amount,
                unlock_at,
                status: WithdrawalStatus::Pending,
                signatures,
                created_at: now,
            };
            
            WithdrawalRequests::<T>::insert(club_id, withdrawal_id, &withdrawal);
            
            Self::deposit_event(Event::WithdrawalRequested {
                club_id,
                withdrawal_id,
                recipient,
                amount,
            });
            
            Ok(())
        }

        /// Approve a withdrawal request
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(2)]
        pub fn approve_withdrawal(
            origin: OriginFor<T>,
            club_id: ClubId,
            withdrawal_id: WithdrawalId,
        ) -> DispatchResult {
            let approver = ensure_signed(origin)?;
            
            let mut withdrawal = WithdrawalRequests::<T>::get(club_id, withdrawal_id)
                .ok_or(Error::<T>::WithdrawalNotFound)?;
            
            ensure!(
                withdrawal.status == WithdrawalStatus::Pending,
                Error::<T>::WithdrawalNotPending
            );
            
            ensure!(
                !withdrawal.signatures.contains(&approver),
                Error::<T>::AlreadySigned
            );
            
            withdrawal.signatures.try_push(approver.clone())
                .map_err(|_| Error::<T>::InsufficientSignatures)?;
            
            // Check if we have enough signatures
            if withdrawal.signatures.len() >= T::MinSignatures::get() as usize {
                withdrawal.status = WithdrawalStatus::Approved;
            }
            
            WithdrawalRequests::<T>::insert(club_id, withdrawal_id, &withdrawal);
            
            Self::deposit_event(Event::WithdrawalApproved {
                club_id,
                withdrawal_id,
                approver,
            });
            
            Ok(())
        }

        /// Execute an approved withdrawal
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(3)]
        pub fn execute_withdrawal(
            origin: OriginFor<T>,
            club_id: ClubId,
            withdrawal_id: WithdrawalId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            let mut withdrawal = WithdrawalRequests::<T>::get(club_id, withdrawal_id)
                .ok_or(Error::<T>::WithdrawalNotFound)?;
            
            ensure!(
                withdrawal.status == WithdrawalStatus::Approved,
                Error::<T>::WithdrawalNotPending
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(
                now >= withdrawal.unlock_at,
                Error::<T>::WithdrawalNotReady
            );
            
            let balance = Self::treasury_balance(club_id);
            ensure!(balance >= withdrawal.amount, Error::<T>::InsufficientBalance);
            
            // Transfer funds
            T::Currency::transfer(
                &Self::treasury_account_id(club_id),
                &withdrawal.recipient,
                withdrawal.amount,
                ExistenceRequirement::AllowDeath,
            )?;
            
            TreasuryBalances::<T>::mutate(club_id, |balance| *balance = balance.saturating_sub(withdrawal.amount));
            
            withdrawal.status = WithdrawalStatus::Executed;
            WithdrawalRequests::<T>::insert(club_id, withdrawal_id, &withdrawal);
            
            Self::deposit_event(Event::WithdrawalExecuted {
                club_id,
                withdrawal_id,
                recipient: withdrawal.recipient,
                amount: withdrawal.amount,
            });
            
            Ok(())
        }

        /// Open a new contribution cycle
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(4)]
        pub fn open_contribution_cycle(
            origin: OriginFor<T>,
            club_id: ClubId,
            contribution_period: Option<BlockNumberFor<T>>,
            minimum_contribution: Option<BalanceOf<T>>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            // Check no active cycle exists
            ensure!(
                !ActiveCycle::<T>::contains_key(club_id),
                Error::<T>::CycleNotClosed
            );
            
            let cycle_id = Self::cycle_count(club_id);
            let new_count = cycle_id.saturating_add(1);
            CycleCount::<T>::insert(club_id, new_count);
            
            let now = <frame_system::Pallet<T>>::block_number();
            let period = contribution_period.unwrap_or(T::DefaultContributionPeriod::get());
            let min_contrib = minimum_contribution.unwrap_or(T::MinContribution::get());
            let end_block = now.saturating_add(period);
            
            let cycle = ContributionCycle {
                id: cycle_id,
                club_id,
                start_block: now,
                end_block,
                total_contributions: Zero::zero(),
                returns: Zero::zero(),
                status: CycleStatus::Open,
                minimum_contribution: min_contrib,
            };
            
            ContributionCycles::<T>::insert(club_id, cycle_id, &cycle);
            ActiveCycle::<T>::insert(club_id, cycle_id);
            
            Self::deposit_event(Event::CycleOpened {
                club_id,
                cycle_id,
                end_block,
            });
            
            Ok(())
        }

        /// Contribute to the active cycle
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(5)]
        pub fn contribute(
            origin: OriginFor<T>,
            club_id: ClubId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let contributor = ensure_signed(origin)?;
            
            let cycle_id = Self::active_cycle(club_id)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            let mut cycle = Self::contribution_cycles(club_id, cycle_id)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            ensure!(
                cycle.status == CycleStatus::Open,
                Error::<T>::CycleNotOpen
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(
                now <= cycle.end_block,
                Error::<T>::CycleNotOpen
            );
            
            ensure!(
                amount >= cycle.minimum_contribution,
                Error::<T>::ContributionBelowMinimum
            );
            
            // Transfer funds to treasury
            T::Currency::transfer(
                &contributor,
                &Self::treasury_account_id(club_id),
                amount,
                ExistenceRequirement::KeepAlive,
            )?;
            
            // Update or create contribution record
            let existing_contrib = Self::contributions(club_id, cycle_id, &contributor);
            if let Some(mut contrib) = existing_contrib {
                contrib.amount = contrib.amount.saturating_add(amount);
                Contributions::<T>::insert((club_id, cycle_id, &contributor), &contrib);
            } else {
                let contrib = Contribution {
                    contributor: contributor.clone(),
                    amount,
                    contributed_at: now,
                };
                Contributions::<T>::insert((club_id, cycle_id, &contributor), &contrib);
            }
            
            // Update cycle totals
            cycle.total_contributions = cycle.total_contributions.saturating_add(amount);
            TreasuryBalances::<T>::mutate(club_id, |balance| *balance = balance.saturating_add(amount));
            ContributionCycles::<T>::insert(club_id, cycle_id, &cycle);
            
            Self::deposit_event(Event::ContributionMade {
                club_id,
                cycle_id,
                contributor,
                amount,
            });
            
            Ok(())
        }

        /// Close the active contribution cycle
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(6)]
        pub fn close_cycle(
            origin: OriginFor<T>,
            club_id: ClubId,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            let cycle_id = Self::active_cycle(club_id)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            let mut cycle = Self::contribution_cycles(club_id, cycle_id)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            ensure!(
                cycle.status == CycleStatus::Open,
                Error::<T>::CycleNotOpen
            );
            
            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(
                now > cycle.end_block,
                Error::<T>::CycleNotClosed
            );
            
            cycle.status = CycleStatus::Closed;
            ContributionCycles::<T>::insert(club_id, cycle_id, &cycle);
            ActiveCycle::<T>::remove(club_id);
            
            Self::deposit_event(Event::CycleClosed {
                club_id,
                cycle_id,
                total_contributions: cycle.total_contributions,
            });
            
            Ok(())
        }

        /// Set returns for a closed cycle and distribute them
        #[pallet::weight(10_000 + T::DbWeight::get().writes(3))]
        #[pallet::call_index(7)]
        pub fn distribute_returns(
            origin: OriginFor<T>,
            club_id: ClubId,
            cycle_id: ContributionCycleId,
            returns: BalanceOf<T>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            let mut cycle = Self::contribution_cycles(club_id, cycle_id)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            ensure!(
                cycle.status == CycleStatus::Closed,
                Error::<T>::CycleNotClosed
            );
            
            ensure!(
                returns > Zero::zero(),
                Error::<T>::NoReturnsToDistribute
            );
            
            cycle.returns = returns;
            cycle.status = CycleStatus::Distributed;
            ContributionCycles::<T>::insert(club_id, cycle_id, &cycle);
            
            // Distribute returns proportionally: share = (user_contribution / total_contribution) * returns
            if cycle.total_contributions > Zero::zero() {
                // Iterate through all contributions and distribute
                // Note: In production, this would need to be optimized for large numbers of contributors
                // For MVP, we'll use a simplified approach
                let treasury_account = Self::treasury_account_id(club_id);
                let total = cycle.total_contributions;
                
                // Calculate distribution for each contributor
                // This is a simplified version - in production, you'd iterate through all contributions
                // For MVP, we store the returns in the cycle and let users claim them individually
                // via a separate claim function
            }
            
            Self::deposit_event(Event::ReturnsDistributed {
                club_id,
                cycle_id,
                total_returns: returns,
            });
            
            Ok(())
        }

        /// Claim returns from a distributed cycle
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(8)]
        pub fn claim_returns(
            origin: OriginFor<T>,
            club_id: ClubId,
            cycle_id: ContributionCycleId,
        ) -> DispatchResult {
            let claimant = ensure_signed(origin)?;
            
            let cycle = Self::contribution_cycles(club_id, cycle_id)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            ensure!(
                cycle.status == CycleStatus::Distributed,
                Error::<T>::CycleNotClosed
            );
            
            let contribution = Self::contributions(club_id, cycle_id, &claimant)
                .ok_or(Error::<T>::CycleNotFound)?;
            
            // Calculate share: (user_contribution / total_contribution) * returns
            let share = if cycle.total_contributions > Zero::zero() {
                // Use fixed point arithmetic: share = (contribution * returns) / total
                // For u128, we can use checked arithmetic
                let contrib_u128: u128 = contribution.amount.saturated_into();
                let returns_u128: u128 = cycle.returns.saturated_into();
                let total_u128: u128 = cycle.total_contributions.saturated_into();
                
                let share_u128 = contrib_u128
                    .checked_mul(returns_u128)
                    .and_then(|x| x.checked_div(total_u128))
                    .unwrap_or(0);
                
                BalanceOf::<T>::saturated_from(share_u128)
            } else {
                Zero::zero()
            };
            
            ensure!(
                share > Zero::zero(),
                Error::<T>::NoReturnsToDistribute
            );
            
            // Transfer share from treasury
            T::Currency::transfer(
                &Self::treasury_account_id(club_id),
                &claimant,
                share,
                ExistenceRequirement::AllowDeath,
            )?;
            
            TreasuryBalances::<T>::mutate(club_id, |balance| *balance = balance.saturating_sub(share));
            
            Ok(())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Auto-close expired cycles
            // This would iterate through active cycles and close expired ones
            // For MVP, we'll rely on manual closing
            Weight::zero()
        }
    }

    impl<T: Config> Pallet<T> {
        /// Generate treasury account ID for a club
        pub fn treasury_account_id(club_id: ClubId) -> T::AccountId {
            use sp_runtime::traits::AccountIdConversion;
            use sp_runtime::ModuleId;
            
            // Create a deterministic account from module ID and club ID
            let module_id = ModuleId(*b"ikubtrsy");
            module_id.into_account_truncating(&club_id.encode())
        }
    }
}

