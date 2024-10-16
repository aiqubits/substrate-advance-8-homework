use frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod config {
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;
        /// A random value generator.
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        type Currency: Currency<Self::AccountId>;

        #[pallet::constant]
        type StakeAmount: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MinBidAmount: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MinBidIncrement: Get<BalanceOf<Self>>;

        #[pallet::constant]
        type MinBidBlockSpan: Get<BlockNumberFor<Self>>;

        #[pallet::constant]
        type MaxKittiesBidPerBlock: Get<u32>;
    }
}
