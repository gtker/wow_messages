use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    InventoryResult, TradeStatus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status.wowm:283`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status.wowm#L283):
/// ```text
/// smsg SMSG_TRADE_STATUS = 0x0120 {
///     TradeStatus status;
///     if (status == BEGIN_TRADE) {
///         Guid unknown1;
///     }
///     else if (status == CLOSE_WINDOW) {
///         (u32)InventoryResult inventory_result;
///         Bool target_error;
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

impl crate::private::Sealed for SMSG_TRADE_STATUS {}
impl crate::Message for SMSG_TRADE_STATUS {
    const OPCODE: u32 = 0x0120;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // status: TradeStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        match &self.status {
            SMSG_TRADE_STATUS_TradeStatus::BeginTrade {
                unknown1,
            } => {
                // unknown1: Guid
                w.write_all(&unknown1.guid().to_le_bytes())?;

            }
            SMSG_TRADE_STATUS_TradeStatus::CloseWindow {
                inventory_result,
                item_limit_category_id,
                target_error,
            } => {
                // inventory_result: InventoryResult
                w.write_all(&u32::from(inventory_result.as_int()).to_le_bytes())?;

                // target_error: Bool
                w.write_all(u8::from(*target_error).to_le_bytes().as_slice())?;

                // item_limit_category_id: u32
                w.write_all(&item_limit_category_id.to_le_bytes())?;

            }
            SMSG_TRADE_STATUS_TradeStatus::OnlyConjured {
                slot,
            } => {
                // slot: u8
                w.write_all(&slot.to_le_bytes())?;

            }
            SMSG_TRADE_STATUS_TradeStatus::NotOnTaplist {
                slot,
            } => {
                // slot: u8
                w.write_all(&slot.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0120, size: body_size });
        }

        // status: TradeStatus
        let status: TradeStatus = crate::util::read_u32_le(&mut r)?.try_into()?;

        let status_if = match status {
            TradeStatus::Busy => SMSG_TRADE_STATUS_TradeStatus::Busy,
            TradeStatus::BeginTrade => {
                // unknown1: Guid
                let unknown1 = Guid::read(&mut r)?;

                SMSG_TRADE_STATUS_TradeStatus::BeginTrade {
                    unknown1,
                }
            }
            TradeStatus::OpenWindow => SMSG_TRADE_STATUS_TradeStatus::OpenWindow,
            TradeStatus::TradeCanceled => SMSG_TRADE_STATUS_TradeStatus::TradeCanceled,
            TradeStatus::TradeAccept => SMSG_TRADE_STATUS_TradeStatus::TradeAccept,
            TradeStatus::Busy2 => SMSG_TRADE_STATUS_TradeStatus::Busy2,
            TradeStatus::NoTarget => SMSG_TRADE_STATUS_TradeStatus::NoTarget,
            TradeStatus::BackToTrade => SMSG_TRADE_STATUS_TradeStatus::BackToTrade,
            TradeStatus::TradeComplete => SMSG_TRADE_STATUS_TradeStatus::TradeComplete,
            TradeStatus::TradeRejected => SMSG_TRADE_STATUS_TradeStatus::TradeRejected,
            TradeStatus::TargetToFar => SMSG_TRADE_STATUS_TradeStatus::TargetToFar,
            TradeStatus::WrongFaction => SMSG_TRADE_STATUS_TradeStatus::WrongFaction,
            TradeStatus::CloseWindow => {
                // inventory_result: InventoryResult
                let inventory_result: InventoryResult = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                // target_error: Bool
                let target_error = crate::util::read_u8_le(&mut r)? != 0;

                // item_limit_category_id: u32
                let item_limit_category_id = crate::util::read_u32_le(&mut r)?;

                SMSG_TRADE_STATUS_TradeStatus::CloseWindow {
                    inventory_result,
                    item_limit_category_id,
                    target_error,
                }
            }
            TradeStatus::Unknown13 => SMSG_TRADE_STATUS_TradeStatus::Unknown13,
            TradeStatus::IgnoreYou => SMSG_TRADE_STATUS_TradeStatus::IgnoreYou,
            TradeStatus::YouStunned => SMSG_TRADE_STATUS_TradeStatus::YouStunned,
            TradeStatus::TargetStunned => SMSG_TRADE_STATUS_TradeStatus::TargetStunned,
            TradeStatus::YouDead => SMSG_TRADE_STATUS_TradeStatus::YouDead,
            TradeStatus::TargetDead => SMSG_TRADE_STATUS_TradeStatus::TargetDead,
            TradeStatus::YouLogout => SMSG_TRADE_STATUS_TradeStatus::YouLogout,
            TradeStatus::TargetLogout => SMSG_TRADE_STATUS_TradeStatus::TargetLogout,
            TradeStatus::TrialAccount => SMSG_TRADE_STATUS_TradeStatus::TrialAccount,
            TradeStatus::OnlyConjured => {
                // slot: u8
                let slot = crate::util::read_u8_le(&mut r)?;

                SMSG_TRADE_STATUS_TradeStatus::OnlyConjured {
                    slot,
                }
            }
            TradeStatus::NotOnTaplist => {
                // slot: u8
                let slot = crate::util::read_u8_le(&mut r)?;

                SMSG_TRADE_STATUS_TradeStatus::NotOnTaplist {
                    slot,
                }
            }
        };

        Ok(Self {
            status: status_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TRADE_STATUS {}

impl SMSG_TRADE_STATUS {
    pub(crate) const fn size(&self) -> usize {
        self.status.size() // status: SMSG_TRADE_STATUS_TradeStatus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_TRADE_STATUS_TradeStatus {
    Busy,
    BeginTrade {
        unknown1: Guid,
    },
    OpenWindow,
    TradeCanceled,
    TradeAccept,
    Busy2,
    NoTarget,
    BackToTrade,
    TradeComplete,
    TradeRejected,
    TargetToFar,
    WrongFaction,
    CloseWindow {
        inventory_result: InventoryResult,
        item_limit_category_id: u32,
        target_error: bool,
    },
    Unknown13,
    IgnoreYou,
    YouStunned,
    TargetStunned,
    YouDead,
    TargetDead,
    YouLogout,
    TargetLogout,
    TrialAccount,
    OnlyConjured {
        slot: u8,
    },
    NotOnTaplist {
        slot: u8,
    },
}

impl Default for SMSG_TRADE_STATUS_TradeStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Busy
    }
}

impl SMSG_TRADE_STATUS_TradeStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Busy => 0,
            Self::BeginTrade { .. } => 1,
            Self::OpenWindow => 2,
            Self::TradeCanceled => 3,
            Self::TradeAccept => 4,
            Self::Busy2 => 5,
            Self::NoTarget => 6,
            Self::BackToTrade => 7,
            Self::TradeComplete => 8,
            Self::TradeRejected => 9,
            Self::TargetToFar => 10,
            Self::WrongFaction => 11,
            Self::CloseWindow { .. } => 12,
            Self::Unknown13 => 13,
            Self::IgnoreYou => 14,
            Self::YouStunned => 15,
            Self::TargetStunned => 16,
            Self::YouDead => 17,
            Self::TargetDead => 18,
            Self::YouLogout => 19,
            Self::TargetLogout => 20,
            Self::TrialAccount => 21,
            Self::OnlyConjured { .. } => 22,
            Self::NotOnTaplist { .. } => 23,
        }
    }

}

