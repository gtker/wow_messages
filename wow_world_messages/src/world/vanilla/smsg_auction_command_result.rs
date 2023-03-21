use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    AuctionCommandAction, AuctionCommandResult, AuctionCommandResultTwo, InventoryResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm:47`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_auction_command_result.wowm#L47):
/// ```text
/// smsg SMSG_AUCTION_COMMAND_RESULT = 0x025B {
///     u32 auction_id;
///     AuctionCommandAction action;
///     if (action == BID_PLACED) {
///         AuctionCommandResult result;
///         if (result == OK) {
///             u32 auction_outbid1;
///         }
///         else if (result == ERR_INVENTORY) {
///             InventoryResult inventory_result;
///         }
///         else if (result == ERR_HIGHER_BID) {
///             Guid higher_bidder;
///             u32 new_bid;
///             u32 auction_outbid2;
///         }
///     }
///     else {
///         AuctionCommandResultTwo result2;
///         if (result2 == ERR_INVENTORY) {
///             InventoryResult inventory_result2;
///         }
///         else if (result2 == ERR_HIGHER_BID) {
///             Guid higher_bidder2;
///             u32 new_bid2;
///             u32 auction_outbid3;
///         }
///     }
/// }
/// ```
pub struct SMSG_AUCTION_COMMAND_RESULT {
    pub auction_id: u32,
    pub action: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction,
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

