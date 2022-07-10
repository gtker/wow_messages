use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::InventoryResult;
use crate::world::version_1_12::TradeStatus;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:102`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L102):
/// ```text
/// smsg SMSG_TRADE_STATUS = 0x0120 {
///     TradeStatus status;
///     if (status == BEGIN_TRADE) {
///         Guid unknown1;
///     }
///     else if (status == CLOSE_WINDOW) {
///         InventoryResult inventory_result;
///         u8 target_error;
///         u32 item_limit_category_id;
///     }
///     else if (status == ONLY_CONJURED
///         || status == NOT_ON_TAPLIST) {
///         u8 slot;
///     }
/// }
/// ```
pub struct SMSG_TRADE_STATUS {
    pub status: SMSG_TRADE_STATUS_TradeStatus,
}

impl ServerMessage for SMSG_TRADE_STATUS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // status: TradeStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        match &self.status {
            SMSG_TRADE_STATUS_TradeStatus::BUSY => {}
            SMSG_TRADE_STATUS_TradeStatus::BEGIN_TRADE {
                unknown1,
            } => {
                // unknown1: Guid
                w.write_all(&unknown1.guid().to_le_bytes())?;

            }
            SMSG_TRADE_STATUS_TradeStatus::OPEN_WINDOW => {}
            SMSG_TRADE_STATUS_TradeStatus::TRADE_CANCELED => {}
            SMSG_TRADE_STATUS_TradeStatus::TRADE_ACCEPT => {}
            SMSG_TRADE_STATUS_TradeStatus::BUSY_2 => {}
            SMSG_TRADE_STATUS_TradeStatus::NO_TARGET => {}
            SMSG_TRADE_STATUS_TradeStatus::BACK_TO_TRADE => {}
            SMSG_TRADE_STATUS_TradeStatus::TRADE_COMPLETE => {}
            SMSG_TRADE_STATUS_TradeStatus::TRADE_REJECTED => {}
            SMSG_TRADE_STATUS_TradeStatus::TARGET_TO_FAR => {}
            SMSG_TRADE_STATUS_TradeStatus::WRONG_FACTION => {}
            SMSG_TRADE_STATUS_TradeStatus::CLOSE_WINDOW {
                inventory_result,
                item_limit_category_id,
                target_error,
            } => {
                // inventory_result: InventoryResult
                w.write_all(&(inventory_result.as_int() as u32).to_le_bytes())?;

                // target_error: u8
                w.write_all(&target_error.to_le_bytes())?;

                // item_limit_category_id: u32
                w.write_all(&item_limit_category_id.to_le_bytes())?;

            }
            SMSG_TRADE_STATUS_TradeStatus::UNKNOWN_13 => {}
            SMSG_TRADE_STATUS_TradeStatus::IGNORE_YOU => {}
            SMSG_TRADE_STATUS_TradeStatus::YOU_STUNNED => {}
            SMSG_TRADE_STATUS_TradeStatus::TARGET_STUNNED => {}
            SMSG_TRADE_STATUS_TradeStatus::YOU_DEAD => {}
            SMSG_TRADE_STATUS_TradeStatus::TARGET_DEAD => {}
            SMSG_TRADE_STATUS_TradeStatus::YOU_LOGOUT => {}
            SMSG_TRADE_STATUS_TradeStatus::TARGET_LOGOUT => {}
            SMSG_TRADE_STATUS_TradeStatus::TRIAL_ACCOUNT => {}
            SMSG_TRADE_STATUS_TradeStatus::ONLY_CONJURED {
                slot,
            } => {
                // slot: u8
                w.write_all(&slot.to_le_bytes())?;

            }
            SMSG_TRADE_STATUS_TradeStatus::NOT_ON_TAPLIST {
                slot,
            } => {
                // slot: u8
                w.write_all(&slot.to_le_bytes())?;

            }
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0120;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // status: TradeStatus
        let status: TradeStatus = crate::util::read_u32_le(r)?.try_into()?;

        let status_if = match status {
            TradeStatus::BUSY => SMSG_TRADE_STATUS_TradeStatus::BUSY,
            TradeStatus::BEGIN_TRADE => {
                // unknown1: Guid
                let unknown1 = Guid::read(r)?;

                SMSG_TRADE_STATUS_TradeStatus::BEGIN_TRADE {
                    unknown1,
                }
            }
            TradeStatus::OPEN_WINDOW => SMSG_TRADE_STATUS_TradeStatus::OPEN_WINDOW,
            TradeStatus::TRADE_CANCELED => SMSG_TRADE_STATUS_TradeStatus::TRADE_CANCELED,
            TradeStatus::TRADE_ACCEPT => SMSG_TRADE_STATUS_TradeStatus::TRADE_ACCEPT,
            TradeStatus::BUSY_2 => SMSG_TRADE_STATUS_TradeStatus::BUSY_2,
            TradeStatus::NO_TARGET => SMSG_TRADE_STATUS_TradeStatus::NO_TARGET,
            TradeStatus::BACK_TO_TRADE => SMSG_TRADE_STATUS_TradeStatus::BACK_TO_TRADE,
            TradeStatus::TRADE_COMPLETE => SMSG_TRADE_STATUS_TradeStatus::TRADE_COMPLETE,
            TradeStatus::TRADE_REJECTED => SMSG_TRADE_STATUS_TradeStatus::TRADE_REJECTED,
            TradeStatus::TARGET_TO_FAR => SMSG_TRADE_STATUS_TradeStatus::TARGET_TO_FAR,
            TradeStatus::WRONG_FACTION => SMSG_TRADE_STATUS_TradeStatus::WRONG_FACTION,
            TradeStatus::CLOSE_WINDOW => {
                // inventory_result: InventoryResult
                let inventory_result: InventoryResult = (crate::util::read_u32_le(r)? as u8).try_into()?;

                // target_error: u8
                let target_error = crate::util::read_u8_le(r)?;

                // item_limit_category_id: u32
                let item_limit_category_id = crate::util::read_u32_le(r)?;

                SMSG_TRADE_STATUS_TradeStatus::CLOSE_WINDOW {
                    inventory_result,
                    item_limit_category_id,
                    target_error,
                }
            }
            TradeStatus::UNKNOWN_13 => SMSG_TRADE_STATUS_TradeStatus::UNKNOWN_13,
            TradeStatus::IGNORE_YOU => SMSG_TRADE_STATUS_TradeStatus::IGNORE_YOU,
            TradeStatus::YOU_STUNNED => SMSG_TRADE_STATUS_TradeStatus::YOU_STUNNED,
            TradeStatus::TARGET_STUNNED => SMSG_TRADE_STATUS_TradeStatus::TARGET_STUNNED,
            TradeStatus::YOU_DEAD => SMSG_TRADE_STATUS_TradeStatus::YOU_DEAD,
            TradeStatus::TARGET_DEAD => SMSG_TRADE_STATUS_TradeStatus::TARGET_DEAD,
            TradeStatus::YOU_LOGOUT => SMSG_TRADE_STATUS_TradeStatus::YOU_LOGOUT,
            TradeStatus::TARGET_LOGOUT => SMSG_TRADE_STATUS_TradeStatus::TARGET_LOGOUT,
            TradeStatus::TRIAL_ACCOUNT => SMSG_TRADE_STATUS_TradeStatus::TRIAL_ACCOUNT,
            TradeStatus::ONLY_CONJURED => {
                // slot: u8
                let slot = crate::util::read_u8_le(r)?;

                SMSG_TRADE_STATUS_TradeStatus::ONLY_CONJURED {
                    slot,
                }
            }
            TradeStatus::NOT_ON_TAPLIST => {
                // slot: u8
                let slot = crate::util::read_u8_le(r)?;

                SMSG_TRADE_STATUS_TradeStatus::NOT_ON_TAPLIST {
                    slot,
                }
            }
        };

        Ok(Self {
            status: status_if,
        })
    }

}

