use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{InventoryResult, InventoryResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INVENTORY_CHANGE_FAILURE {
    pub result: SMSG_INVENTORY_CHANGE_FAILUREInventoryResult,
}

impl ServerMessageWrite for SMSG_INVENTORY_CHANGE_FAILURE {}

impl MessageBody for SMSG_INVENTORY_CHANGE_FAILURE {
    const OPCODE: u16 = 0x0112;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_INVENTORY_CHANGE_FAILUREError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: InventoryResult
        let result = InventoryResult::read(r)?;

        let result_if = match result {
            InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK,
            InventoryResult::CANT_EQUIP_LEVEL_I => {
                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                }
            }
        };

        let result_if = match result {
            InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK,
            InventoryResult::CANT_EQUIP_LEVEL_I => {
                // required_level: u32
                let required_level = crate::util::read_u32_le(r)?;

                // item1_guid: u64
                let item1_guid = crate::util::read_u64_le(r)?;

                // item2_guid: u64
                let item2_guid = crate::util::read_u64_le(r)?;

                // bag_type_subclass: u8
                let bag_type_subclass = crate::util::read_u8_le(r)?;

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
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

                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: InventoryResult
        self.result.write(w)?;

        match &self.result {
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => {}
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
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
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                bag_type_subclass,
                item1_guid,
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
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => {}
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
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
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                bag_type_subclass,
                item1_guid,
                item2_guid,
            } => {
                // item1_guid: u64
                w.write_all(&item1_guid.to_le_bytes())?;

                // item2_guid: u64
                w.write_all(&item2_guid.to_le_bytes())?;

                // bag_type_subclass: u8
                w.write_all(&bag_type_subclass.to_le_bytes())?;

            }
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                bag_type_subclass,
                item1_guid,
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

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: InventoryResult
            let result = InventoryResult::tokio_read(r).await?;

            let result_if = match result {
                InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK,
                InventoryResult::CANT_EQUIP_LEVEL_I => {
                    // required_level: u32
                    let required_level = crate::util::tokio_read_u32_le(r).await?;

                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                        required_level,
                    }
                }
                InventoryResult::CANT_EQUIP_SKILL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_TO_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_TRADE_EQUIP_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ONLY_AMMO_CAN_GO_HERE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_REQUIRED_PROFICIENCY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_WITH_TWOHANDED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DUAL_WIELD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_CARRY_MORE_OF_THIS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_BE_EQUIPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEMS_CANT_BE_SWAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::SLOT_IS_EMPTY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DROP_SOULBOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OUT_OF_RANGE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::COULDNT_SPLIT_ITEMS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MISSING_REAGENT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_ENOUGH_MONEY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_A_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::DONT_OWN_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_QUIVER => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_FAR_AWAY_FROM_BANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_LOCKED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_STUNNED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_DEAD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DO_RIGHT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INT_BAG_ERROR => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_BOLT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::STACKABLE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::EQUIPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::WRAPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BOUND_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::UNIQUE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAGS_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ALREADY_LOOTED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INVENTORY_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BANK_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL4 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OBJECT_IS_BUSY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_IN_COMBAT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_WHILE_DISARMED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL6 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_RANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_REPUTATION => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_MANY_SPECIAL_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::LOOT_CANT_LOOT_THAT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
            };

            let result_if = match result {
                InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK,
                InventoryResult::CANT_EQUIP_LEVEL_I => {
                    // required_level: u32
                    let required_level = crate::util::tokio_read_u32_le(r).await?;

                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                        required_level,
                    }
                }
                InventoryResult::CANT_EQUIP_SKILL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_TO_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_TRADE_EQUIP_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ONLY_AMMO_CAN_GO_HERE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_REQUIRED_PROFICIENCY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_WITH_TWOHANDED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DUAL_WIELD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_CARRY_MORE_OF_THIS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_BE_EQUIPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEMS_CANT_BE_SWAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::SLOT_IS_EMPTY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DROP_SOULBOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OUT_OF_RANGE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::COULDNT_SPLIT_ITEMS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MISSING_REAGENT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_ENOUGH_MONEY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_A_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::DONT_OWN_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_QUIVER => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_FAR_AWAY_FROM_BANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_LOCKED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_STUNNED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_DEAD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DO_RIGHT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INT_BAG_ERROR => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_BOLT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::STACKABLE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::EQUIPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::WRAPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BOUND_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::UNIQUE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAGS_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ALREADY_LOOTED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INVENTORY_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BANK_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL4 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OBJECT_IS_BUSY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_IN_COMBAT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_WHILE_DISARMED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL6 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_RANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_REPUTATION => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_MANY_SPECIAL_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::LOOT_CANT_LOOT_THAT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::tokio_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::tokio_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::tokio_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: InventoryResult
            self.result.tokio_write(w).await?;

            match &self.result {
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => {}
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                } => {
                    // required_level: u32
                    w.write_all(&required_level.to_le_bytes()).await?;

                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
            }

            match &self.result {
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => {}
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                } => {
                    // required_level: u32
                    w.write_all(&required_level.to_le_bytes()).await?;

                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: InventoryResult
            let result = InventoryResult::astd_read(r).await?;

            let result_if = match result {
                InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK,
                InventoryResult::CANT_EQUIP_LEVEL_I => {
                    // required_level: u32
                    let required_level = crate::util::astd_read_u32_le(r).await?;

                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                        required_level,
                    }
                }
                InventoryResult::CANT_EQUIP_SKILL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_TO_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_TRADE_EQUIP_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ONLY_AMMO_CAN_GO_HERE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_REQUIRED_PROFICIENCY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_WITH_TWOHANDED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DUAL_WIELD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_CARRY_MORE_OF_THIS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_BE_EQUIPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEMS_CANT_BE_SWAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::SLOT_IS_EMPTY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DROP_SOULBOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OUT_OF_RANGE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::COULDNT_SPLIT_ITEMS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MISSING_REAGENT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_ENOUGH_MONEY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_A_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::DONT_OWN_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_QUIVER => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_FAR_AWAY_FROM_BANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_LOCKED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_STUNNED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_DEAD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DO_RIGHT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INT_BAG_ERROR => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_BOLT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::STACKABLE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::EQUIPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::WRAPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BOUND_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::UNIQUE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAGS_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ALREADY_LOOTED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INVENTORY_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BANK_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL4 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OBJECT_IS_BUSY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_IN_COMBAT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_WHILE_DISARMED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL6 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_RANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_REPUTATION => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_MANY_SPECIAL_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::LOOT_CANT_LOOT_THAT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
            };

            let result_if = match result {
                InventoryResult::OK => SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK,
                InventoryResult::CANT_EQUIP_LEVEL_I => {
                    // required_level: u32
                    let required_level = crate::util::astd_read_u32_le(r).await?;

                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                        required_level,
                    }
                }
                InventoryResult::CANT_EQUIP_SKILL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_TO_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_TRADE_EQUIP_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ONLY_AMMO_CAN_GO_HERE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_REQUIRED_PROFICIENCY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_WITH_TWOHANDED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DUAL_WIELD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_CARRY_MORE_OF_THIS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_BE_EQUIPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEMS_CANT_BE_SWAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::SLOT_IS_EMPTY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DROP_SOULBOUND => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OUT_OF_RANGE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::COULDNT_SPLIT_ITEMS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MISSING_REAGENT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_ENOUGH_MONEY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_A_BAG => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::DONT_OWN_THAT_ITEM => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_QUIVER => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_FAR_AWAY_FROM_BANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_LOCKED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_STUNNED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::YOU_ARE_DEAD => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_DO_RIGHT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INT_BAG_ERROR => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_BOLT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::STACKABLE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::EQUIPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::WRAPPED_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BOUND_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::UNIQUE_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAGS_CANT_BE_WRAPPED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ALREADY_LOOTED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::INVENTORY_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BANK_FULL => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL3 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_NOT_FOUND2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_CANT_STACK2 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL4 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::ITEM_SOLD_OUT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::OBJECT_IS_BUSY => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NONE => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_IN_COMBAT => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::NOT_WHILE_DISARMED => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::BAG_FULL6 => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_RANK => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::CANT_EQUIP_REPUTATION => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::TOO_MANY_SPECIAL_BAGS => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
                InventoryResult::LOOT_CANT_LOOT_THAT_NOW => {
                    // item1_guid: u64
                    let item1_guid = crate::util::astd_read_u64_le(r).await?;

                    // item2_guid: u64
                    let item2_guid = crate::util::astd_read_u64_le(r).await?;

                    // bag_type_subclass: u8
                    let bag_type_subclass = crate::util::astd_read_u8_le(r).await?;

                    SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                        bag_type_subclass,
                        item1_guid,
                        item2_guid,
                    }
                }
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: InventoryResult
            self.result.astd_write(w).await?;

            match &self.result {
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => {}
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                } => {
                    // required_level: u32
                    w.write_all(&required_level.to_le_bytes()).await?;

                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
            }

            match &self.result {
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => {}
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                    required_level,
                } => {
                    // required_level: u32
                    w.write_all(&required_level.to_le_bytes()).await?;

                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
                SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW {
                    bag_type_subclass,
                    item1_guid,
                    item2_guid,
                } => {
                    // item1_guid: u64
                    w.write_all(&item1_guid.to_le_bytes()).await?;

                    // item2_guid: u64
                    w.write_all(&item2_guid.to_le_bytes()).await?;

                    // bag_type_subclass: u8
                    w.write_all(&bag_type_subclass.to_le_bytes()).await?;

                }
            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_INVENTORY_CHANGE_FAILURE {
    fn size(&self) -> usize {
        0
        + self.result.size() // result: SMSG_INVENTORY_CHANGE_FAILUREInventoryResult
    }
}

impl MaximumPossibleSized for SMSG_INVENTORY_CHANGE_FAILURE {
    fn maximum_possible_size() -> usize {
        0
        + 22 // result: SMSG_INVENTORY_CHANGE_FAILUREInventoryResult
    }
}

#[derive(Debug)]
pub enum SMSG_INVENTORY_CHANGE_FAILUREError {
    Io(std::io::Error),
    InventoryResult(InventoryResultError),
}

impl std::error::Error for SMSG_INVENTORY_CHANGE_FAILUREError {}
impl std::fmt::Display for SMSG_INVENTORY_CHANGE_FAILUREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InventoryResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_INVENTORY_CHANGE_FAILUREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<InventoryResultError> for SMSG_INVENTORY_CHANGE_FAILUREError {
    fn from(e: InventoryResultError) -> Self {
        Self::InventoryResult(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_INVENTORY_CHANGE_FAILUREInventoryResult {
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

impl From<&InventoryResult> for SMSG_INVENTORY_CHANGE_FAILUREInventoryResult {
    fn from(e: &InventoryResult) -> Self {
        match &e {
            InventoryResult::OK => Self::OK,
            InventoryResult::CANT_EQUIP_LEVEL_I => Self::CANT_EQUIP_LEVEL_I {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
                required_level: Default::default(),
            },
            InventoryResult::CANT_EQUIP_SKILL => Self::CANT_EQUIP_SKILL {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_DOESNT_GO_TO_SLOT => Self::ITEM_DOESNT_GO_TO_SLOT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BAG_FULL => Self::BAG_FULL {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG => Self::NONEMPTY_BAG_OVER_OTHER_BAG {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_TRADE_EQUIP_BAGS => Self::CANT_TRADE_EQUIP_BAGS {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ONLY_AMMO_CAN_GO_HERE => Self::ONLY_AMMO_CAN_GO_HERE {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NO_REQUIRED_PROFICIENCY => Self::NO_REQUIRED_PROFICIENCY {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE => Self::NO_EQUIPMENT_SLOT_AVAILABLE {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM => Self::YOU_CAN_NEVER_USE_THAT_ITEM {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 => Self::YOU_CAN_NEVER_USE_THAT_ITEM2 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 => Self::NO_EQUIPMENT_SLOT_AVAILABLE2 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_EQUIP_WITH_TWOHANDED => Self::CANT_EQUIP_WITH_TWOHANDED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_DUAL_WIELD => Self::CANT_DUAL_WIELD {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_DOESNT_GO_INTO_BAG => Self::ITEM_DOESNT_GO_INTO_BAG {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_DOESNT_GO_INTO_BAG2 => Self::ITEM_DOESNT_GO_INTO_BAG2 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_CARRY_MORE_OF_THIS => Self::CANT_CARRY_MORE_OF_THIS {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 => Self::NO_EQUIPMENT_SLOT_AVAILABLE3 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_CANT_STACK => Self::ITEM_CANT_STACK {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_CANT_BE_EQUIPPED => Self::ITEM_CANT_BE_EQUIPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEMS_CANT_BE_SWAPPED => Self::ITEMS_CANT_BE_SWAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::SLOT_IS_EMPTY => Self::SLOT_IS_EMPTY {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_NOT_FOUND => Self::ITEM_NOT_FOUND {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_DROP_SOULBOUND => Self::CANT_DROP_SOULBOUND {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::OUT_OF_RANGE => Self::OUT_OF_RANGE {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT => Self::TRIED_TO_SPLIT_MORE_THAN_COUNT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::COULDNT_SPLIT_ITEMS => Self::COULDNT_SPLIT_ITEMS {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::MISSING_REAGENT => Self::MISSING_REAGENT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NOT_ENOUGH_MONEY => Self::NOT_ENOUGH_MONEY {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NOT_A_BAG => Self::NOT_A_BAG {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS => Self::CAN_ONLY_DO_WITH_EMPTY_BAGS {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::DONT_OWN_THAT_ITEM => Self::DONT_OWN_THAT_ITEM {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CAN_EQUIP_ONLY1_QUIVER => Self::CAN_EQUIP_ONLY1_QUIVER {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::MUST_PURCHASE_THAT_BAG_SLOT => Self::MUST_PURCHASE_THAT_BAG_SLOT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::TOO_FAR_AWAY_FROM_BANK => Self::TOO_FAR_AWAY_FROM_BANK {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_LOCKED => Self::ITEM_LOCKED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::YOU_ARE_STUNNED => Self::YOU_ARE_STUNNED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::YOU_ARE_DEAD => Self::YOU_ARE_DEAD {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_DO_RIGHT_NOW => Self::CANT_DO_RIGHT_NOW {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::INT_BAG_ERROR => Self::INT_BAG_ERROR {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CAN_EQUIP_ONLY1_BOLT => Self::CAN_EQUIP_ONLY1_BOLT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH => Self::CAN_EQUIP_ONLY1_AMMOPOUCH {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::STACKABLE_CANT_BE_WRAPPED => Self::STACKABLE_CANT_BE_WRAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::EQUIPPED_CANT_BE_WRAPPED => Self::EQUIPPED_CANT_BE_WRAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::WRAPPED_CANT_BE_WRAPPED => Self::WRAPPED_CANT_BE_WRAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BOUND_CANT_BE_WRAPPED => Self::BOUND_CANT_BE_WRAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::UNIQUE_CANT_BE_WRAPPED => Self::UNIQUE_CANT_BE_WRAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BAGS_CANT_BE_WRAPPED => Self::BAGS_CANT_BE_WRAPPED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ALREADY_LOOTED => Self::ALREADY_LOOTED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::INVENTORY_FULL => Self::INVENTORY_FULL {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BANK_FULL => Self::BANK_FULL {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT => Self::ITEM_IS_CURRENTLY_SOLD_OUT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BAG_FULL3 => Self::BAG_FULL3 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_NOT_FOUND2 => Self::ITEM_NOT_FOUND2 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_CANT_STACK2 => Self::ITEM_CANT_STACK2 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BAG_FULL4 => Self::BAG_FULL4 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::ITEM_SOLD_OUT => Self::ITEM_SOLD_OUT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::OBJECT_IS_BUSY => Self::OBJECT_IS_BUSY {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NONE => Self::NONE {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NOT_IN_COMBAT => Self::NOT_IN_COMBAT {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::NOT_WHILE_DISARMED => Self::NOT_WHILE_DISARMED {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::BAG_FULL6 => Self::BAG_FULL6 {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_EQUIP_RANK => Self::CANT_EQUIP_RANK {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::CANT_EQUIP_REPUTATION => Self::CANT_EQUIP_REPUTATION {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::TOO_MANY_SPECIAL_BAGS => Self::TOO_MANY_SPECIAL_BAGS {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
            InventoryResult::LOOT_CANT_LOOT_THAT_NOW => Self::LOOT_CANT_LOOT_THAT_NOW {
                bag_type_subclass: Default::default(),
                item1_guid: Default::default(),
                item2_guid: Default::default(),
            },
        }
    }
}

impl From<&SMSG_INVENTORY_CHANGE_FAILUREInventoryResult> for InventoryResult {
    fn from(v: &SMSG_INVENTORY_CHANGE_FAILUREInventoryResult) -> Self {
        match &v {
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OK => Self::OK,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_LEVEL_I { .. } => Self::CANT_EQUIP_LEVEL_I,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_SKILL { .. } => Self::CANT_EQUIP_SKILL,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_TO_SLOT { .. } => Self::ITEM_DOESNT_GO_TO_SLOT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL { .. } => Self::BAG_FULL,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONEMPTY_BAG_OVER_OTHER_BAG { .. } => Self::NONEMPTY_BAG_OVER_OTHER_BAG,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_TRADE_EQUIP_BAGS { .. } => Self::CANT_TRADE_EQUIP_BAGS,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ONLY_AMMO_CAN_GO_HERE { .. } => Self::ONLY_AMMO_CAN_GO_HERE,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_REQUIRED_PROFICIENCY { .. } => Self::NO_REQUIRED_PROFICIENCY,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE { .. } => Self::NO_EQUIPMENT_SLOT_AVAILABLE,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM { .. } => Self::YOU_CAN_NEVER_USE_THAT_ITEM,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_CAN_NEVER_USE_THAT_ITEM2 { .. } => Self::YOU_CAN_NEVER_USE_THAT_ITEM2,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE2 { .. } => Self::NO_EQUIPMENT_SLOT_AVAILABLE2,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_WITH_TWOHANDED { .. } => Self::CANT_EQUIP_WITH_TWOHANDED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DUAL_WIELD { .. } => Self::CANT_DUAL_WIELD,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG { .. } => Self::ITEM_DOESNT_GO_INTO_BAG,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_DOESNT_GO_INTO_BAG2 { .. } => Self::ITEM_DOESNT_GO_INTO_BAG2,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_CARRY_MORE_OF_THIS { .. } => Self::CANT_CARRY_MORE_OF_THIS,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NO_EQUIPMENT_SLOT_AVAILABLE3 { .. } => Self::NO_EQUIPMENT_SLOT_AVAILABLE3,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK { .. } => Self::ITEM_CANT_STACK,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_BE_EQUIPPED { .. } => Self::ITEM_CANT_BE_EQUIPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEMS_CANT_BE_SWAPPED { .. } => Self::ITEMS_CANT_BE_SWAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::SLOT_IS_EMPTY { .. } => Self::SLOT_IS_EMPTY,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND { .. } => Self::ITEM_NOT_FOUND,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DROP_SOULBOUND { .. } => Self::CANT_DROP_SOULBOUND,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OUT_OF_RANGE { .. } => Self::OUT_OF_RANGE,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TRIED_TO_SPLIT_MORE_THAN_COUNT { .. } => Self::TRIED_TO_SPLIT_MORE_THAN_COUNT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::COULDNT_SPLIT_ITEMS { .. } => Self::COULDNT_SPLIT_ITEMS,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MISSING_REAGENT { .. } => Self::MISSING_REAGENT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_ENOUGH_MONEY { .. } => Self::NOT_ENOUGH_MONEY,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_A_BAG { .. } => Self::NOT_A_BAG,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_ONLY_DO_WITH_EMPTY_BAGS { .. } => Self::CAN_ONLY_DO_WITH_EMPTY_BAGS,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::DONT_OWN_THAT_ITEM { .. } => Self::DONT_OWN_THAT_ITEM,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_QUIVER { .. } => Self::CAN_EQUIP_ONLY1_QUIVER,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::MUST_PURCHASE_THAT_BAG_SLOT { .. } => Self::MUST_PURCHASE_THAT_BAG_SLOT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_FAR_AWAY_FROM_BANK { .. } => Self::TOO_FAR_AWAY_FROM_BANK,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_LOCKED { .. } => Self::ITEM_LOCKED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_STUNNED { .. } => Self::YOU_ARE_STUNNED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::YOU_ARE_DEAD { .. } => Self::YOU_ARE_DEAD,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_DO_RIGHT_NOW { .. } => Self::CANT_DO_RIGHT_NOW,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INT_BAG_ERROR { .. } => Self::INT_BAG_ERROR,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_BOLT { .. } => Self::CAN_EQUIP_ONLY1_BOLT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CAN_EQUIP_ONLY1_AMMOPOUCH { .. } => Self::CAN_EQUIP_ONLY1_AMMOPOUCH,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::STACKABLE_CANT_BE_WRAPPED { .. } => Self::STACKABLE_CANT_BE_WRAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::EQUIPPED_CANT_BE_WRAPPED { .. } => Self::EQUIPPED_CANT_BE_WRAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::WRAPPED_CANT_BE_WRAPPED { .. } => Self::WRAPPED_CANT_BE_WRAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BOUND_CANT_BE_WRAPPED { .. } => Self::BOUND_CANT_BE_WRAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::UNIQUE_CANT_BE_WRAPPED { .. } => Self::UNIQUE_CANT_BE_WRAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAGS_CANT_BE_WRAPPED { .. } => Self::BAGS_CANT_BE_WRAPPED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ALREADY_LOOTED { .. } => Self::ALREADY_LOOTED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::INVENTORY_FULL { .. } => Self::INVENTORY_FULL,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BANK_FULL { .. } => Self::BANK_FULL,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_IS_CURRENTLY_SOLD_OUT { .. } => Self::ITEM_IS_CURRENTLY_SOLD_OUT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL3 { .. } => Self::BAG_FULL3,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_NOT_FOUND2 { .. } => Self::ITEM_NOT_FOUND2,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_CANT_STACK2 { .. } => Self::ITEM_CANT_STACK2,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL4 { .. } => Self::BAG_FULL4,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::ITEM_SOLD_OUT { .. } => Self::ITEM_SOLD_OUT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::OBJECT_IS_BUSY { .. } => Self::OBJECT_IS_BUSY,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NONE { .. } => Self::NONE,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_IN_COMBAT { .. } => Self::NOT_IN_COMBAT,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::NOT_WHILE_DISARMED { .. } => Self::NOT_WHILE_DISARMED,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::BAG_FULL6 { .. } => Self::BAG_FULL6,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_RANK { .. } => Self::CANT_EQUIP_RANK,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::CANT_EQUIP_REPUTATION { .. } => Self::CANT_EQUIP_REPUTATION,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::TOO_MANY_SPECIAL_BAGS { .. } => Self::TOO_MANY_SPECIAL_BAGS,
            SMSG_INVENTORY_CHANGE_FAILUREInventoryResult::LOOT_CANT_LOOT_THAT_NOW { .. } => Self::LOOT_CANT_LOOT_THAT_NOW,
        }
    }
}

impl Default for SMSG_INVENTORY_CHANGE_FAILUREInventoryResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::OK
    }
}

impl SMSG_INVENTORY_CHANGE_FAILUREInventoryResult {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    pub(crate) fn as_int(&self) -> u8 {
        let a: InventoryResult = self.into();
        a.as_int() as u8
    }

}

impl VariableSized for SMSG_INVENTORY_CHANGE_FAILUREInventoryResult {
    fn size(&self) -> usize {
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

impl MaximumPossibleSized for SMSG_INVENTORY_CHANGE_FAILUREInventoryResult {
    fn maximum_possible_size() -> usize {
        22
    }
}

