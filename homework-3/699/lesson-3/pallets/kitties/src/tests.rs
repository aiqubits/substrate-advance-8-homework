use super::*;
use crate::{mock::*, Error, Event, Kitties, KittyOwner};
use frame_support::{assert_noop, assert_ok};
use frame_system::Config;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn it_works_create_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u32 = 0;
        let caller=<<Test as Config>::RuntimeOrigin>::signed(alice.into());

        assert_ok!(PalletKitties::create(caller));
        assert_eq!(KittyOwner::<Test>::get(alice),Some(alice.into()));
        assert_eq!(NextKittyId::<Test>::get(),1);

        System::assert_has_event(Event::KittyCreated{
            creator: 0,
            index: 0,
            data: Kitties::<Test>::get(0).unwrap().0.clone(),
        }.into(), );
    });
}

#[test]
fn it_kitty_id_overflow() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice=0;
        let caller=<<Test as Config>::RuntimeOrigin>::signed(alice);
        NextKittyId::<Test>::put(u32::MAX);
        assert_noop!(PalletKitties::create(caller),Error::<Test>::NextKittyIdOverflow);
    });
}

#[test]
fn it_works_breed_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));

        run_to_block(3);

        assert_ok!(PalletKitties::breed(RuntimeOrigin::signed(0),0,1));
        assert_eq!(KittyOwner::<Test>::get(2),Some(0));
        assert_eq!(NextKittyId::<Test>::get(),3);


        System::assert_has_event(Event::KittyCreated{
            creator:0,
            index:0,
            data: Kitties::<Test>::get(2).unwrap().0.clone(),
        }.into(), );
    });
}

#[test]
fn it_works_transfer_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);

        assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(0),0,1));
        assert_eq!(KittyOwner::<Test>::get(0),Some(1));


        System::assert_has_event(Event::KittyTransfered{
            from: 0,
            to: 1,
            kitty_id: 0,
        }.into(), );
    });
}

#[test]
fn it_failed_transfer_to_self() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(PalletKitties::transfer(RuntimeOrigin::signed(0),0,0),Error::<Test>::TransferToSelf);
    });
}

#[test]
fn it_failed_transfer_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(PalletKitties::transfer(RuntimeOrigin::signed(1),0,2),Error::<Test>::NotOwner);
    });
}

#[test]
fn it_works_sale_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(PalletKitties::sale(RuntimeOrigin::signed(0),0,100,20));

        assert_eq!(KittyOnSale::<Test>::get(0),Some((100,20)));

        System::assert_has_event(Event::KittyOnSaled{
            owner: 0,
            kitty_id: 0,
            until_block: 100,
        }.into(), );
    });
}

#[test]
fn it_failed_sale_not_owner() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_noop!(PalletKitties::sale(RuntimeOrigin::signed(1),0,100,20), Error::<Test>::NotOwner);
    });
}

#[test]
fn it_works_bid_kitty() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        assert_ok!(PalletKitties::create(RuntimeOrigin::signed(0)));
        run_to_block(2);
        assert_ok!(PalletKitties::sale(RuntimeOrigin::signed(0),0,20,20));
        run_to_block(3);
        assert_ok!(PalletKitties::bid(RuntimeOrigin::signed(1),0,30));
        assert_eq!(KittiesBid::<Test>::get(0),Some((1,30)));
        System::assert_has_event(Event::KittyBid {
            bidder: 1,
            kitty_id:0,
            price: 30,
        }.into(), );
        run_to_block(4);
        assert_ok!(PalletKitties::bid(RuntimeOrigin::signed(2),0,50));
        assert_eq!(KittiesBid::<Test>::get(0),Some((2,50)));
        System::assert_has_event(Event::KittyBid {
            bidder: 2,
            kitty_id:0,
            price: 50,
        }.into(), );
    });
}
