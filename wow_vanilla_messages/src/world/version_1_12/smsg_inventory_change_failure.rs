use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::InventoryResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
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

impl ServerMessage for SMSG_INVENTORY_CHANGE_FAILURE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: InventoryResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OK => {}
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_LEVEL_I {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_SKILL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_TO_SLOT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_TRADE_EQUIP_BAGS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ONLY_AMMO_CAN_GO_HERE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_REQUIRED_PROFICIENCY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_WITH_TWOHANDED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DUAL_WIELD {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_CARRY_MORE_OF_THIS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_BE_EQUIPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEMS_CANT_BE_SWAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SLOT_IS_EMPTY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DROP_SOULBOUND {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OUT_OF_RANGE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::COULDNT_SPLIT_ITEMS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MISSING_REAGENT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_ENOUGH_MONEY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_A_BAG {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DONT_OWN_THAT_ITEM {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_QUIVER {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_FAR_AWAY_FROM_BANK {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_LOCKED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_STUNNED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_DEAD {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DO_RIGHT_NOW {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INT_BAG_ERROR {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_BOLT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::STACKABLE_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EQUIPPED_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WRAPPED_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BOUND_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UNIQUE_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAGS_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ALREADY_LOOTED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INVENTORY_FULL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BANK_FULL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL3 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL4 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_SOLD_OUT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OBJECT_IS_BUSY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_IN_COMBAT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_WHILE_DISARMED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL6 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_RANK {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_REPUTATION {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_MANY_SPECIAL_BAGS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LOOT_CANT_LOOT_THAT_NOW {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OK => {}
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_LEVEL_I {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_SKILL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_TO_SLOT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_TRADE_EQUIP_BAGS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ONLY_AMMO_CAN_GO_HERE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_REQUIRED_PROFICIENCY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_WITH_TWOHANDED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DUAL_WIELD {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_CARRY_MORE_OF_THIS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_BE_EQUIPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEMS_CANT_BE_SWAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SLOT_IS_EMPTY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DROP_SOULBOUND {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OUT_OF_RANGE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::COULDNT_SPLIT_ITEMS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MISSING_REAGENT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_ENOUGH_MONEY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_A_BAG {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DONT_OWN_THAT_ITEM {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_QUIVER {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_FAR_AWAY_FROM_BANK {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_LOCKED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_STUNNED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_DEAD {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DO_RIGHT_NOW {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INT_BAG_ERROR {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_BOLT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::STACKABLE_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EQUIPPED_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WRAPPED_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BOUND_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UNIQUE_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAGS_CANT_BE_WRAPPED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ALREADY_LOOTED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INVENTORY_FULL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BANK_FULL {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL3 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK2 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL4 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_SOLD_OUT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OBJECT_IS_BUSY {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONE {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_IN_COMBAT {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_WHILE_DISARMED {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL6 {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_RANK {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_REPUTATION {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_MANY_SPECIAL_BAGS {
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
            SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LOOT_CANT_LOOT_THAT_NOW {
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
    const OPCODE: u16 = 0x0112;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: InventoryResult
        let result: InventoryResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OK,
            InventoryResult::CANT_EQUIP_LEVEL_I => {
                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_LEVEL_I {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                }
            }
            InventoryResult::CANT_EQUIP_SKILL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_SKILL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_DOESNT_GO_TO_SLOT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_TRADE_EQUIP_BAGS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_TRADE_EQUIP_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ONLY_AMMO_CAN_GO_HERE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ONLY_AMMO_CAN_GO_HERE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_REQUIRED_PROFICIENCY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_REQUIRED_PROFICIENCY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_EQUIP_WITH_TWOHANDED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_DUAL_WIELD => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DUAL_WIELD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_DOESNT_GO_INTO_BAG => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_CARRY_MORE_OF_THIS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_CARRY_MORE_OF_THIS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_CANT_STACK => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_CANT_BE_EQUIPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_BE_EQUIPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEMS_CANT_BE_SWAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEMS_CANT_BE_SWAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::SLOT_IS_EMPTY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SLOT_IS_EMPTY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_NOT_FOUND => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_DROP_SOULBOUND => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DROP_SOULBOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OUT_OF_RANGE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OUT_OF_RANGE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::COULDNT_SPLIT_ITEMS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::COULDNT_SPLIT_ITEMS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MISSING_REAGENT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MISSING_REAGENT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_ENOUGH_MONEY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_ENOUGH_MONEY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_A_BAG => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_A_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::DONT_OWN_THAT_ITEM => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DONT_OWN_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_EQUIP_ONLY1_QUIVER => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TOO_FAR_AWAY_FROM_BANK => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_FAR_AWAY_FROM_BANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_LOCKED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_LOCKED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_ARE_STUNNED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_STUNNED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_ARE_DEAD => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_DEAD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_DO_RIGHT_NOW => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DO_RIGHT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::INT_BAG_ERROR => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INT_BAG_ERROR {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_EQUIP_ONLY1_BOLT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_BOLT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::STACKABLE_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::STACKABLE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::EQUIPPED_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::WRAPPED_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WRAPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BOUND_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BOUND_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::UNIQUE_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UNIQUE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAGS_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAGS_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ALREADY_LOOTED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ALREADY_LOOTED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::INVENTORY_FULL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INVENTORY_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BANK_FULL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BANK_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_NOT_FOUND2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_CANT_STACK2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL4 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_SOLD_OUT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OBJECT_IS_BUSY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OBJECT_IS_BUSY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NONE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_IN_COMBAT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_IN_COMBAT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_WHILE_DISARMED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_WHILE_DISARMED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL6 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_EQUIP_RANK => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_RANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_EQUIP_REPUTATION => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_REPUTATION {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TOO_MANY_SPECIAL_BAGS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_MANY_SPECIAL_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::LOOT_CANT_LOOT_THAT_NOW => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
        };

        let result_if = match result {
            InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OK,
            InventoryResult::CANT_EQUIP_LEVEL_I => {
                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_LEVEL_I {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                }
            }
            InventoryResult::CANT_EQUIP_SKILL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_SKILL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_DOESNT_GO_TO_SLOT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_TRADE_EQUIP_BAGS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_TRADE_EQUIP_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ONLY_AMMO_CAN_GO_HERE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ONLY_AMMO_CAN_GO_HERE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_REQUIRED_PROFICIENCY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_REQUIRED_PROFICIENCY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_EQUIP_WITH_TWOHANDED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_DUAL_WIELD => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DUAL_WIELD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_DOESNT_GO_INTO_BAG => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_CARRY_MORE_OF_THIS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_CARRY_MORE_OF_THIS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_CANT_STACK => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_CANT_BE_EQUIPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_BE_EQUIPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEMS_CANT_BE_SWAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEMS_CANT_BE_SWAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::SLOT_IS_EMPTY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::SLOT_IS_EMPTY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_NOT_FOUND => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_DROP_SOULBOUND => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DROP_SOULBOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OUT_OF_RANGE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OUT_OF_RANGE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::COULDNT_SPLIT_ITEMS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::COULDNT_SPLIT_ITEMS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MISSING_REAGENT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MISSING_REAGENT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_ENOUGH_MONEY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_ENOUGH_MONEY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_A_BAG => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_A_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::DONT_OWN_THAT_ITEM => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::DONT_OWN_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_EQUIP_ONLY1_QUIVER => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TOO_FAR_AWAY_FROM_BANK => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_FAR_AWAY_FROM_BANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_LOCKED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_LOCKED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_ARE_STUNNED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_STUNNED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::YOU_ARE_DEAD => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::YOU_ARE_DEAD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_DO_RIGHT_NOW => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_DO_RIGHT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::INT_BAG_ERROR => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INT_BAG_ERROR {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_EQUIP_ONLY1_BOLT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_BOLT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::STACKABLE_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::STACKABLE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::EQUIPPED_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::WRAPPED_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::WRAPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BOUND_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BOUND_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::UNIQUE_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::UNIQUE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAGS_CANT_BE_WRAPPED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAGS_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ALREADY_LOOTED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ALREADY_LOOTED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::INVENTORY_FULL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::INVENTORY_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BANK_FULL => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BANK_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL3 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_NOT_FOUND2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_NOT_FOUND2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_CANT_STACK2 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_CANT_STACK2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL4 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::ITEM_SOLD_OUT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::ITEM_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::OBJECT_IS_BUSY => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::OBJECT_IS_BUSY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NONE => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NONE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_IN_COMBAT => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_IN_COMBAT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::NOT_WHILE_DISARMED => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::NOT_WHILE_DISARMED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::BAG_FULL6 => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::BAG_FULL6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_EQUIP_RANK => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_RANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::CANT_EQUIP_REPUTATION => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::CANT_EQUIP_REPUTATION {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::TOO_MANY_SPECIAL_BAGS => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::TOO_MANY_SPECIAL_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
            InventoryResult::LOOT_CANT_LOOT_THAT_NOW => {
                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult::LOOT_CANT_LOOT_THAT_NOW {
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

impl SMSG_INVENTORY_CHANGE_FAILURE {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    OK,
    CANT_EQUIP_LEVEL_I {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
        required_level: u32,
    },
    CANT_EQUIP_SKILL {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_DOESNT_GO_TO_SLOT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BAG_FULL {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NONEMPTY_BAG_OVER_OTHER_BAG {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_TRADE_EQUIP_BAGS {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ONLY_AMMO_CAN_GO_HERE {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NO_REQUIRED_PROFICIENCY {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NO_EQUIPMENT_SLOT_AVAILABLE {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YOU_CAN_NEVER_USE_THAT_ITEM {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YOU_CAN_NEVER_USE_THAT_ITEM2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NO_EQUIPMENT_SLOT_AVAILABLE2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_EQUIP_WITH_TWOHANDED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_DUAL_WIELD {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_DOESNT_GO_INTO_BAG {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_DOESNT_GO_INTO_BAG2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_CARRY_MORE_OF_THIS {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NO_EQUIPMENT_SLOT_AVAILABLE3 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_CANT_STACK {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_CANT_BE_EQUIPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEMS_CANT_BE_SWAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    SLOT_IS_EMPTY {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_NOT_FOUND {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_DROP_SOULBOUND {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    OUT_OF_RANGE {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    TRIED_TO_SPLIT_MORE_THAN_COUNT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    COULDNT_SPLIT_ITEMS {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    MISSING_REAGENT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NOT_ENOUGH_MONEY {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NOT_A_BAG {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CAN_ONLY_DO_WITH_EMPTY_BAGS {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    DONT_OWN_THAT_ITEM {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CAN_EQUIP_ONLY1_QUIVER {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    MUST_PURCHASE_THAT_BAG_SLOT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    TOO_FAR_AWAY_FROM_BANK {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_LOCKED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YOU_ARE_STUNNED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    YOU_ARE_DEAD {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_DO_RIGHT_NOW {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    INT_BAG_ERROR {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CAN_EQUIP_ONLY1_BOLT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CAN_EQUIP_ONLY1_AMMOPOUCH {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    STACKABLE_CANT_BE_WRAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    EQUIPPED_CANT_BE_WRAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    WRAPPED_CANT_BE_WRAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BOUND_CANT_BE_WRAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    UNIQUE_CANT_BE_WRAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BAGS_CANT_BE_WRAPPED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ALREADY_LOOTED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    INVENTORY_FULL {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BANK_FULL {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_IS_CURRENTLY_SOLD_OUT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BAG_FULL3 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_NOT_FOUND2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_CANT_STACK2 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BAG_FULL4 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    ITEM_SOLD_OUT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    OBJECT_IS_BUSY {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NONE {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NOT_IN_COMBAT {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    NOT_WHILE_DISARMED {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    BAG_FULL6 {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_EQUIP_RANK {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    CANT_EQUIP_REPUTATION {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    TOO_MANY_SPECIAL_BAGS {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
    LOOT_CANT_LOOT_THAT_NOW {
        bag_type_subclass: u8,
        item1_guid: u64,
        item2_guid: u64,
    },
}

impl Default for SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::OK
    }
}

impl SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::OK => 0,
            Self::CANT_EQUIP_LEVEL_I { .. } => 1,
            Self::CANT_EQUIP_SKILL { .. } => 2,
            Self::ITEM_DOESNT_GO_TO_SLOT { .. } => 3,
            Self::BAG_FULL { .. } => 4,
            Self::NONEMPTY_BAG_OVER_OTHER_BAG { .. } => 5,
            Self::CANT_TRADE_EQUIP_BAGS { .. } => 6,
            Self::ONLY_AMMO_CAN_GO_HERE { .. } => 7,
            Self::NO_REQUIRED_PROFICIENCY { .. } => 8,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE { .. } => 9,
            Self::YOU_CAN_NEVER_USE_THAT_ITEM { .. } => 10,
            Self::YOU_CAN_NEVER_USE_THAT_ITEM2 { .. } => 11,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE2 { .. } => 12,
            Self::CANT_EQUIP_WITH_TWOHANDED { .. } => 13,
            Self::CANT_DUAL_WIELD { .. } => 14,
            Self::ITEM_DOESNT_GO_INTO_BAG { .. } => 15,
            Self::ITEM_DOESNT_GO_INTO_BAG2 { .. } => 16,
            Self::CANT_CARRY_MORE_OF_THIS { .. } => 17,
            Self::NO_EQUIPMENT_SLOT_AVAILABLE3 { .. } => 18,
            Self::ITEM_CANT_STACK { .. } => 19,
            Self::ITEM_CANT_BE_EQUIPPED { .. } => 20,
            Self::ITEMS_CANT_BE_SWAPPED { .. } => 21,
            Self::SLOT_IS_EMPTY { .. } => 22,
            Self::ITEM_NOT_FOUND { .. } => 23,
            Self::CANT_DROP_SOULBOUND { .. } => 24,
            Self::OUT_OF_RANGE { .. } => 25,
            Self::TRIED_TO_SPLIT_MORE_THAN_COUNT { .. } => 26,
            Self::COULDNT_SPLIT_ITEMS { .. } => 27,
            Self::MISSING_REAGENT { .. } => 28,
            Self::NOT_ENOUGH_MONEY { .. } => 29,
            Self::NOT_A_BAG { .. } => 30,
            Self::CAN_ONLY_DO_WITH_EMPTY_BAGS { .. } => 31,
            Self::DONT_OWN_THAT_ITEM { .. } => 32,
            Self::CAN_EQUIP_ONLY1_QUIVER { .. } => 33,
            Self::MUST_PURCHASE_THAT_BAG_SLOT { .. } => 34,
            Self::TOO_FAR_AWAY_FROM_BANK { .. } => 35,
            Self::ITEM_LOCKED { .. } => 36,
            Self::YOU_ARE_STUNNED { .. } => 37,
            Self::YOU_ARE_DEAD { .. } => 38,
            Self::CANT_DO_RIGHT_NOW { .. } => 39,
            Self::INT_BAG_ERROR { .. } => 40,
            Self::CAN_EQUIP_ONLY1_BOLT { .. } => 41,
            Self::CAN_EQUIP_ONLY1_AMMOPOUCH { .. } => 42,
            Self::STACKABLE_CANT_BE_WRAPPED { .. } => 43,
            Self::EQUIPPED_CANT_BE_WRAPPED { .. } => 44,
            Self::WRAPPED_CANT_BE_WRAPPED { .. } => 45,
            Self::BOUND_CANT_BE_WRAPPED { .. } => 46,
            Self::UNIQUE_CANT_BE_WRAPPED { .. } => 47,
            Self::BAGS_CANT_BE_WRAPPED { .. } => 48,
            Self::ALREADY_LOOTED { .. } => 49,
            Self::INVENTORY_FULL { .. } => 50,
            Self::BANK_FULL { .. } => 51,
            Self::ITEM_IS_CURRENTLY_SOLD_OUT { .. } => 52,
            Self::BAG_FULL3 { .. } => 53,
            Self::ITEM_NOT_FOUND2 { .. } => 54,
            Self::ITEM_CANT_STACK2 { .. } => 55,
            Self::BAG_FULL4 { .. } => 56,
            Self::ITEM_SOLD_OUT { .. } => 57,
            Self::OBJECT_IS_BUSY { .. } => 58,
            Self::NONE { .. } => 59,
            Self::NOT_IN_COMBAT { .. } => 60,
            Self::NOT_WHILE_DISARMED { .. } => 61,
            Self::BAG_FULL6 { .. } => 62,
            Self::CANT_EQUIP_RANK { .. } => 63,
            Self::CANT_EQUIP_REPUTATION { .. } => 64,
            Self::TOO_MANY_SPECIAL_BAGS { .. } => 65,
            Self::LOOT_CANT_LOOT_THAT_NOW { .. } => 66,
        }
    }

}

impl SMSG_INVENTORY_CHANGE_FAILURE_InventoryResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::OK => {
                1
            }
            Self::CANT_EQUIP_LEVEL_I {
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
            Self::CANT_EQUIP_SKILL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_DOESNT_GO_TO_SLOT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BAG_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NONEMPTY_BAG_OVER_OTHER_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_TRADE_EQUIP_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ONLY_AMMO_CAN_GO_HERE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NO_REQUIRED_PROFICIENCY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NO_EQUIPMENT_SLOT_AVAILABLE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YOU_CAN_NEVER_USE_THAT_ITEM {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_EQUIP_WITH_TWOHANDED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_DUAL_WIELD {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_DOESNT_GO_INTO_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_DOESNT_GO_INTO_BAG2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_CARRY_MORE_OF_THIS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_CANT_STACK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_CANT_BE_EQUIPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEMS_CANT_BE_SWAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::SLOT_IS_EMPTY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_NOT_FOUND {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_DROP_SOULBOUND {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::OUT_OF_RANGE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::COULDNT_SPLIT_ITEMS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::MISSING_REAGENT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NOT_ENOUGH_MONEY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NOT_A_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::DONT_OWN_THAT_ITEM {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CAN_EQUIP_ONLY1_QUIVER {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::MUST_PURCHASE_THAT_BAG_SLOT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::TOO_FAR_AWAY_FROM_BANK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_LOCKED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YOU_ARE_STUNNED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::YOU_ARE_DEAD {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_DO_RIGHT_NOW {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::INT_BAG_ERROR {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CAN_EQUIP_ONLY1_BOLT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CAN_EQUIP_ONLY1_AMMOPOUCH {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::STACKABLE_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::EQUIPPED_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::WRAPPED_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BOUND_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::UNIQUE_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BAGS_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ALREADY_LOOTED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::INVENTORY_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BANK_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_IS_CURRENTLY_SOLD_OUT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BAG_FULL3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_NOT_FOUND2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_CANT_STACK2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BAG_FULL4 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::ITEM_SOLD_OUT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::OBJECT_IS_BUSY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NONE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NOT_IN_COMBAT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::NOT_WHILE_DISARMED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::BAG_FULL6 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_EQUIP_RANK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::CANT_EQUIP_REPUTATION {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::TOO_MANY_SPECIAL_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                1
                + 1 // bag_type_subclass: u8
                + 8 // item1_guid: u64
                + 8 // item2_guid: u64
            }
            Self::LOOT_CANT_LOOT_THAT_NOW {
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

