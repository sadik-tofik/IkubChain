#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, Time},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, Saturating};
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
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Type aliases
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub type ClubId = u64;
    pub type WithdrawalId = u64;

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

