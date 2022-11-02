use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::InventoryResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
///         u32 required_level;
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
        let size_assert_header_size = w.len();
        // result: InventoryResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok => {}
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

                // required_level: u32
                w.write_all(&required_level.to_le_bytes())?;

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
        }

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok => {}
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

                // required_level: u32
                w.write_all(&required_level.to_le_bytes())?;

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
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: InventoryResult
        let result: InventoryResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            InventoryResult::Ok => SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok,
            InventoryResult::CantEquipLevelI => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                    bag_type_subclass,
                    item1,
                    item2,
                    required_level,
                }
            }
            InventoryResult::CantEquipSkill => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemDoesntGoToSlot => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NonemptyBagOverOtherBag => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantTradeEquipBags => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::OnlyAmmoCanGoHere => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoRequiredProficiency => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouCanNeverUseThatItem => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouCanNeverUseThatItem2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantEquipWithTwohanded => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantDualWield => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantCarryMoreOfThis => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemCantStack => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemCantBeEquipped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemsCantBeSwapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::SlotIsEmpty => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemNotFound => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantDropSoulbound => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::OutOfRange => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TriedToSplitMoreThanCount => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CouldntSplitItems => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::MissingReagent => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotEnoughMoney => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotABag => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::DontOwnThatItem => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanEquipOnly1Quiver => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::MustPurchaseThatBagSlot => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TooFarAwayFromBank => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemLocked => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouAreStunned => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouAreDead => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantDoRightNow => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::IntBagError => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanEquipOnly1Bolt => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanEquipOnly1Ammopouch => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::StackableCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::EquippedCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::WrappedCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BoundCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::UniqueCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagsCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::AlreadyLooted => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::InventoryFull => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BankFull => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemIsCurrentlySoldOut => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull3 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemNotFound2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemCantStack2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull4 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemSoldOut => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ObjectIsBusy => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::None => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotInCombat => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotWhileDisarmed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull6 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantEquipRank => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantEquipReputation => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TooManySpecialBags => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::LootCantLootThatNow => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemUniqueEquipable => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::VendorMissingTurnins => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotEnoughHonorPoints => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotEnoughArenaPoints => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemMaxCountSocketed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::MailBoundItem => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoSplitWhileProspecting => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull7 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemMaxCountEquippedSocketed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemUniqueEquippableSocketed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TooMuchGold => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotDuringArenaMatch => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CannotTradeThat => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::PersonalArenaRatingTooLow => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
        };

        let result_if = match result {
            InventoryResult::Ok => SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::Ok,
            InventoryResult::CantEquipLevelI => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipLevelI {
                    bag_type_subclass,
                    item1,
                    item2,
                    required_level,
                }
            }
            InventoryResult::CantEquipSkill => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipSkill {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemDoesntGoToSlot => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoToSlot {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NonemptyBagOverOtherBag => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NonemptyBagOverOtherBag {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantTradeEquipBags => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantTradeEquipBags {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::OnlyAmmoCanGoHere => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OnlyAmmoCanGoHere {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoRequiredProficiency => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoRequiredProficiency {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouCanNeverUseThatItem => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouCanNeverUseThatItem2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouCanNeverUseThatItem2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantEquipWithTwohanded => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipWithTwohanded {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantDualWield => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDualWield {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemDoesntGoIntoBag2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemDoesntGoIntoBag2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantCarryMoreOfThis => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantCarryMoreOfThis {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoEquipmentSlotAvailable3 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoEquipmentSlotAvailable3 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemCantStack => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemCantBeEquipped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantBeEquipped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemsCantBeSwapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemsCantBeSwapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::SlotIsEmpty => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SlotIsEmpty {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemNotFound => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantDropSoulbound => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDropSoulbound {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::OutOfRange => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OutOfRange {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TriedToSplitMoreThanCount => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TriedToSplitMoreThanCount {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CouldntSplitItems => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CouldntSplitItems {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::MissingReagent => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MissingReagent {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotEnoughMoney => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughMoney {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotABag => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotABag {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanOnlyDoWithEmptyBags => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanOnlyDoWithEmptyBags {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::DontOwnThatItem => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DontOwnThatItem {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanEquipOnly1Quiver => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Quiver {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::MustPurchaseThatBagSlot => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MustPurchaseThatBagSlot {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TooFarAwayFromBank => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooFarAwayFromBank {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemLocked => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemLocked {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouAreStunned => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreStunned {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::YouAreDead => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YouAreDead {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantDoRightNow => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantDoRightNow {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::IntBagError => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::IntBagError {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanEquipOnly1Bolt => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Bolt {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CanEquipOnly1Ammopouch => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CanEquipOnly1Ammopouch {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::StackableCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::StackableCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::EquippedCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EquippedCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::WrappedCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WrappedCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BoundCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BoundCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::UniqueCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UniqueCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagsCantBeWrapped => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagsCantBeWrapped {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::AlreadyLooted => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::AlreadyLooted {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::InventoryFull => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::InventoryFull {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BankFull => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BankFull {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemIsCurrentlySoldOut => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemIsCurrentlySoldOut {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull3 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull3 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemNotFound2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemNotFound2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemCantStack2 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemCantStack2 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull4 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull4 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemSoldOut => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemSoldOut {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ObjectIsBusy => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ObjectIsBusy {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::None => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::None {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotInCombat => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotInCombat {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotWhileDisarmed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotWhileDisarmed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull6 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull6 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantEquipRank => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipRank {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CantEquipReputation => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CantEquipReputation {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TooManySpecialBags => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooManySpecialBags {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::LootCantLootThatNow => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LootCantLootThatNow {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemUniqueEquipable => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquipable {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::VendorMissingTurnins => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::VendorMissingTurnins {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotEnoughHonorPoints => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughHonorPoints {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotEnoughArenaPoints => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotEnoughArenaPoints {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemMaxCountSocketed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountSocketed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::MailBoundItem => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MailBoundItem {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NoSplitWhileProspecting => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NoSplitWhileProspecting {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::BagFull7 => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BagFull7 {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemMaxCountEquippedSocketed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemMaxCountEquippedSocketed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::ItemUniqueEquippableSocketed => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ItemUniqueEquippableSocketed {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::TooMuchGold => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TooMuchGold {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::NotDuringArenaMatch => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NotDuringArenaMatch {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::CannotTradeThat => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CannotTradeThat {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
            InventoryResult::PersonalArenaRatingTooLow => {
                // item1: Guid
                let item1 = Guid::read(r)?;

                // item2: Guid
                let item2 = Guid::read(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::PersonalArenaRatingTooLow {
                    bag_type_subclass,
                    item1,
                    item2,
                }
            }
        };

        Ok(Self {
            result: result_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_INVENTORY_CHANGE_FAILURE {}

impl SMSG_INVENTORY_CHANGE_FAILURE {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    Ok,
    CantEquipLevelI {
        bag_type_subclass: u8,
        item1: Guid,
        item2: Guid,
        required_level: u32,
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

impl SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Ok => {
                1
            }
            Self::CantEquipLevelI {
                bag_type_subclass,
                item1,
                item2,
                required_level,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
                + 4 // required_level: u32
            }
            Self::CantEquipSkill {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemDoesntGoToSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NonemptyBagOverOtherBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantTradeEquipBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::OnlyAmmoCanGoHere {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoRequiredProficiency {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoEquipmentSlotAvailable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouCanNeverUseThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouCanNeverUseThatItem2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoEquipmentSlotAvailable2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantEquipWithTwohanded {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantDualWield {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemDoesntGoIntoBag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemDoesntGoIntoBag2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantCarryMoreOfThis {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoEquipmentSlotAvailable3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemCantStack {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemCantBeEquipped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemsCantBeSwapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::SlotIsEmpty {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemNotFound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantDropSoulbound {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::OutOfRange {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TriedToSplitMoreThanCount {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CouldntSplitItems {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::MissingReagent {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotEnoughMoney {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotABag {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanOnlyDoWithEmptyBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::DontOwnThatItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanEquipOnly1Quiver {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::MustPurchaseThatBagSlot {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TooFarAwayFromBank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemLocked {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouAreStunned {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::YouAreDead {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantDoRightNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::IntBagError {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanEquipOnly1Bolt {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CanEquipOnly1Ammopouch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::StackableCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::EquippedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::WrappedCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BoundCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::UniqueCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagsCantBeWrapped {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::AlreadyLooted {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::InventoryFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BankFull {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemIsCurrentlySoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull3 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemNotFound2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemCantStack2 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull4 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemSoldOut {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ObjectIsBusy {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::None {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotInCombat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotWhileDisarmed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull6 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantEquipRank {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CantEquipReputation {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TooManySpecialBags {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::LootCantLootThatNow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemUniqueEquipable {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::VendorMissingTurnins {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotEnoughHonorPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotEnoughArenaPoints {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemMaxCountSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::MailBoundItem {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NoSplitWhileProspecting {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::BagFull7 {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemMaxCountEquippedSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::ItemUniqueEquippableSocketed {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::TooMuchGold {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::NotDuringArenaMatch {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::CannotTradeThat {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
            Self::PersonalArenaRatingTooLow {
                bag_type_subclass,
                item1,
                item2,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1: Guid
                + 8 // item2: Guid
            }
        }
    }
}

