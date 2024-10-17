//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use crate::Pallet as PalletKitties;

#[benchmarks]
mod benchmarks {
    use super::*;

    const SEED: u32 = 0;

    #[benchmark]
    fn create(){
        let caller:T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        create(RawOrigin::Signed(caller.clone()));
        assert_eq!(KittyOwner::<T>::get(0),Some(caller.clone()));
        assert_eq!(NextKittyId::<T>::get(),1);
        assert_eq!(Kitties::<T>::get(0).is_some(),true);
    }

    #[benchmark]
    fn breed(){
        let caller:T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        breed(RawOrigin::Signed(caller.clone()),0,1);
    }


    #[benchmark]
    fn transfer(){
        let caller1:T::AccountId = whitelisted_caller();
        let caller2: T::AccountId = account("recipient", 0, SEED);
        let _ = PalletKitties::<T>::create(RawOrigin::Signed(caller1.clone()).into());
        #[extrinsic_call]
        transfer(RawOrigin::Signed(caller1.clone()),0,caller2.clone());

    }

    #[benchmark]
    fn sale(){
        let caller:T::AccountId = whitelisted_caller();
        let _ = PalletKitties::<T>::create(RawOrigin::Signed(caller.clone()).into());
        #[extrinsic_call]
        sale(RawOrigin::Signed(caller.clone()),0,20u32.into(), 20u32.into());
    }

    #[benchmark]
    fn bid(){
        let caller1:T::AccountId = whitelisted_caller();
        let caller2: T::AccountId = account("bidder", 0, SEED);
        let _ = PalletKitties::<T>::create(RawOrigin::Signed(caller1.clone()).into());
        let _ = PalletKitties::<T>::sale(RawOrigin::Signed(caller1.clone()).into(),0,20u32.into(), 20u32.into());
        #[extrinsic_call]
        bid(RawOrigin::Signed(caller2.clone()),0,30u32.into());

    }

    impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
