use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::tbc::InventoryResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm#L18):
/// ```text
/// smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
///     InventoryResult result;
///     if (result != OK) {
///         Guid item1;
///         Guid item2;
///         u8 bag_type_subclass;
///     }
///     if (result == CANT_EQUIP_LEVEL_I) {
///         Level32 required_level;
///     }
/// }
/// ```
pub struct SMSG_INVENTORY_CHANGE_FAILURE {
    pub result: SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult,
}

impl crate::private::Sealed for SMSG_INVENTORY_CHANGE_FAILURE {}
impl SMSG_INVENTORY_CHANGE_FAILURE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=22).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        let mut result_if_item1 = Default::default();
        let mut result_if_item2 = Default::default();
        let mut result_if_bag_type_subclass = Default::default();
        let mut result_if_required_level = Default::default();

        // result: InventoryResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        match result {
            InventoryResult::Ok => {}
            InventoryResult::CantEquipLevelI => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantEquipSkill => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemDoesntGoToSlot => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BagFull => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NonemptyBagOverOtherBag => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantTradeEquipBags => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::OnlyAmmoCanGoHere => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NoRequiredProficiency => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NoEquipmentSlotAvailable => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::YouCanNeverUseThatItem => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::YouCanNeverUseThatItem2 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantEquipWithTwohanded => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantDualWield => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemDoesntGoIntoBag => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantCarryMoreOfThis => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemCantStack => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemCantBeEquipped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemsCantBeSwapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::SlotIsEmpty => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemNotFound => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantDropSoulbound => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::OutOfRange => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::TriedToSplitMoreThanCount => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CouldntSplitItems => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::MissingReagent => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotEnoughMoney => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotABag => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::DontOwnThatItem => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CanEquipOnly1Quiver => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::MustPurchaseThatBagSlot => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::TooFarAwayFromBank => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemLocked => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::YouAreStunned => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::YouAreDead => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantDoRightNow => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::IntBagError => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CanEquipOnly1Bolt => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CanEquipOnly1Ammopouch => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::StackableCantBeWrapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::EquippedCantBeWrapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::WrappedCantBeWrapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BoundCantBeWrapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::UniqueCantBeWrapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BagsCantBeWrapped => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::AlreadyLooted => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::InventoryFull => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BankFull => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemIsCurrentlySoldOut => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BagFull3 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemNotFound2 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemCantStack2 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BagFull4 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemSoldOut => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ObjectIsBusy => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::None => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotInCombat => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotWhileDisarmed => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BagFull6 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantEquipRank => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CantEquipReputation => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::TooManySpecialBags => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::LootCantLootThatNow => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemUniqueEquipable => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::VendorMissingTurnins => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotEnoughHonorPoints => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotEnoughArenaPoints => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemMaxCountSocketed => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::MailBoundItem => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NoSplitWhileProspecting => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::BagFull7 => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemMaxCountEquippedSocketed => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::ItemUniqueEquippableSocketed => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::TooMuchGold => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::NotDuringArenaMatch => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::CannotTradeThat => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
            InventoryResult::PersonalArenaRatingTooLow => {
                // item1: Guid
                result_if_item1 = crate::util::read_guid(&mut r)?;

                // item2: Guid
                result_if_item2 = crate::util::read_guid(&mut r)?;

                // bag_type_subclass: u8
                result_if_bag_type_subclass = crate::util::read_u8_le(&mut r)?;

            }
        };

        match result {
            InventoryResult::Ok => {}
            InventoryResult::CantEquipLevelI => {
                // required_level: Level32
                result_if_required_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

            }
            InventoryResult::CantEquipSkill => {
            }
            InventoryResult::ItemDoesntGoToSlot => {
            }
            InventoryResult::BagFull => {
            }
            InventoryResult::NonemptyBagOverOtherBag => {
            }
            InventoryResult::CantTradeEquipBags => {
            }
            InventoryResult::OnlyAmmoCanGoHere => {
            }
            InventoryResult::NoRequiredProficiency => {
            }
            InventoryResult::NoEquipmentSlotAvailable => {
            }
            InventoryResult::YouCanNeverUseThatItem => {
            }
            InventoryResult::YouCanNeverUseThatItem2 => {
            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
            }
            InventoryResult::CantEquipWithTwohanded => {
            }
            InventoryResult::CantDualWield => {
            }
            InventoryResult::ItemDoesntGoIntoBag => {
            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
            }
            InventoryResult::CantCarryMoreOfThis => {
            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
            }
            InventoryResult::ItemCantStack => {
            }
            InventoryResult::ItemCantBeEquipped => {
            }
            InventoryResult::ItemsCantBeSwapped => {
            }
            InventoryResult::SlotIsEmpty => {
            }
            InventoryResult::ItemNotFound => {
            }
            InventoryResult::CantDropSoulbound => {
            }
            InventoryResult::OutOfRange => {
            }
            InventoryResult::TriedToSplitMoreThanCount => {
            }
            InventoryResult::CouldntSplitItems => {
            }
            InventoryResult::MissingReagent => {
            }
            InventoryResult::NotEnoughMoney => {
            }
            InventoryResult::NotABag => {
            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
            }
            InventoryResult::DontOwnThatItem => {
            }
            InventoryResult::CanEquipOnly1Quiver => {
            }
            InventoryResult::MustPurchaseThatBagSlot => {
            }
            InventoryResult::TooFarAwayFromBank => {
            }
            InventoryResult::ItemLocked => {
            }
            InventoryResult::YouAreStunned => {
            }
            InventoryResult::YouAreDead => {
            }
            InventoryResult::CantDoRightNow => {
            }
            InventoryResult::IntBagError => {
            }
            InventoryResult::CanEquipOnly1Bolt => {
            }
            InventoryResult::CanEquipOnly1Ammopouch => {
            }
            InventoryResult::StackableCantBeWrapped => {
            }
            InventoryResult::EquippedCantBeWrapped => {
            }
            InventoryResult::WrappedCantBeWrapped => {
            }
            InventoryResult::BoundCantBeWrapped => {
            }
            InventoryResult::UniqueCantBeWrapped => {
            }
            InventoryResult::BagsCantBeWrapped => {
            }
            InventoryResult::AlreadyLooted => {
            }
            InventoryResult::InventoryFull => {
            }
            InventoryResult::BankFull => {
            }
            InventoryResult::ItemIsCurrentlySoldOut => {
            }
            InventoryResult::BagFull3 => {
            }
            InventoryResult::ItemNotFound2 => {
            }
            InventoryResult::ItemCantStack2 => {
            }
            InventoryResult::BagFull4 => {
            }
            InventoryResult::ItemSoldOut => {
            }
            InventoryResult::ObjectIsBusy => {
            }
            InventoryResult::None => {
            }
            InventoryResult::NotInCombat => {
            }
            InventoryResult::NotWhileDisarmed => {
            }
            InventoryResult::BagFull6 => {
            }
            InventoryResult::CantEquipRank => {
            }
            InventoryResult::CantEquipReputation => {
            }
            InventoryResult::TooManySpecialBags => {
            }
            InventoryResult::LootCantLootThatNow => {
            }
            InventoryResult::ItemUniqueEquipable => {
            }
            InventoryResult::VendorMissingTurnins => {
            }
            InventoryResult::NotEnoughHonorPoints => {
            }
            InventoryResult::NotEnoughArenaPoints => {
            }
            InventoryResult::ItemMaxCountSocketed => {
            }
            InventoryResult::MailBoundItem => {
            }
            InventoryResult::NoSplitWhileProspecting => {
            }
            InventoryResult::BagFull7 => {
            }
            InventoryResult::ItemMaxCountEquippedSocketed => {
            }
            InventoryResult::ItemUniqueEquippableSocketed => {
            }
            InventoryResult::TooMuchGold => {
            }
            InventoryResult::NotDuringArenaMatch => {
            }
            InventoryResult::CannotTradeThat => {
            }
            InventoryResult::PersonalArenaRatingTooLow => {
            }
        };

        let result_if = match result {
            InventoryResult::Ok => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok {
                }
            }
            InventoryResult::CantEquipLevelI => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                    required_level: result_if_required_level,
                }
            }
            InventoryResult::CantEquipSkill => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemDoesntGoToSlot => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BagFull => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NonemptyBagOverOtherBag => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantTradeEquipBags => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::OnlyAmmoCanGoHere => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NoRequiredProficiency => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::YouCanNeverUseThatItem => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::YouCanNeverUseThatItem2 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantEquipWithTwohanded => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantDualWield => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantCarryMoreOfThis => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemCantStack => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemCantBeEquipped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemsCantBeSwapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::SlotIsEmpty => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemNotFound => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantDropSoulbound => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::OutOfRange => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::TriedToSplitMoreThanCount => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CouldntSplitItems => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::MissingReagent => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotEnoughMoney => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotABag => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::DontOwnThatItem => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CanEquipOnly1Quiver => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::MustPurchaseThatBagSlot => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::TooFarAwayFromBank => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemLocked => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::YouAreStunned => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::YouAreDead => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantDoRightNow => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::IntBagError => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CanEquipOnly1Bolt => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CanEquipOnly1Ammopouch => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::StackableCantBeWrapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::EquippedCantBeWrapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::WrappedCantBeWrapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BoundCantBeWrapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::UniqueCantBeWrapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BagsCantBeWrapped => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::AlreadyLooted => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::InventoryFull => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BankFull => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemIsCurrentlySoldOut => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BagFull3 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemNotFound2 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemCantStack2 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BagFull4 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemSoldOut => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ObjectIsBusy => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::None => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotInCombat => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotWhileDisarmed => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BagFull6 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantEquipRank => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CantEquipReputation => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::TooManySpecialBags => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::LootCantLootThatNow => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemUniqueEquipable => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::VendorMissingTurnins => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotEnoughHonorPoints => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotEnoughArenaPoints => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemMaxCountSocketed => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::MailBoundItem => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NoSplitWhileProspecting => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::BagFull7 => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemMaxCountEquippedSocketed => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::ItemUniqueEquippableSocketed => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::TooMuchGold => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::NotDuringArenaMatch => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::CannotTradeThat => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
            InventoryResult::PersonalArenaRatingTooLow => {
                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                    bag_type_subclass: result_if_bag_type_subclass,
                    item1: result_if_item1,
                    item2: result_if_item2,
                }
            }
        };

        Ok(Self {
            result: result_if,
        })
    }

}

