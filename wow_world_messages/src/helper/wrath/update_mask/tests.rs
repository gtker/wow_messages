use crate::wrath::update_mask::UpdatePlayer;
use crate::wrath::BuybackSlot;

#[test]
fn buyback_integer_arrays_round_trip_first_and_last_slots() {
    let mut update = UpdatePlayer::builder()
        .set_player_buyback_price(BuybackSlot::Slot1, 0x8000_0001)
        .set_player_buyback_timestamp(BuybackSlot::Slot1, u32::MAX)
        .finalize();

    update.set_player_buyback_price(BuybackSlot::Slot12, 0x8000_1234);
    update.set_player_buyback_timestamp(BuybackSlot::Slot12, 0xFFFF_FFFE);

    assert_eq!(
        update.player_buyback_price(BuybackSlot::Slot1),
        Some(0x8000_0001)
    );
    assert_eq!(
        update.player_buyback_timestamp(BuybackSlot::Slot1),
        Some(u32::MAX)
    );
    assert_eq!(
        update.player_buyback_price(BuybackSlot::Slot12),
        Some(0x8000_1234)
    );
    assert_eq!(
        update.player_buyback_timestamp(BuybackSlot::Slot12),
        Some(0xFFFF_FFFE)
    );
}