impl SMSG_TRADE_STATUS {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: SMSG_TRADE_STATUS_TradeStatus
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_TRADE_STATUS_TradeStatus {
    BUSY,
    BEGIN_TRADE {
        unknown1: Guid,
    },
    OPEN_WINDOW,
    TRADE_CANCELED,
    TRADE_ACCEPT,
    BUSY_2,
    NO_TARGET,
    BACK_TO_TRADE,
    TRADE_COMPLETE,
    TRADE_REJECTED,
    TARGET_TO_FAR,
    WRONG_FACTION,
    CLOSE_WINDOW {
        inventory_result: InventoryResult,
        item_limit_category_id: u32,
        target_error: u8,
    },
    UNKNOWN_13,
    IGNORE_YOU,
    YOU_STUNNED,
    TARGET_STUNNED,
    YOU_DEAD,
    TARGET_DEAD,
    YOU_LOGOUT,
    TARGET_LOGOUT,
    TRIAL_ACCOUNT,
    ONLY_CONJURED {
        slot: u8,
    },
    NOT_ON_TAPLIST {
        slot: u8,
    },
}

impl Default for SMSG_TRADE_STATUS_TradeStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::BUSY
    }
}

impl SMSG_TRADE_STATUS_TradeStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::BUSY => 0,
            Self::BEGIN_TRADE { .. } => 1,
            Self::OPEN_WINDOW => 2,
            Self::TRADE_CANCELED => 3,
            Self::TRADE_ACCEPT => 4,
            Self::BUSY_2 => 5,
            Self::NO_TARGET => 6,
            Self::BACK_TO_TRADE => 7,
            Self::TRADE_COMPLETE => 8,
            Self::TRADE_REJECTED => 9,
            Self::TARGET_TO_FAR => 10,
            Self::WRONG_FACTION => 11,
            Self::CLOSE_WINDOW { .. } => 12,
            Self::UNKNOWN_13 => 13,
            Self::IGNORE_YOU => 14,
            Self::YOU_STUNNED => 15,
            Self::TARGET_STUNNED => 16,
            Self::YOU_DEAD => 17,
            Self::TARGET_DEAD => 18,
            Self::YOU_LOGOUT => 19,
            Self::TARGET_LOGOUT => 20,
            Self::TRIAL_ACCOUNT => 21,
            Self::ONLY_CONJURED { .. } => 22,
            Self::NOT_ON_TAPLIST { .. } => 23,
        }
    }

}

