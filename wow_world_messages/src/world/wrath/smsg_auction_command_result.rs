use std::io::{Read, Write};

use crate::wrath::{
    AuctionCommandAction, AuctionCommandResult, InventoryResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm:109`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_command_result.wowm#L109):
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

#[cfg(feature = "print-testcase")]
impl SMSG_AUCTION_COMMAND_RESULT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_COMMAND_RESULT {{").unwrap();
        // Members
        writeln!(s, "    auction_id = {};", self.auction_id).unwrap();
        writeln!(s, "    action = {};", self.action.as_test_case_value()).unwrap();
        writeln!(s, "    result = {};", crate::vanilla::AuctionCommandResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::wrath::SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrInventory {
                inventory_result,
            } => {
                writeln!(s, "    inventory_result = {};", inventory_result.as_test_case_value()).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 603_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "action", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");
        match &self.result {
            crate::wrath::SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrInventory {
                inventory_result,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "inventory_result", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_AUCTION_COMMAND_RESULT {}
impl crate::Message for SMSG_AUCTION_COMMAND_RESULT {
    const OPCODE: u32 = 0x025b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_AUCTION_COMMAND_RESULT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // action: AuctionCommandAction
        w.write_all(&(self.action.as_int().to_le_bytes()))?;

        // result: AuctionCommandResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrInventory {
                inventory_result,
            } => {
                // inventory_result: InventoryResult
                w.write_all(&(inventory_result.as_int().to_le_bytes()))?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025B, size: body_size });
        }

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // action: AuctionCommandAction
        let action = crate::util::read_u32_le(&mut r)?.try_into()?;

        // result: AuctionCommandResult
        let result = crate::util::read_u32_le(&mut r)?.try_into()?;

        let result_if = match result {
            AuctionCommandResult::Ok => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::Ok,
            AuctionCommandResult::ErrInventory => {
                // inventory_result: InventoryResult
                let inventory_result = crate::util::read_u8_le(&mut r)?.try_into()?;

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

impl std::fmt::Display for SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::ErrInventory{ .. } => f.write_str("ErrInventory"),
            Self::ErrDatabase => f.write_str("ErrDatabase"),
            Self::ErrNotEnoughMoney => f.write_str("ErrNotEnoughMoney"),
            Self::ErrItemNotFound => f.write_str("ErrItemNotFound"),
            Self::ErrHigherBid => f.write_str("ErrHigherBid"),
            Self::ErrBidIncrement => f.write_str("ErrBidIncrement"),
            Self::ErrBidOwn => f.write_str("ErrBidOwn"),
            Self::ErrRestrictedAccount => f.write_str("ErrRestrictedAccount"),
        }
    }
}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::ErrInventory {
                ..
            } => {
                4
                + 1 // inventory_result: InventoryResult
            }
            _ => 4,
        }
    }
}

