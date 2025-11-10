/// Weight information for the runtime.
pub mod weights {
    use frame_support::weights::Weight;

    pub const WEIGHT_REF_TIME_PER_NANOS: Weight = Weight::from_ref_time(1_000_000_000 / 1_000);
}

pub use weights::*;