impl crate::Message for SMSG_INVENTORY_CHANGE_FAILURE {
    const OPCODE: u32 = 0x0112;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INVENTORY_CHANGE_FAILURE {{").unwrap();
        // Members
        writeln!(s, "    result = {};", InventoryResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                writeln!(s, "    item1 = {};", item1.guid()).unwrap();
                writeln!(s, "    item2 = {};", item2.guid()).unwrap();
                writeln!(s, "    bag_type_subclass = {};", bag_type_subclass).unwrap();
            }
            _ => {}
        }

        match &self.result {
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                writeln!(s, "    required_level = {};", required_level.as_int()).unwrap();
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 274_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self.result {
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item2", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_type_subclass", "    ");
            }
            _ => {}
        }

        match &self.result {
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "required_level", "    ");
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            crate::tbc::SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: InventoryResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                // item1: Guid
                w.write_all(&item1.guid().to_le_bytes())?;

                // item2: Guid
                w.write_all(&item2.guid().to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            _ => {}
        }

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                // required_level: Level32
                w.write_all(&u32::from(required_level.as_int()).to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(274, "SMSG_INVENTORY_CHANGE_FAILURE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INVENTORY_CHANGE_FAILURE {}

impl SMSG_INVENTORY_CHANGE_FAILURE {
    pub(crate) const fn size(&self) -> usize {
        self.result.size() // result: SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    Ok,
    CantEquipLevelI {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
        required_level: Level,
    },
    CantEquipSkill {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemDoesntGoToSlot {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BagFull {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NonemptyBagOverOtherBag {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantTradeEquipBags {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    OnlyAmmoCanGoHere {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NoRequiredProficiency {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NoEquipmentSlotAvailable {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    YouCanNeverUseThatItem {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    YouCanNeverUseThatItem2 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NoEquipmentSlotAvailable2 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantEquipWithTwohanded {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantDualWield {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemDoesntGoIntoBag {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemDoesntGoIntoBag2 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantCarryMoreOfThis {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NoEquipmentSlotAvailable3 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemCantStack {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemCantBeEquipped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemsCantBeSwapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    SlotIsEmpty {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemNotFound {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantDropSoulbound {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    OutOfRange {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    TriedToSplitMoreThanCount {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CouldntSplitItems {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    MissingReagent {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotEnoughMoney {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotABag {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CanOnlyDoWithEmptyBags {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    DontOwnThatItem {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CanEquipOnly1Quiver {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    MustPurchaseThatBagSlot {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    TooFarAwayFromBank {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemLocked {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    YouAreStunned {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    YouAreDead {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantDoRightNow {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    IntBagError {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CanEquipOnly1Bolt {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CanEquipOnly1Ammopouch {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    StackableCantBeWrapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    EquippedCantBeWrapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    WrappedCantBeWrapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BoundCantBeWrapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    UniqueCantBeWrapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BagsCantBeWrapped {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    AlreadyLooted {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    InventoryFull {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BankFull {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemIsCurrentlySoldOut {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BagFull3 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemNotFound2 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemCantStack2 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BagFull4 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemSoldOut {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ObjectIsBusy {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    None {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotInCombat {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotWhileDisarmed {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BagFull6 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantEquipRank {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CantEquipReputation {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    TooManySpecialBags {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    LootCantLootThatNow {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemUniqueEquipable {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    VendorMissingTurnins {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotEnoughHonorPoints {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotEnoughArenaPoints {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemMaxCountSocketed {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    MailBoundItem {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NoSplitWhileProspecting {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    BagFull7 {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemMaxCountEquippedSocketed {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    ItemUniqueEquippableSocketed {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    TooMuchGold {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    NotDuringArenaMatch {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    CannotTradeThat {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
    PersonalArenaRatingTooLow {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
    },
}

impl Default for SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Ok
    }
}

impl SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Ok => 0,
            Self::CantEquipLevelI { .. } => 1,
            Self::CantEquipSkill { .. } => 2,
            Self::ItemDoesntGoToSlot { .. } => 3,
            Self::BagFull { .. } => 4,
            Self::NonemptyBagOverOtherBag { .. } => 5,
            Self::CantTradeEquipBags { .. } => 6,
            Self::OnlyAmmoCanGoHere { .. } => 7,
            Self::NoRequiredProficiency { .. } => 8,
            Self::NoEquipmentSlotAvailable { .. } => 9,
            Self::YouCanNeverUseThatItem { .. } => 10,
            Self::YouCanNeverUseThatItem2 { .. } => 11,
            Self::NoEquipmentSlotAvailable2 { .. } => 12,
            Self::CantEquipWithTwohanded { .. } => 13,
            Self::CantDualWield { .. } => 14,
            Self::ItemDoesntGoIntoBag { .. } => 15,
            Self::ItemDoesntGoIntoBag2 { .. } => 16,
            Self::CantCarryMoreOfThis { .. } => 17,
            Self::NoEquipmentSlotAvailable3 { .. } => 18,
            Self::ItemCantStack { .. } => 19,
            Self::ItemCantBeEquipped { .. } => 20,
            Self::ItemsCantBeSwapped { .. } => 21,
            Self::SlotIsEmpty { .. } => 22,
            Self::ItemNotFound { .. } => 23,
            Self::CantDropSoulbound { .. } => 24,
            Self::OutOfRange { .. } => 25,
            Self::TriedToSplitMoreThanCount { .. } => 26,
            Self::CouldntSplitItems { .. } => 27,
            Self::MissingReagent { .. } => 28,
            Self::NotEnoughMoney { .. } => 29,
            Self::NotABag { .. } => 30,
            Self::CanOnlyDoWithEmptyBags { .. } => 31,
            Self::DontOwnThatItem { .. } => 32,
            Self::CanEquipOnly1Quiver { .. } => 33,
            Self::MustPurchaseThatBagSlot { .. } => 34,
            Self::TooFarAwayFromBank { .. } => 35,
            Self::ItemLocked { .. } => 36,
            Self::YouAreStunned { .. } => 37,
            Self::YouAreDead { .. } => 38,
            Self::CantDoRightNow { .. } => 39,
            Self::IntBagError { .. } => 40,
            Self::CanEquipOnly1Bolt { .. } => 41,
            Self::CanEquipOnly1Ammopouch { .. } => 42,
            Self::StackableCantBeWrapped { .. } => 43,
            Self::EquippedCantBeWrapped { .. } => 44,
            Self::WrappedCantBeWrapped { .. } => 45,
            Self::BoundCantBeWrapped { .. } => 46,
            Self::UniqueCantBeWrapped { .. } => 47,
            Self::BagsCantBeWrapped { .. } => 48,
            Self::AlreadyLooted { .. } => 49,
            Self::InventoryFull { .. } => 50,
            Self::BankFull { .. } => 51,
            Self::ItemIsCurrentlySoldOut { .. } => 52,
            Self::BagFull3 { .. } => 53,
            Self::ItemNotFound2 { .. } => 54,
            Self::ItemCantStack2 { .. } => 55,
            Self::BagFull4 { .. } => 56,
            Self::ItemSoldOut { .. } => 57,
            Self::ObjectIsBusy { .. } => 58,
            Self::None { .. } => 59,
            Self::NotInCombat { .. } => 60,
            Self::NotWhileDisarmed { .. } => 61,
            Self::BagFull6 { .. } => 62,
            Self::CantEquipRank { .. } => 63,
            Self::CantEquipReputation { .. } => 64,
            Self::TooManySpecialBags { .. } => 65,
            Self::LootCantLootThatNow { .. } => 66,
            Self::ItemUniqueEquipable { .. } => 67,
            Self::VendorMissingTurnins { .. } => 68,
            Self::NotEnoughHonorPoints { .. } => 69,
            Self::NotEnoughArenaPoints { .. } => 70,
            Self::ItemMaxCountSocketed { .. } => 71,
            Self::MailBoundItem { .. } => 72,
            Self::NoSplitWhileProspecting { .. } => 73,
            Self::BagFull7 { .. } => 74,
            Self::ItemMaxCountEquippedSocketed { .. } => 75,
            Self::ItemUniqueEquippableSocketed { .. } => 76,
            Self::TooMuchGold { .. } => 77,
            Self::NotDuringArenaMatch { .. } => 78,
            Self::CannotTradeThat { .. } => 79,
            Self::PersonalArenaRatingTooLow { .. } => 80,
        }
    }

}

impl std::fmt::Display for SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::CantEquipLevelI{ .. } => f.write_str("CantEquipLevelI"),
            Self::CantEquipSkill{ .. } => f.write_str("CantEquipSkill"),
            Self::ItemDoesntGoToSlot{ .. } => f.write_str("ItemDoesntGoToSlot"),
            Self::BagFull{ .. } => f.write_str("BagFull"),
            Self::NonemptyBagOverOtherBag{ .. } => f.write_str("NonemptyBagOverOtherBag"),
            Self::CantTradeEquipBags{ .. } => f.write_str("CantTradeEquipBags"),
            Self::OnlyAmmoCanGoHere{ .. } => f.write_str("OnlyAmmoCanGoHere"),
            Self::NoRequiredProficiency{ .. } => f.write_str("NoRequiredProficiency"),
            Self::NoEquipmentSlotAvailable{ .. } => f.write_str("NoEquipmentSlotAvailable"),
            Self::YouCanNeverUseThatItem{ .. } => f.write_str("YouCanNeverUseThatItem"),
            Self::YouCanNeverUseThatItem2{ .. } => f.write_str("YouCanNeverUseThatItem2"),
            Self::NoEquipmentSlotAvailable2{ .. } => f.write_str("NoEquipmentSlotAvailable2"),
            Self::CantEquipWithTwohanded{ .. } => f.write_str("CantEquipWithTwohanded"),
            Self::CantDualWield{ .. } => f.write_str("CantDualWield"),
            Self::ItemDoesntGoIntoBag{ .. } => f.write_str("ItemDoesntGoIntoBag"),
            Self::ItemDoesntGoIntoBag2{ .. } => f.write_str("ItemDoesntGoIntoBag2"),
            Self::CantCarryMoreOfThis{ .. } => f.write_str("CantCarryMoreOfThis"),
            Self::NoEquipmentSlotAvailable3{ .. } => f.write_str("NoEquipmentSlotAvailable3"),
            Self::ItemCantStack{ .. } => f.write_str("ItemCantStack"),
            Self::ItemCantBeEquipped{ .. } => f.write_str("ItemCantBeEquipped"),
            Self::ItemsCantBeSwapped{ .. } => f.write_str("ItemsCantBeSwapped"),
            Self::SlotIsEmpty{ .. } => f.write_str("SlotIsEmpty"),
            Self::ItemNotFound{ .. } => f.write_str("ItemNotFound"),
            Self::CantDropSoulbound{ .. } => f.write_str("CantDropSoulbound"),
            Self::OutOfRange{ .. } => f.write_str("OutOfRange"),
            Self::TriedToSplitMoreThanCount{ .. } => f.write_str("TriedToSplitMoreThanCount"),
            Self::CouldntSplitItems{ .. } => f.write_str("CouldntSplitItems"),
            Self::MissingReagent{ .. } => f.write_str("MissingReagent"),
            Self::NotEnoughMoney{ .. } => f.write_str("NotEnoughMoney"),
            Self::NotABag{ .. } => f.write_str("NotABag"),
            Self::CanOnlyDoWithEmptyBags{ .. } => f.write_str("CanOnlyDoWithEmptyBags"),
            Self::DontOwnThatItem{ .. } => f.write_str("DontOwnThatItem"),
            Self::CanEquipOnly1Quiver{ .. } => f.write_str("CanEquipOnly1Quiver"),
            Self::MustPurchaseThatBagSlot{ .. } => f.write_str("MustPurchaseThatBagSlot"),
            Self::TooFarAwayFromBank{ .. } => f.write_str("TooFarAwayFromBank"),
            Self::ItemLocked{ .. } => f.write_str("ItemLocked"),
            Self::YouAreStunned{ .. } => f.write_str("YouAreStunned"),
            Self::YouAreDead{ .. } => f.write_str("YouAreDead"),
            Self::CantDoRightNow{ .. } => f.write_str("CantDoRightNow"),
            Self::IntBagError{ .. } => f.write_str("IntBagError"),
            Self::CanEquipOnly1Bolt{ .. } => f.write_str("CanEquipOnly1Bolt"),
            Self::CanEquipOnly1Ammopouch{ .. } => f.write_str("CanEquipOnly1Ammopouch"),
            Self::StackableCantBeWrapped{ .. } => f.write_str("StackableCantBeWrapped"),
            Self::EquippedCantBeWrapped{ .. } => f.write_str("EquippedCantBeWrapped"),
            Self::WrappedCantBeWrapped{ .. } => f.write_str("WrappedCantBeWrapped"),
            Self::BoundCantBeWrapped{ .. } => f.write_str("BoundCantBeWrapped"),
            Self::UniqueCantBeWrapped{ .. } => f.write_str("UniqueCantBeWrapped"),
            Self::BagsCantBeWrapped{ .. } => f.write_str("BagsCantBeWrapped"),
            Self::AlreadyLooted{ .. } => f.write_str("AlreadyLooted"),
            Self::InventoryFull{ .. } => f.write_str("InventoryFull"),
            Self::BankFull{ .. } => f.write_str("BankFull"),
            Self::ItemIsCurrentlySoldOut{ .. } => f.write_str("ItemIsCurrentlySoldOut"),
            Self::BagFull3{ .. } => f.write_str("BagFull3"),
            Self::ItemNotFound2{ .. } => f.write_str("ItemNotFound2"),
            Self::ItemCantStack2{ .. } => f.write_str("ItemCantStack2"),
            Self::BagFull4{ .. } => f.write_str("BagFull4"),
            Self::ItemSoldOut{ .. } => f.write_str("ItemSoldOut"),
            Self::ObjectIsBusy{ .. } => f.write_str("ObjectIsBusy"),
            Self::None{ .. } => f.write_str("None"),
            Self::NotInCombat{ .. } => f.write_str("NotInCombat"),
            Self::NotWhileDisarmed{ .. } => f.write_str("NotWhileDisarmed"),
            Self::BagFull6{ .. } => f.write_str("BagFull6"),
            Self::CantEquipRank{ .. } => f.write_str("CantEquipRank"),
            Self::CantEquipReputation{ .. } => f.write_str("CantEquipReputation"),
            Self::TooManySpecialBags{ .. } => f.write_str("TooManySpecialBags"),
            Self::LootCantLootThatNow{ .. } => f.write_str("LootCantLootThatNow"),
            Self::ItemUniqueEquipable{ .. } => f.write_str("ItemUniqueEquipable"),
            Self::VendorMissingTurnins{ .. } => f.write_str("VendorMissingTurnins"),
            Self::NotEnoughHonorPoints{ .. } => f.write_str("NotEnoughHonorPoints"),
            Self::NotEnoughArenaPoints{ .. } => f.write_str("NotEnoughArenaPoints"),
            Self::ItemMaxCountSocketed{ .. } => f.write_str("ItemMaxCountSocketed"),
            Self::MailBoundItem{ .. } => f.write_str("MailBoundItem"),
            Self::NoSplitWhileProspecting{ .. } => f.write_str("NoSplitWhileProspecting"),
            Self::BagFull7{ .. } => f.write_str("BagFull7"),
            Self::ItemMaxCountEquippedSocketed{ .. } => f.write_str("ItemMaxCountEquippedSocketed"),
            Self::ItemUniqueEquippableSocketed{ .. } => f.write_str("ItemUniqueEquippableSocketed"),
            Self::TooMuchGold{ .. } => f.write_str("TooMuchGold"),
            Self::NotDuringArenaMatch{ .. } => f.write_str("NotDuringArenaMatch"),
            Self::CannotTradeThat{ .. } => f.write_str("CannotTradeThat"),
            Self::PersonalArenaRatingTooLow{ .. } => f.write_str("PersonalArenaRatingTooLow"),
        }
    }
}

impl SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::CantEquipLevelI {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
                + 4 // required_level: Level32
            }
            Self::CantEquipSkill {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemDoesntGoToSlot {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NonemptyBagOverOtherBag {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantTradeEquipBags {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::OnlyAmmoCanGoHere {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoRequiredProficiency {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoEquipmentSlotAvailable {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouCanNeverUseThatItem {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouCanNeverUseThatItem2 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoEquipmentSlotAvailable2 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantEquipWithTwohanded {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantDualWield {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemDoesntGoIntoBag {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemDoesntGoIntoBag2 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantCarryMoreOfThis {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoEquipmentSlotAvailable3 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemCantStack {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemCantBeEquipped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemsCantBeSwapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::SlotIsEmpty {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemNotFound {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantDropSoulbound {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::OutOfRange {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TriedToSplitMoreThanCount {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CouldntSplitItems {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::MissingReagent {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotEnoughMoney {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotABag {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanOnlyDoWithEmptyBags {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::DontOwnThatItem {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanEquipOnly1Quiver {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::MustPurchaseThatBagSlot {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TooFarAwayFromBank {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemLocked {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouAreStunned {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouAreDead {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantDoRightNow {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::IntBagError {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanEquipOnly1Bolt {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanEquipOnly1Ammopouch {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::StackableCantBeWrapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::EquippedCantBeWrapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::WrappedCantBeWrapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BoundCantBeWrapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::UniqueCantBeWrapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagsCantBeWrapped {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::AlreadyLooted {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::InventoryFull {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BankFull {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemIsCurrentlySoldOut {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull3 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemNotFound2 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemCantStack2 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull4 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemSoldOut {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ObjectIsBusy {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::None {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotInCombat {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotWhileDisarmed {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull6 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantEquipRank {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantEquipReputation {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TooManySpecialBags {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::LootCantLootThatNow {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemUniqueEquipable {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::VendorMissingTurnins {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotEnoughHonorPoints {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotEnoughArenaPoints {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemMaxCountSocketed {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::MailBoundItem {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoSplitWhileProspecting {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull7 {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemMaxCountEquippedSocketed {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemUniqueEquippableSocketed {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TooMuchGold {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotDuringArenaMatch {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CannotTradeThat {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::PersonalArenaRatingTooLow {
                ..
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            _ => 1,
        }
    }
}