impl SMSG_TRADE_STATUS_TradeStatus {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Busy => {
                4
            }
            Self::BeginTrade {
                unknown1,
            } => {
                4
                + 8 // unknown1: Guid
            }
            Self::OpenWindow => {
                4
            }
            Self::TradeCanceled => {
                4
            }
            Self::TradeAccept => {
                4
            }
            Self::Busy2 => {
                4
            }
            Self::NoTarget => {
                4
            }
            Self::BackToTrade => {
                4
            }
            Self::TradeComplete => {
                4
            }
            Self::TradeRejected => {
                4
            }
            Self::TargetToFar => {
                4
            }
            Self::WrongFaction => {
                4
            }
            Self::CloseWindow {
                inventory_result,
                item_limit_category_id,
                target_error,
            } => {
                4
                + 4 // inventory_result: InventoryResult
                + 4 // item_limit_category_id: u32
                + 1 // target_error: Bool
            }
            Self::Unknown13 => {
                4
            }
            Self::IgnoreYou => {
                4
            }
            Self::YouStunned => {
                4
            }
            Self::TargetStunned => {
                4
            }
            Self::YouDead => {
                4
            }
            Self::TargetDead => {
                4
            }
            Self::YouLogout => {
                4
            }
            Self::TargetLogout => {
                4
            }
            Self::TrialAccount => {
                4
            }
            Self::OnlyConjured {
                slot,
            } => {
                4
                + 1 // slot: u8
            }
            Self::NotOnTaplist {
                slot,
            } => {
                4
                + 1 // slot: u8
            }
        }
    }
}

