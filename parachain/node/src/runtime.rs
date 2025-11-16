#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    PalletId,
};
use frame_system::{
    limits::{BlockLength, BlockWeights},
    EnsureRoot,
};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, NumberFor, OpaqueKeys},
    transaction_validity::{TransactionSource, TransactionValidity, TransactionValidityError, ValidTransaction},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
pub use sp_runtime::{Perbill, Permill};

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("ikubchain"),
    impl_name: create_runtime_str!("ikubchain-node"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

// Implementations of some helper traits passed into runtime modules
impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as sp_runtime::traits::Verify>::Signer;
    type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    Call: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        _public: <Signature as sp_runtime::traits::Verify>::Signer,
        _account: AccountId,
        _nonce: Index,
    ) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
        None
    }
}

// Parameters for the IkubMembers pallet
parameter_types! {
    pub const MaxMembersPerClub: u32 = 1000;
    
    // Parameters for IkubTreasury pallet
    pub const MaxSigners: u32 = 10;
    pub const MinSignatures: u32 = 2;
    pub const MinContribution: u128 = 1_000_000_000_000; // 1 UNIT (assuming 12 decimals)
    pub const DefaultContributionPeriod: BlockNumber = 7 * 24 * 60 * 10; // 7 days (assuming 6s block time)
    
    // Parameters for IkubGovernance pallet
    pub const MaxProposalsPerClub: u32 = 100;
    pub const MaxVotingDuration: BlockNumber = 7 * 24 * 60 * 10; // 7 days (assuming 6s block time)
    pub const MinProposalDeposit: u128 = 10_000_000_000_000; // 10 UNIT (assuming 12 decimals)
    
    // Parameters for IkubDisputes pallet
    pub const MaxEvidencePerDispute: u32 = 10;
    pub const MaxDisputesPerClub: u32 = 50;
    
    // Parameters for IkubAnalytics pallet
    pub const MaxAnalyticsEvents: u32 = 1000;
    
    // Parameters for IkubCrosschain pallet
    pub const MaxCrossChainOperations: u32 = 100;
    pub const MaxOperationRetries: u32 = 3;
}

// Implement the configuration trait for IkubMembers
impl pallet_ikub_members::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxMembersPerClub = MaxMembersPerClub;
}

// Implement the configuration trait for IkubTreasury
impl pallet_ikub_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxSigners = MaxSigners;
    type MinSignatures = MinSignatures;
    type MinContribution = MinContribution;
    type DefaultContributionPeriod = DefaultContributionPeriod;
}

// Implement the configuration trait for IkubGovernance
impl pallet_ikub_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxProposalsPerClub = MaxProposalsPerClub;
    type MaxVotingDuration = MaxVotingDuration;
    type MinProposalDeposit = MinProposalDeposit;
}

// Implement the configuration trait for IkubDisputes
impl pallet_ikub_disputes::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxEvidencePerDispute = MaxEvidencePerDispute;
    type MaxDisputesPerClub = MaxDisputesPerClub;
}

// Implement the configuration trait for IkubAnalytics
impl pallet_ikub_analytics::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxAnalyticsEvents = MaxAnalyticsEvents;
}

// Implement the configuration trait for IkubCrosschain
impl pallet_ikub_crosschain::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxCrossChainOperations = MaxCrossChainOperations;
    type MaxOperationRetries = MaxOperationRetries;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system,
        RandomnessCollectiveFlip: pallet_randomness_collective_flip,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        
        // Include the custom pallets
        IkubMembers: pallet_ikub_members,
        IkubTreasury: pallet_ikub_treasury,
        IkubGovernance: pallet_ikub_governance,
        IkubDisputes: pallet_ikub_disputes,
        IkubAnalytics: pallet_ikub_analytics,
        IkubCrosschain: pallet_ikub_crosschain,
    }
);

// Implement the runtime API for the node
impl_runtime_apis! {
    // Implementation of the runtime API for the runtime
}