        match &self.action {
            SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction::Started {
                result2,
            } => {
                // result2: AuctionCommandResultTwo
                w.write_all(&u32::from(result2.as_int()).to_le_bytes())?;

                match &result2 {
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrInventory {
                        inventory_result2,
                    } => {
                        // inventory_result2: InventoryResult
                        w.write_all(&u8::from(inventory_result2.as_int()).to_le_bytes())?;

                    }
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrHigherBid {
                        auction_outbid3,
                        higher_bidder2,
                        new_bid2,
                    } => {
                        // higher_bidder2: Guid
                        w.write_all(&higher_bidder2.guid().to_le_bytes())?;

                        // new_bid2: u32
                        w.write_all(&new_bid2.to_le_bytes())?;

                        // auction_outbid3: u32
                        w.write_all(&auction_outbid3.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction::Removed {
                result2,
            } => {
                // result2: AuctionCommandResultTwo
                w.write_all(&u32::from(result2.as_int()).to_le_bytes())?;

                match &result2 {
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrInventory {
                        inventory_result2,
                    } => {
                        // inventory_result2: InventoryResult
                        w.write_all(&u8::from(inventory_result2.as_int()).to_le_bytes())?;

                    }
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrHigherBid {
                        auction_outbid3,
                        higher_bidder2,
                        new_bid2,
                    } => {
                        // higher_bidder2: Guid
                        w.write_all(&higher_bidder2.guid().to_le_bytes())?;

                        // new_bid2: u32
                        w.write_all(&new_bid2.to_le_bytes())?;

                        // auction_outbid3: u32
                        w.write_all(&auction_outbid3.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
            SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction::BidPlaced {
                result,
            } => {
                // result: AuctionCommandResult
                w.write_all(&u32::from(result.as_int()).to_le_bytes())?;

                match &result {
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::Ok {
                        auction_outbid1,
                    } => {
                        // auction_outbid1: u32
                        w.write_all(&auction_outbid1.to_le_bytes())?;

                    }
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrInventory {
                        inventory_result,
                    } => {
                        // inventory_result: InventoryResult
                        w.write_all(&u8::from(inventory_result.as_int()).to_le_bytes())?;

                    }
                    SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrHigherBid {
                        auction_outbid2,
                        higher_bidder,
                        new_bid,
                    } => {
                        // higher_bidder: Guid
                        w.write_all(&higher_bidder.guid().to_le_bytes())?;

                        // new_bid: u32
                        w.write_all(&new_bid.to_le_bytes())?;

                        // auction_outbid2: u32
                        w.write_all(&auction_outbid2.to_le_bytes())?;

                    }
                    _ => {}
                }

            }
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(12..=28).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025B, size: body_size as u32 });
        }

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // action: AuctionCommandAction
        let action: AuctionCommandAction = crate::util::read_u32_le(&mut r)?.try_into()?;

        let action_if = match action {
            AuctionCommandAction::Started => {
                // result2: AuctionCommandResultTwo
                let result2: AuctionCommandResultTwo = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    AuctionCommandResultTwo::Ok => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::Ok,
                    AuctionCommandResultTwo::ErrInventory => {
                        // inventory_result2: InventoryResult
                        let inventory_result2: InventoryResult = crate::util::read_u8_le(&mut r)?.try_into()?;

                        SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrInventory {
                            inventory_result2,
                        }
                    }
                    AuctionCommandResultTwo::ErrDatabase => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrDatabase,
                    AuctionCommandResultTwo::ErrNotEnoughMoney => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrNotEnoughMoney,
                    AuctionCommandResultTwo::ErrItemNotFound => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrItemNotFound,
                    AuctionCommandResultTwo::ErrHigherBid => {
                        // higher_bidder2: Guid
                        let higher_bidder2 = Guid::read(&mut r)?;

                        // new_bid2: u32
                        let new_bid2 = crate::util::read_u32_le(&mut r)?;

                        // auction_outbid3: u32
                        let auction_outbid3 = crate::util::read_u32_le(&mut r)?;

                        SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrHigherBid {
                            auction_outbid3,
                            higher_bidder2,
                            new_bid2,
                        }
                    }
                    AuctionCommandResultTwo::ErrBidIncrement => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrBidIncrement,
                    AuctionCommandResultTwo::ErrBidOwn => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrBidOwn,
                    AuctionCommandResultTwo::ErrRestrictedAccount => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrRestrictedAccount,
                };

                SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction::Started {
                    result2: result2_if,
                }
            }
            AuctionCommandAction::Removed => {
                // result2: AuctionCommandResultTwo
                let result2: AuctionCommandResultTwo = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result2_if = match result2 {
                    AuctionCommandResultTwo::Ok => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::Ok,
                    AuctionCommandResultTwo::ErrInventory => {
                        // inventory_result2: InventoryResult
                        let inventory_result2: InventoryResult = crate::util::read_u8_le(&mut r)?.try_into()?;

                        SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrInventory {
                            inventory_result2,
                        }
                    }
                    AuctionCommandResultTwo::ErrDatabase => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrDatabase,
                    AuctionCommandResultTwo::ErrNotEnoughMoney => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrNotEnoughMoney,
                    AuctionCommandResultTwo::ErrItemNotFound => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrItemNotFound,
                    AuctionCommandResultTwo::ErrHigherBid => {
                        // higher_bidder2: Guid
                        let higher_bidder2 = Guid::read(&mut r)?;

                        // new_bid2: u32
                        let new_bid2 = crate::util::read_u32_le(&mut r)?;

                        // auction_outbid3: u32
                        let auction_outbid3 = crate::util::read_u32_le(&mut r)?;

                        SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrHigherBid {
                            auction_outbid3,
                            higher_bidder2,
                            new_bid2,
                        }
                    }
                    AuctionCommandResultTwo::ErrBidIncrement => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrBidIncrement,
                    AuctionCommandResultTwo::ErrBidOwn => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrBidOwn,
                    AuctionCommandResultTwo::ErrRestrictedAccount => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo::ErrRestrictedAccount,
                };

                SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction::Removed {
                    result2: result2_if,
                }
            }
            AuctionCommandAction::BidPlaced => {
                // result: AuctionCommandResult
                let result: AuctionCommandResult = crate::util::read_u32_le(&mut r)?.try_into()?;

                let result_if = match result {
                    AuctionCommandResult::Ok => {
                        // auction_outbid1: u32
                        let auction_outbid1 = crate::util::read_u32_le(&mut r)?;

                        SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::Ok {
                            auction_outbid1,
                        }
                    }
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
                    AuctionCommandResult::ErrHigherBid => {
                        // higher_bidder: Guid
                        let higher_bidder = Guid::read(&mut r)?;

                        // new_bid: u32
                        let new_bid = crate::util::read_u32_le(&mut r)?;

                        // auction_outbid2: u32
                        let auction_outbid2 = crate::util::read_u32_le(&mut r)?;

                        SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrHigherBid {
                            auction_outbid2,
                            higher_bidder,
                            new_bid,
                        }
                    }
                    AuctionCommandResult::ErrBidIncrement => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrBidIncrement,
                    AuctionCommandResult::ErrBidOwn => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrBidOwn,
                    AuctionCommandResult::ErrRestrictedAccount => SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult::ErrRestrictedAccount,
                };

                SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction::BidPlaced {
                    result: result_if,
                }
            }
        };

        Ok(Self {
            auction_id,
            action: action_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_COMMAND_RESULT {}

impl SMSG_AUCTION_COMMAND_RESULT {
    pub(crate) const fn size(&self) -> usize {
        4 // auction_id: u32
        + self.action.size() // action: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo {
    Ok,
    ErrInventory {
        inventory_result2: InventoryResult,
    },
    ErrDatabase,
    ErrNotEnoughMoney,
    ErrItemNotFound,
    ErrHigherBid {
        auction_outbid3: u32,
        higher_bidder2: Guid,
        new_bid2: u32,
    },
    ErrBidIncrement,
    ErrBidOwn,
    ErrRestrictedAccount,
}

impl Default for SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Ok
    }
}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0,
            Self::ErrInventory { .. } => 1,
            Self::ErrDatabase => 2,
            Self::ErrNotEnoughMoney => 3,
            Self::ErrItemNotFound => 4,
            Self::ErrHigherBid { .. } => 5,
            Self::ErrBidIncrement => 7,
            Self::ErrBidOwn => 10,
            Self::ErrRestrictedAccount => 13,
        }
    }

}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Ok => {
                4
            }
            Self::ErrInventory {
                inventory_result2,
            } => {
                4
                + 1 // inventory_result2: InventoryResult
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
            Self::ErrHigherBid {
                auction_outbid3,
                higher_bidder2,
                new_bid2,
            } => {
                4
                + 4 // auction_outbid3: u32
                + 8 // higher_bidder2: Guid
                + 4 // new_bid2: u32
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    Ok {
        auction_outbid1: u32,
    },
    ErrInventory {
        inventory_result: InventoryResult,
    },
    ErrDatabase,
    ErrNotEnoughMoney,
    ErrItemNotFound,
    ErrHigherBid {
        auction_outbid2: u32,
        higher_bidder: Guid,
        new_bid: u32,
    },
    ErrBidIncrement,
    ErrBidOwn,
    ErrRestrictedAccount,
}

impl Default for SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ErrDatabase
    }
}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Ok { .. } => 0,
            Self::ErrInventory { .. } => 1,
            Self::ErrDatabase => 2,
            Self::ErrNotEnoughMoney => 3,
            Self::ErrItemNotFound => 4,
            Self::ErrHigherBid { .. } => 5,
            Self::ErrBidIncrement => 7,
            Self::ErrBidOwn => 10,
            Self::ErrRestrictedAccount => 13,
        }
    }

}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Ok {
                auction_outbid1,
            } => {
                4
                + 4 // auction_outbid1: u32
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
            Self::ErrHigherBid {
                auction_outbid2,
                higher_bidder,
                new_bid,
            } => {
                4
                + 4 // auction_outbid2: u32
                + 8 // higher_bidder: Guid
                + 4 // new_bid: u32
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction {
    Started {
        result2: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo,
    },
    Removed {
        result2: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo,
    },
    BidPlaced {
        result: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult,
    },
}

impl Default for SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Started {
            result2: Default::default(),
        }
    }
}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Started { .. } => 0,
            Self::Removed { .. } => 1,
            Self::BidPlaced { .. } => 2,
        }
    }

}

impl SMSG_AUCTION_COMMAND_RESULT_AuctionCommandAction {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Started {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo
            }
            Self::Removed {
                result2,
            } => {
                4
                + result2.size() // result2: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResultTwo
            }
            Self::BidPlaced {
                result,
            } => {
                4
                + result.size() // result: SMSG_AUCTION_COMMAND_RESULT_AuctionCommandResult
            }
        }
    }
}

