use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::InventoryResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_inventory_change_failure.wowm#L5):
/// ```text
/// smsg SMSG_INVENTORY_CHANGE_FAILURE = 0x0112 {
///     InventoryResult result;
///     if (result == CANT_EQUIP_LEVEL_I) {
///         u32 required_level;
///     }
///     if (result != OK) {
///         u64 item1_guid;
///         u64 item2_guid;
///         u8 bag_type_subclass;
///     }
/// }
/// ```
pub struct SMSG_INVENTORY_CHANGE_FAILURE {
    pub result: SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult,
}

impl crate::Message for SMSG_INVENTORY_CHANGE_FAILURE {
    const OPCODE: u32 = 0x0112;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: InventoryResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok => {}
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1_guid,
                item2_guid,
                required_level,
            } => {
                // required_level: u32
                w.write_all(&required_level.to_le_bytes())?;

                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
        }

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok => {}
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                bag_type_subclass,
                item1_guid,
                item2_guid,
                required_level,
            } => {
                // required_level: u32
                w.write_all(&required_level.to_le_bytes())?;

                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: InventoryResult
        let result: InventoryResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            InventoryResult::Ok => SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok,
            InventoryResult::CantEquipLevelI => {
                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                }
            }
            InventoryResult::CantEquipSkill => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemDoesntGoToSlot => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NonemptyBagOverOtherBag => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantTradeEquipBags => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OnlyAmmoCanGoHere => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoRequiredProficiency => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouCanNeverUseThatItem => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouCanNeverUseThatItem2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantEquipWithTwohanded => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantDualWield => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantCarryMoreOfThis => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemCantStack => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemCantBeEquipped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemsCantBeSwapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::SlotIsEmpty => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemNotFound => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantDropSoulbound => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OutOfRange => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TriedToSplitMoreThanCount => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CouldntSplitItems => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MissingReagent => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotEnoughMoney => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotABag => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::DontOwnThatItem => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanEquipOnly1Quiver => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MustPurchaseThatBagSlot => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TooFarAwayFromBank => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemLocked => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouAreStunned => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouAreDead => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantDoRightNow => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::IntBagError => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanEquipOnly1Bolt => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanEquipOnly1Ammopouch => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::StackableCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::EquippedCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::WrappedCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BoundCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::UniqueCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagsCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::AlreadyLooted => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::InventoryFull => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BankFull => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemIsCurrentlySoldOut => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemNotFound2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemCantStack2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull4 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemSoldOut => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ObjectIsBusy => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::None => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotInCombat => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotWhileDisarmed => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull6 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantEquipRank => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantEquipReputation => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TooManySpecialBags => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::LootCantLootThatNow => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
        };

        let result_if = match result {
            InventoryResult::Ok => SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok,
            InventoryResult::CantEquipLevelI => {
                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                }
            }
            InventoryResult::CantEquipSkill => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemDoesntGoToSlot => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NonemptyBagOverOtherBag => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantTradeEquipBags => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OnlyAmmoCanGoHere => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoRequiredProficiency => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouCanNeverUseThatItem => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouCanNeverUseThatItem2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantEquipWithTwohanded => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantDualWield => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantCarryMoreOfThis => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemCantStack => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemCantBeEquipped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemsCantBeSwapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::SlotIsEmpty => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemNotFound => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantDropSoulbound => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OutOfRange => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TriedToSplitMoreThanCount => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CouldntSplitItems => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MissingReagent => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotEnoughMoney => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotABag => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::DontOwnThatItem => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanEquipOnly1Quiver => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MustPurchaseThatBagSlot => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TooFarAwayFromBank => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemLocked => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouAreStunned => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YouAreDead => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantDoRightNow => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::IntBagError => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanEquipOnly1Bolt => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CanEquipOnly1Ammopouch => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::StackableCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::EquippedCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::WrappedCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BoundCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::UniqueCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagsCantBeWrapped => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::AlreadyLooted => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::InventoryFull => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BankFull => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemIsCurrentlySoldOut => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemNotFound2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemCantStack2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull4 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ItemSoldOut => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ObjectIsBusy => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::None => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotInCombat => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NotWhileDisarmed => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BagFull6 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantEquipRank => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CantEquipReputation => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TooManySpecialBags => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::LootCantLootThatNow => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
        };

        Ok(Self {
            result: result_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_INVENTORY_CHANGE_FAILURE {}

impl SMSG_INVENTORY_CHANGE_FAILURE {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    Ok,
    CantEquipLevelI {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
        required_level: u32,
    },
    CantEquipSkill {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemDoesntGoToSlot {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BagFull {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NonemptyBagOverOtherBag {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantTradeEquipBags {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    OnlyAmmoCanGoHere {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NoRequiredProficiency {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NoEquipmentSlotAvailable {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YouCanNeverUseThatItem {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YouCanNeverUseThatItem2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NoEquipmentSlotAvailable2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantEquipWithTwohanded {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantDualWield {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemDoesntGoIntoBag {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemDoesntGoIntoBag2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantCarryMoreOfThis {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NoEquipmentSlotAvailable3 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemCantStack {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemCantBeEquipped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemsCantBeSwapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    SlotIsEmpty {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemNotFound {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantDropSoulbound {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    OutOfRange {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    TriedToSplitMoreThanCount {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CouldntSplitItems {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    MissingReagent {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NotEnoughMoney {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NotABag {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CanOnlyDoWithEmptyBags {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    DontOwnThatItem {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CanEquipOnly1Quiver {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    MustPurchaseThatBagSlot {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    TooFarAwayFromBank {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemLocked {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YouAreStunned {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YouAreDead {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantDoRightNow {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    IntBagError {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CanEquipOnly1Bolt {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CanEquipOnly1Ammopouch {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    StackableCantBeWrapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    EquippedCantBeWrapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    WrappedCantBeWrapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BoundCantBeWrapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    UniqueCantBeWrapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BagsCantBeWrapped {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    AlreadyLooted {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    InventoryFull {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BankFull {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemIsCurrentlySoldOut {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BagFull3 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemNotFound2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemCantStack2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BagFull4 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ItemSoldOut {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ObjectIsBusy {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    None {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NotInCombat {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NotWhileDisarmed {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BagFull6 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantEquipRank {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CantEquipReputation {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    TooManySpecialBags {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    LootCantLootThatNow {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
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
        }
    }

}

impl SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Ok => {
                1
            }
            Self::CantEquipLevelI {
                bag_type_subclass,
                item1_guid,
                item2_guid,
                required_level,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
                + 4 // required_level: u32
            }
            Self::CantEquipSkill {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BagFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantTradeEquipBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NoRequiredProficiency {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantEquipWithTwohanded {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantDualWield {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantCarryMoreOfThis {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemCantStack {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemCantBeEquipped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemsCantBeSwapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::SlotIsEmpty {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemNotFound {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantDropSoulbound {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::OutOfRange {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CouldntSplitItems {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::MissingReagent {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NotEnoughMoney {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NotABag {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::DontOwnThatItem {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::TooFarAwayFromBank {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemLocked {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YouAreStunned {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YouAreDead {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantDoRightNow {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::IntBagError {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::StackableCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::EquippedCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::WrappedCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BoundCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::UniqueCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BagsCantBeWrapped {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::AlreadyLooted {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::InventoryFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BankFull {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BagFull3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemNotFound2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemCantStack2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BagFull4 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ItemSoldOut {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ObjectIsBusy {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::None {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NotInCombat {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NotWhileDisarmed {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BagFull6 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantEquipRank {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CantEquipReputation {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::TooManySpecialBags {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::LootCantLootThatNow {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
        }
    }
}

