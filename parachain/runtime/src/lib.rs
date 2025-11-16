#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;
pub use constants::*;

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, ConstU16, ConstU8, EitherOfDiverse, EqualPrivilege, Nothing, Get},
    weights::Weight,
};
use frame_system::limits::{BlockLength, BlockWeights};
use pallet_transaction_payment::CurrencyAdapter;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

#[cfg(feature = "std")]
use sp_version::NativeVersion;

// Import the pallets
pub use pallet_ikub_governance;
pub use pallet_ikub_treasury;
pub use pallet_ikub_crosschain;
pub use pallet_ikub_members;
pub use pallet_ikub_disputes;
pub use pallet_ikub_analytics;

/// Runtime API versions
pub const RUNTIME_API_VERSIONS: sp_version::RuntimeVersion = sp_version::RuntimeVersion {
    spec_name: create_runtime_str!("ikubchain"),
    impl_name: create_runtime_str!("ikubchain"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    apis: sp_version::create_apis_vec!([]),
    transaction_version: 1,
    state_version: 1,
};

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("ikubchain"),
    impl_name: create_runtime_str!("ikubchain"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type AccountId = AccountId;
    type RuntimeCall = RuntimeCall;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Nonce = Nonce;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    // For MVP standalone mode, use frame_system's default
    // In production parachain, use: cumulus_pallet_parachain_system::ParachainSetCode<Self>
    type OnSetCode = frame_system::DefaultOnSetCode<Self>;
    type MaxConsumers = ConstU32<16>;
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64::<0>;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const MaxProposalsPerClub: u32 = 100;
    pub const MaxVotingDuration: u32 = 100000;
    pub const MinProposalDeposit: u128 = 1000;
}

impl pallet_ikub_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxProposalsPerClub = MaxProposalsPerClub;
    type MaxVotingDuration = ConstU32<MaxVotingDuration>;
    type MinProposalDeposit = ConstU128<MinProposalDeposit>;
}

parameter_types! {
    pub const MaxSigners: u32 = 10;
    pub const MinSignatures: u32 = 3;
    pub const MinContribution: u128 = 1000;
    pub const DefaultContributionPeriod: u32 = 10000; // blocks
}

impl pallet_ikub_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxSigners = MaxSigners;
    type MinSignatures = MinSignatures;
    type MinContribution = ConstU128<MinContribution>;
    type DefaultContributionPeriod = ConstU32<DefaultContributionPeriod>;
}

// For MVP, we'll simplify the crosschain config
// In production, this would need proper XCM configuration
impl pallet_ikub_crosschain::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
}

parameter_types! {
    pub const MaxMembersPerClub: u32 = 1000;
}

impl pallet_ikub_members::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxMembersPerClub = MaxMembersPerClub;
}

impl pallet_ikub_disputes::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_ikub_analytics::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<5>;
    type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = ();
    type LengthToFee = ();
    type FeeMultiplierUpdate = ();
}

// Types
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Hash = sp_core::H256;
pub type Signature = MultiSignature;
pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;

// Runtime types
pub type Block = generic::Block<Header, OpaqueExtrinsic>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type OpaqueExtrinsic = sp_runtime::OpaqueExtrinsic;
pub type OpaqueBlock = generic::Block<OpaqueExtrinsic, OpaqueMetadata>;

// Runtime constants
pub const MILLISECS_PER_BLOCK: u64 = 12000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_SLOTS: u64 = 4 * HOURS;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Weight and database
pub const NORMAL_DISPATCH_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(75);
pub const AVERAGE_ON_INITIALIZE_RATIO: sp_runtime::Perbill = sp_runtime::Perbill::from_percent(10);

parameter_types! {
    pub const BlockExecutionWeight: Weight = Weight::from_parts(5_000_000, 0);
    pub BlockWeights: BlockWeights = BlockWeights::with_sensible_defaults(
        Weight::from_parts(2_000_000_000_000, u64::MAX),
        NORMAL_DISPATCH_RATIO,
    );
    pub BlockLength: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
        read: 25_000 * constants::WEIGHT_REF_TIME_PER_NANOS,
        write: 100_000 * constants::WEIGHT_REF_TIME_PER_NANOS,
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RuntimeDbWeight;
impl frame_support::weights::WeightDatabase for RuntimeDbWeight {
    fn get(&self, _key: &[u8]) -> Option<Weight> {
        None
    }
}

// Construct the runtime
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
        Sudo: pallet_sudo,
        TransactionPayment: pallet_transaction_payment,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        
        // IkubChain Pallets
        IkubGovernance: pallet_ikub_governance,
        IkubTreasury: pallet_ikub_treasury,
        IkubCrosschain: pallet_ikub_crosschain,
        IkubMembers: pallet_ikub_members,
        IkubDisputes: pallet_ikub_disputes,
        IkubAnalytics: pallet_ikub_analytics,
    }
);

// Runtime API
sp_api::impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            SessionKeys::decode(encoded)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration {
            sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities().into_inner()
        }
    }

    impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
        fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
            // For MVP standalone mode, return empty collation info
            // In production, this would use ParachainSystem
            cumulus_primitives_core::CollationInfo {
                upward_messages: vec![],
                horizontal_messages: vec![],
                new_validation_code: None,
                processed_downward_messages: 0,
                hrmp_watermark: 0,
            }
        }
    }
}

// Additional types and implementations would go here
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

pub type AllPalletsWithSystem = (
    IkubGovernance,
    IkubTreasury,
    IkubCrosschain,
    IkubMembers,
    IkubDisputes,
    IkubAnalytics,
    frame_system::Pallet<Runtime>,
    pallet_timestamp::Pallet<Runtime>,
    pallet_balances::Pallet<Runtime>,
    pallet_sudo::Pallet<Runtime>,
    pallet_transaction_payment::Pallet<Runtime>,
    pallet_aura::Pallet<Runtime>,
    pallet_grandpa::Pallet<Runtime>,
);

// Session keys
pub struct SessionKeys;
impl sp_runtime::traits::OpaqueKeys<KeyTypeId> for SessionKeys {
    fn get_raw_keys(&self, _id: KeyTypeId) -> &[Vec<u8>] {
        &[]
    }

    fn keys<C: sp_runtime::traits::IsMember<AuraId>>(&self) -> Vec<C> {
        vec![]
    }
}

impl sp_runtime::traits::BoundToRuntimeAppPublic for SessionKeys {
    type Public = AuraId;
}

impl sp_runtime::traits::IdentifyAccount for SessionKeys {
    type AccountId = AccountId;
    fn into_account(self) -> Self::AccountId {
        Default::default()
    }
}

