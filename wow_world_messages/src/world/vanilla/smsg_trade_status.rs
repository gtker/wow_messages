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
impl SMSG_TRADE_STATUS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // status: TradeStatus
        let status = crate::util::read_u32_le(&mut r)?.try_into()?;

        let status_if = match status {
            TradeStatus::Busy => SMSG_TRADE_STATUS_TradeStatus::Busy,
            TradeStatus::BeginTrade => {
                // unknown1: Guid
                let unknown1 = crate::util::read_guid(&mut r)?;

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
                let inventory_result = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

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

impl crate::Message for SMSG_TRADE_STATUS {
    const OPCODE: u32 = 0x0120;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRADE_STATUS {{").unwrap();
        // Members
        writeln!(s, "    status = {};", TradeStatus::try_from(self.status.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.status {
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::BeginTrade {
                unknown1,
            } => {
                writeln!(s, "    unknown1 = {};", unknown1.guid()).unwrap();
            }
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::CloseWindow {
                inventory_result,
                item_limit_category_id,
                target_error,
            } => {
                writeln!(s, "    inventory_result = {};", inventory_result.as_test_case_value()).unwrap();
                writeln!(s, "    target_error = {};", if *target_error { "TRUE" } else { "FALSE" }).unwrap();
                writeln!(s, "    item_limit_category_id = {};", item_limit_category_id).unwrap();
            }
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::OnlyConjured {
                slot,
            } => {
                writeln!(s, "    slot = {};", slot).unwrap();
            }
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::NotOnTaplist {
                slot,
            } => {
                writeln!(s, "    slot = {};", slot).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 288_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "status", "    ");
        match &self.status {
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::BeginTrade {
                unknown1,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "unknown1", "    ");
            }
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::CloseWindow {
                inventory_result,
                item_limit_category_id,
                target_error,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "inventory_result", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "target_error", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_limit_category_id", "    ");
            }
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::OnlyConjured {
                slot,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
            }
            crate::vanilla::SMSG_TRADE_STATUS_TradeStatus::NotOnTaplist {
                slot,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(288, "SMSG_TRADE_STATUS", body_size, a))
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

impl std::fmt::Display for SMSG_TRADE_STATUS_TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Busy => f.write_str("Busy"),
            Self::BeginTrade{ .. } => f.write_str("BeginTrade"),
            Self::OpenWindow => f.write_str("OpenWindow"),
            Self::TradeCanceled => f.write_str("TradeCanceled"),
            Self::TradeAccept => f.write_str("TradeAccept"),
            Self::Busy2 => f.write_str("Busy2"),
            Self::NoTarget => f.write_str("NoTarget"),
            Self::BackToTrade => f.write_str("BackToTrade"),
            Self::TradeComplete => f.write_str("TradeComplete"),
            Self::TradeRejected => f.write_str("TradeRejected"),
            Self::TargetToFar => f.write_str("TargetToFar"),
            Self::WrongFaction => f.write_str("WrongFaction"),
            Self::CloseWindow{ .. } => f.write_str("CloseWindow"),
            Self::Unknown13 => f.write_str("Unknown13"),
            Self::IgnoreYou => f.write_str("IgnoreYou"),
            Self::YouStunned => f.write_str("YouStunned"),
            Self::TargetStunned => f.write_str("TargetStunned"),
            Self::YouDead => f.write_str("YouDead"),
            Self::TargetDead => f.write_str("TargetDead"),
            Self::YouLogout => f.write_str("YouLogout"),
            Self::TargetLogout => f.write_str("TargetLogout"),
            Self::TrialAccount => f.write_str("TrialAccount"),
            Self::OnlyConjured{ .. } => f.write_str("OnlyConjured"),
            Self::NotOnTaplist{ .. } => f.write_str("NotOnTaplist"),
        }
    }
}

impl SMSG_TRADE_STATUS_TradeStatus {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::BeginTrade {
                ..
            } => {
                4
                + 8 // unknown1: Guid
            }
            Self::CloseWindow {
                ..
            } => {
                4
                + 4 // inventory_result: InventoryResult
                + 4 // item_limit_category_id: u32
                + 1 // target_error: Bool
            }
            Self::OnlyConjured {
                ..
            } => {
                4
                + 1 // slot: u8
            }
            Self::NotOnTaplist {
                ..
            } => {
                4
                + 1 // slot: u8
            }
            _ => 4,
        }
    }
}

