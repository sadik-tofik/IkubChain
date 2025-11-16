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

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub struct PerformanceMetrics {
        pub total_return: i64,
        pub risk_score: u8,
        pub diversification_score: u8,
    }

    /// Analytics event types
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum AnalyticsEvent {
        NewMember,
        NewClub,
        Contribution,
        ProposalCreated,
        CrossChainCall,
        Dispute,
    }

    /// Event counter per club
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub struct EventCounts {
        pub new_members: u64,
        pub new_clubs: u64,
        pub contributions: u64,
        pub proposals_created: u64,
        pub cross_chain_calls: u64,
        pub disputes: u64,
    }

    #[pallet::storage]
    #[pallet::getter(fn metrics)]
    pub type ClubMetrics<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        PerformanceMetrics,
        OptionQuery,
    >;

    /// Storage: Event counts per club
    #[pallet::storage]
    #[pallet::getter(fn event_counts)]
    pub type EventCounts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        EventCounts,
        ValueQuery,
    >;

    /// Storage: Global event counts
    #[pallet::storage]
    #[pallet::getter(fn global_event_counts)]
    pub type GlobalEventCounts<T: Config> = StorageValue<_, EventCounts, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MetricsUpdated {
            club_id: ClubId,
        },
        EventTracked {
            club_id: ClubId,
            event_type: AnalyticsEvent,
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Update performance metrics for a club
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        #[pallet::call_index(0)]
        pub fn update_metrics(
            origin: OriginFor<T>,
            club_id: ClubId,
            metrics: PerformanceMetrics,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            
            ClubMetrics::<T>::insert(club_id, &metrics);
            
            Self::deposit_event(Event::MetricsUpdated { club_id });
            
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Track an analytics event (called by other pallets)
        pub fn track_event(club_id: ClubId, event_type: AnalyticsEvent) {
            // Update club-specific counts
            EventCounts::<T>::mutate(club_id, |counts| {
                match event_type {
                    AnalyticsEvent::NewMember => counts.new_members = counts.new_members.saturating_add(1),
                    AnalyticsEvent::NewClub => counts.new_clubs = counts.new_clubs.saturating_add(1),
                    AnalyticsEvent::Contribution => counts.contributions = counts.contributions.saturating_add(1),
                    AnalyticsEvent::ProposalCreated => counts.proposals_created = counts.proposals_created.saturating_add(1),
                    AnalyticsEvent::CrossChainCall => counts.cross_chain_calls = counts.cross_chain_calls.saturating_add(1),
                    AnalyticsEvent::Dispute => counts.disputes = counts.disputes.saturating_add(1),
                }
            });
            
            // Update global counts
            GlobalEventCounts::<T>::mutate(|counts| {
                match event_type {
                    AnalyticsEvent::NewMember => counts.new_members = counts.new_members.saturating_add(1),
                    AnalyticsEvent::NewClub => counts.new_clubs = counts.new_clubs.saturating_add(1),
                    AnalyticsEvent::Contribution => counts.contributions = counts.contributions.saturating_add(1),
                    AnalyticsEvent::ProposalCreated => counts.proposals_created = counts.proposals_created.saturating_add(1),
                    AnalyticsEvent::CrossChainCall => counts.cross_chain_calls = counts.cross_chain_calls.saturating_add(1),
                    AnalyticsEvent::Dispute => counts.disputes = counts.disputes.saturating_add(1),
                }
            });
        }
    }
}

