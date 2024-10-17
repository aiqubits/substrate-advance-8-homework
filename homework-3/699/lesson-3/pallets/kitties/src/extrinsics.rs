use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        // #[pallet::weight({0})]
        #[pallet::weight(T::WeightInfo::create())]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let value = Self::random_value(&who);
            Self::mint_kitty(&who, value)?;
            Ok(())
        }

        #[pallet::call_index(1)]
        // #[pallet::weight({0})]
        #[pallet::weight(T::WeightInfo::breed())]
        pub fn breed(origin: OriginFor<T>, kitty_1: u32, kitty_2: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(kitty_1 != kitty_2, Error::<T>::SameKittyId);

            if let (Some(kitty1), Some(kitty2)) = (Kitties::<T>::get(kitty_1), Kitties::<T>::get(kitty_2)) {
                let data = Self::breed_kitty(&who, kitty1.0, kitty2.0);
                Self::mint_kitty(&who, data)?;
            }
            
            Ok(())
        }

        #[pallet::call_index(2)]
        // #[pallet::weight({0})]
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>, kitty_id: u32, to: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                !KittiesBid::<T>::contains_key(kitty_id),
                Error::<T>::KittyAlreadyOnSale
            );
            Self::transfer_kitty(who, to, kitty_id)?;
            Ok(())
        }

        #[pallet::call_index(3)]
        // #[pallet::weight({0})]
        #[pallet::weight(T::WeightInfo::sale())]
        pub fn sale(
            origin: OriginFor<T>,
            kitty_id: u32,
            until_block: BlockNumberFor<T>,
            init_amount: BalanceOf<T>
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // 打印变量
             log::info!(
                 "调用 sale 函数，who: {:?}, kitty_id: {:?}, until_block: {:?}, init_amount: {:?}",
                 who,
                 kitty_id,
                 until_block,
                 init_amount
             );
            ensure!(
                until_block >= <system::Pallet<T>>::block_number() + T::MinBidBlockSpan::get(),
                Error::<T>::BlockSpanTooSmall
            );

            Self::sale_kitty(who, kitty_id, until_block, init_amount)?;

            Ok(())
        }

        #[pallet::call_index(4)]
        // #[pallet::weight({0})]
        #[pallet::weight(T::WeightInfo::bid())]
        pub fn bid(origin: OriginFor<T>, kitty_id: u32, price: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::bid_kitty(who, kitty_id, price)?;

            Ok(())
        }
    }
}
