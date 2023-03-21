use std::io::{Read, Write};

use crate::wrath::{
    AuctionCommandAction, AuctionCommandResult, InventoryResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm:109`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm#L109):
/// ```text
/// smsg SMSG_AUCTION_COMMAND_RESULT = 0x025B {
///     u32 auction_id;
///     AuctionCommandAction action;
///     AuctionCommandResult result;
///     if (result == ERR_INVENTORY) {
///         InventoryResult inventory_result;
///     }
/// }
/// ```
pub struct SMSG_AUCTION_COMMAND_RESULT {
    pub auction_id: u32,
    pub action: AuctionCommandAction,
    pub result: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult,
}

impl crate::Message for SMSG_AUCTION_COMMAND_RESULT {
    const OPCODE: u32 = 0x025b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // action: AuctionCommandAction
        w.write_all(&u32::from(self.action.as_int()).to_le_bytes())?;

        // result: AuctionCommandResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        match &self.result {
            SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrInventory {
                inventory_result,
            } => {
                // inventory_result: InventoryResult
                w.write_all(&u8::from(inventory_result.as_int()).to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(12..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025B, size: body_size as u32 });
        }

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // action: AuctionCommandAction
        let action: AuctionCommandAction = crate::util::read_u32_le(&mut r)?.try_into()?;

        // result: AuctionCommandResult
        let result: AuctionCommandResult = crate::util::read_u32_le(&mut r)?.try_into()?;

        let result_if = match result {
            AuctionCommandResult::Ok => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::Ok,
            AuctionCommandResult::ErrInventory => {
                // inventory_result: InventoryResult
                let inventory_result: InventoryResult = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrInventory {
                    inventory_result,
                }
            }
            AuctionCommandResult::ErrDatabase => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrDatabase,
            AuctionCommandResult::ErrNotEnoughMoney => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrNotEnoughMoney,
            AuctionCommandResult::ErrItemNotFound => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrItemNotFound,
            AuctionCommandResult::ErrHigherBid => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrHigherBid,
            AuctionCommandResult::ErrBidIncrement => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrBidIncrement,
            AuctionCommandResult::ErrBidOwn => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrBidOwn,
            AuctionCommandResult::ErrRestrictedAccount => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrRestrictedAccount,
        };

        Ok(Self {
            auction_id,
            action,
            result: result_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUCTION_COMMAND_RESULT {}

impl SMSG_AUCTION_COMMAND_RESULT {
    pub(crate) const fn size(&self) -> usize {
        4 // auction_id: u32
        + 4 // action: AuctionCommandAction
        + self.result.size() // result: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    Ok,
    ErrInventory {
        inventory_result: InventoryResult,
    },
    ErrDatabase,
    ErrNotEnoughMoney,
    ErrItemNotFound,
    ErrHigherBid,
    ErrBidIncrement,
    ErrBidOwn,
    ErrRestrictedAccount,
}

impl Default for SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Ok
    }
}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0,
            Self::ErrInventory { .. } => 1,
            Self::ErrDatabase => 2,
            Self::ErrNotEnoughMoney => 3,
            Self::ErrItemNotFound => 4,
            Self::ErrHigherBid => 5,
            Self::ErrBidIncrement => 7,
            Self::ErrBidOwn => 10,
            Self::ErrRestrictedAccount => 13,
        }
    }

}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Ok => {
                4
            }
            Self::ErrInventory {
                inventory_result,
            } => {
                4
                + 1 // inventory_result: InventoryResult
            }
            Self::ErrDatabase => {
                4
            }
            Self::ErrNotEnoughMoney => {
                4
            }
            Self::ErrItemNotFound => {
                4
            }
            Self::ErrHigherBid => {
                4
            }
            Self::ErrBidIncrement => {
                4
            }
            Self::ErrBidOwn => {
                4
            }
            Self::ErrRestrictedAccount => {
                4
            }
        }
    }
}

