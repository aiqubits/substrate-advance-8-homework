use super::*;
use crate::{mock::*, Error, Event, Something};
use frame_support::{{assert_noop, assert_ok, BoundedVec}, pallet_prelude::Get};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 16);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	})
}

#[test]
fn revoke_claim_failed_when_claim_non_existent() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_with_owner_non_existent() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

		// let bounded_claim =
		// 	BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_non_existent() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![3, 2, 1]).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_failed_owner_non_existent() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![2, 3, 4, 5]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(10), claim.clone()));

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(PoeModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Something::<Test>::get(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			PoeModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
