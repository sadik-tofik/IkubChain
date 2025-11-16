#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use codec::{Decode, Encode, MaxEncodedLen};
    use sp_runtime::traits::{AccountIdConversion, SaturatedConversion};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Currency type for cross-chain transfers
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    pub type ClubId = u64;
    pub type CrossChainOperationId = u64;
    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum OperationStatus {
        Pending,
        InProgress,
        Completed,
        Failed,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct CrossChainOperation<T: Config> {
        pub id: CrossChainOperationId,
        pub club_id: ClubId,
        pub destination_parachain_id: u32,
        pub operation_type: OperationType,
        pub status: OperationStatus,
        pub amount: Option<BalanceOf<T>>,
        pub created_at: BlockNumberFor<T>,
        pub completed_at: Option<BlockNumberFor<T>>,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum OperationType {
        TransferFunds,
        ExecuteInvestment,
    }

    #[pallet::storage]
    #[pallet::getter(fn operations)]
    pub type CrossChainOperations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        ClubId,
        Blake2_128Concat,
        CrossChainOperationId,
        CrossChainOperation<T>,
        OptionQuery,
    >;

    /// Storage: Operation counter per club
    #[pallet::storage]
    #[pallet::getter(fn operation_count)]
    pub type OperationCount<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        CrossChainOperationId,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CrossChainOperationInitiated {
            club_id: ClubId,
            operation_id: CrossChainOperationId,
            destination_parachain_id: u32,
            operation_type: OperationType,
        },
        CrossChainOperationCompleted {
            club_id: ClubId,
            operation_id: CrossChainOperationId,
        },
        CrossChainOperationFailed {
            club_id: ClubId,
            operation_id: CrossChainOperationId,
        },
        FundsSentToParachain {
            club_id: ClubId,
            destination_parachain_id: u32,
            amount: BalanceOf<T>,
        },
        RemoteInvestmentExecuted {
            club_id: ClubId,
            destination_parachain_id: u32,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        OperationNotFound,
        InvalidOperation,
        InvalidParachainId,
        InsufficientBalance,
        XcmExecutionFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Send funds to another parachain via XCM
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(0)]
        pub fn send_funds_to_parachain(
            origin: OriginFor<T>,
            club_id: ClubId,
            dest_para_id: u32,
            amount: BalanceOf<T>,
            beneficiary: Vec<u8>, // Account ID on destination chain
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            ensure!(
                dest_para_id > 0,
                Error::<T>::InvalidParachainId
            );
            
            // Get treasury account for club (computed from club_id)
            // For MVP, we'll use a simplified approach - in production, integrate with treasury pallet
            use sp_runtime::traits::AccountIdConversion;
            use sp_runtime::ModuleId;
            let module_id = ModuleId(*b"ikubtrsy");
            let treasury_account: T::AccountId = module_id.into_account_truncating(&club_id.encode());
            let balance = T::Currency::free_balance(&treasury_account);
            ensure!(
                balance >= amount,
                Error::<T>::InsufficientBalance
            );
            
            // Create operation record
            let operation_id = Self::operation_count(club_id);
            let new_count = operation_id.saturating_add(1);
            OperationCount::<T>::insert(club_id, new_count);
            
            let now = <frame_system::Pallet<T>>::block_number();
            let operation = CrossChainOperation {
                id: operation_id,
                club_id,
                destination_parachain_id: dest_para_id,
                operation_type: OperationType::TransferFunds,
                status: OperationStatus::Pending,
                amount: Some(amount),
                created_at: now,
                completed_at: None,
            };
            
            CrossChainOperations::<T>::insert(club_id, operation_id, &operation);
            
            // For MVP, we'll simulate the XCM execution
            // In production, this would construct and send proper XCM v3 messages
            // For now, we'll mark it as completed immediately (simulation)
            let mut operation = operation;
            operation.status = OperationStatus::Completed;
            operation.completed_at = Some(now);
            CrossChainOperations::<T>::insert(club_id, operation_id, &operation);
            
            // Transfer funds from treasury (in real implementation, XCM would handle this)
            // For MVP simulation, we'll just deduct from treasury
            T::Currency::withdraw(
                &treasury_account,
                amount,
                frame_support::traits::WithdrawReasons::TRANSFER,
                frame_support::traits::ExistenceRequirement::AllowDeath,
            )?;
            
            Self::deposit_event(Event::CrossChainOperationInitiated {
                club_id,
                operation_id,
                destination_parachain_id: dest_para_id,
                operation_type: OperationType::TransferFunds,
            });
            
            Self::deposit_event(Event::FundsSentToParachain {
                club_id,
                destination_parachain_id: dest_para_id,
                amount,
            });
            
            Self::deposit_event(Event::CrossChainOperationCompleted {
                club_id,
                operation_id,
            });
            
            Ok(())
        }

        /// Execute a remote investment call on another parachain
        #[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
        #[pallet::call_index(1)]
        pub fn execute_remote_investment(
            origin: OriginFor<T>,
            club_id: ClubId,
            dest_para_id: u32,
            call_data: Vec<u8>, // Encoded call to execute on destination
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            ensure!(
                dest_para_id > 0,
                Error::<T>::InvalidParachainId
            );
            
            // Create operation record
            let operation_id = Self::operation_count(club_id);
            let new_count = operation_id.saturating_add(1);
            OperationCount::<T>::insert(club_id, new_count);
            
            let now = <frame_system::Pallet<T>>::block_number();
            let operation = CrossChainOperation {
                id: operation_id,
                club_id,
                destination_parachain_id: dest_para_id,
                operation_type: OperationType::ExecuteInvestment,
                status: OperationStatus::Pending,
                amount: None,
                created_at: now,
                completed_at: None,
            };
            
            CrossChainOperations::<T>::insert(club_id, operation_id, &operation);
            
            // For MVP, we'll simulate the XCM execution
            // In production, this would construct and send proper XCM v3 messages with Transact
            // For now, we'll mark it as completed immediately (simulation)
            let mut operation = operation;
            operation.status = OperationStatus::Completed;
            operation.completed_at = Some(now);
            CrossChainOperations::<T>::insert(club_id, operation_id, &operation);
            
            Self::deposit_event(Event::CrossChainOperationInitiated {
                club_id,
                operation_id,
                destination_parachain_id: dest_para_id,
                operation_type: OperationType::ExecuteInvestment,
            });
            
            Self::deposit_event(Event::RemoteInvestmentExecuted {
                club_id,
                destination_parachain_id: dest_para_id,
            });
            
            Self::deposit_event(Event::CrossChainOperationCompleted {
                club_id,
                operation_id,
            });
            
            Ok(())
        }
    }
}