impl SMSG_TRADE_STATUS_TradeStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::BUSY => {
                4
            }
            Self::BEGIN_TRADE {
                unknown1,
            } => {
                4
                + 8 // unknown1: Guid
            }
            Self::OPEN_WINDOW => {
                4
            }
            Self::TRADE_CANCELED => {
                4
            }
            Self::TRADE_ACCEPT => {
                4
            }
            Self::BUSY_2 => {
                4
            }
            Self::NO_TARGET => {
                4
            }
            Self::BACK_TO_TRADE => {
                4
            }
            Self::TRADE_COMPLETE => {
                4
            }
            Self::TRADE_REJECTED => {
                4
            }
            Self::TARGET_TO_FAR => {
                4
            }
            Self::WRONG_FACTION => {
                4
            }
            Self::CLOSE_WINDOW {
                inventory_result,
                item_limit_category_id,
                target_error,
            } => {
                4
                + 4 // inventory_result: InventoryResult
                + 4 // item_limit_category_id: u32
                + 1 // target_error: u8
            }
            Self::UNKNOWN_13 => {
                4
            }
            Self::IGNORE_YOU => {
                4
            }
            Self::YOU_STUNNED => {
                4
            }
            Self::TARGET_STUNNED => {
                4
            }
            Self::YOU_DEAD => {
                4
            }
            Self::TARGET_DEAD => {
                4
            }
            Self::YOU_LOGOUT => {
                4
            }
            Self::TARGET_LOGOUT => {
                4
            }
            Self::TRIAL_ACCOUNT => {
                4
            }
            Self::ONLY_CONJURED {
                slot,
            } => {
                4
                + 1 // slot: u8
            }
            Self::NOT_ON_TAPLIST {
                slot,
            } => {
                4
                + 1 // slot: u8
            }
        }
    }
}

