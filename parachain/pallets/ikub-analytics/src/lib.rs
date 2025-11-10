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

    #[pallet::storage]
    #[pallet::getter(fn metrics)]
    pub type ClubMetrics<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        PerformanceMetrics,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MetricsUpdated {
            club_id: ClubId,
        },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
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
}

