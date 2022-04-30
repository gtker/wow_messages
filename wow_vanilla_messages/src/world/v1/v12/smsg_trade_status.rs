use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{InventoryResult, InventoryResultError};
use crate::world::v1::v12::{TradeStatus, TradeStatusError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_TRADE_STATUS {
    pub status: SMSG_TRADE_STATUSTradeStatus,
}

impl ServerMessageWrite for SMSG_TRADE_STATUS {}

impl MessageBody for SMSG_TRADE_STATUS {
    const OPCODE: u16 = 0x0120;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_TRADE_STATUSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // status: TradeStatus
        let status = TradeStatus::read(r)?;

        let status_if = match status {
            TradeStatus::BUSY => SMSG_TRADE_STATUSTradeStatus::BUSY,
            TradeStatus::BEGIN_TRADE => {
                // unknown1: Guid
                let unknown1 = Guid::read(r)?;

                SMSG_TRADE_STATUSTradeStatus::BEGIN_TRADE {
                    unknown1,
                }
            }
            TradeStatus::OPEN_WINDOW => SMSG_TRADE_STATUSTradeStatus::OPEN_WINDOW,
            TradeStatus::TRADE_CANCELED => SMSG_TRADE_STATUSTradeStatus::TRADE_CANCELED,
            TradeStatus::TRADE_ACCEPT => SMSG_TRADE_STATUSTradeStatus::TRADE_ACCEPT,
            TradeStatus::BUSY_2 => SMSG_TRADE_STATUSTradeStatus::BUSY_2,
            TradeStatus::NO_TARGET => SMSG_TRADE_STATUSTradeStatus::NO_TARGET,
            TradeStatus::BACK_TO_TRADE => SMSG_TRADE_STATUSTradeStatus::BACK_TO_TRADE,
            TradeStatus::TRADE_COMPLETE => SMSG_TRADE_STATUSTradeStatus::TRADE_COMPLETE,
            TradeStatus::TRADE_REJECTED => SMSG_TRADE_STATUSTradeStatus::TRADE_REJECTED,
            TradeStatus::TARGET_TO_FAR => SMSG_TRADE_STATUSTradeStatus::TARGET_TO_FAR,
            TradeStatus::WRONG_FACTION => SMSG_TRADE_STATUSTradeStatus::WRONG_FACTION,
            TradeStatus::CLOSE_WINDOW => {
                // inventory_result: InventoryResult
                let inventory_result = InventoryResult::read_u32_le(r)?;

                // target_error: u8
                let target_error = crate::util::read_u8_le(r)?;

                // item_limit_category_id: u32
                let item_limit_category_id = crate::util::read_u32_le(r)?;

                SMSG_TRADE_STATUSTradeStatus::CLOSE_WINDOW {
                    inventory_result,
                    target_error,
                    item_limit_category_id,
                }
            }
            TradeStatus::UNKNOWN_13 => SMSG_TRADE_STATUSTradeStatus::UNKNOWN_13,
            TradeStatus::IGNORE_YOU => SMSG_TRADE_STATUSTradeStatus::IGNORE_YOU,
            TradeStatus::YOU_STUNNED => SMSG_TRADE_STATUSTradeStatus::YOU_STUNNED,
            TradeStatus::TARGET_STUNNED => SMSG_TRADE_STATUSTradeStatus::TARGET_STUNNED,
            TradeStatus::YOU_DEAD => SMSG_TRADE_STATUSTradeStatus::YOU_DEAD,
            TradeStatus::TARGET_DEAD => SMSG_TRADE_STATUSTradeStatus::TARGET_DEAD,
            TradeStatus::YOU_LOGOUT => SMSG_TRADE_STATUSTradeStatus::YOU_LOGOUT,
            TradeStatus::TARGET_LOGOUT => SMSG_TRADE_STATUSTradeStatus::TARGET_LOGOUT,
            TradeStatus::TRIAL_ACCOUNT => SMSG_TRADE_STATUSTradeStatus::TRIAL_ACCOUNT,
            TradeStatus::ONLY_CONJURED => {
                // slot: u8
                let slot = crate::util::read_u8_le(r)?;

                SMSG_TRADE_STATUSTradeStatus::ONLY_CONJURED {
                    slot,
                }
            }
            TradeStatus::NOT_ON_TAPLIST => {
                // slot: u8
                let slot = crate::util::read_u8_le(r)?;

                SMSG_TRADE_STATUSTradeStatus::NOT_ON_TAPLIST {
                    slot,
                }
            }
        };

        Ok(Self {
            status: status_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // status: TradeStatus
        self.status.write(w)?;

        match &self.status {
            SMSG_TRADE_STATUSTradeStatus::BUSY => {}
            SMSG_TRADE_STATUSTradeStatus::BEGIN_TRADE {
                unknown1,
            } => {
                // unknown1: Guid
                unknown1.write(w)?;

            }
            SMSG_TRADE_STATUSTradeStatus::OPEN_WINDOW => {}
            SMSG_TRADE_STATUSTradeStatus::TRADE_CANCELED => {}
            SMSG_TRADE_STATUSTradeStatus::TRADE_ACCEPT => {}
            SMSG_TRADE_STATUSTradeStatus::BUSY_2 => {}
            SMSG_TRADE_STATUSTradeStatus::NO_TARGET => {}
            SMSG_TRADE_STATUSTradeStatus::BACK_TO_TRADE => {}
            SMSG_TRADE_STATUSTradeStatus::TRADE_COMPLETE => {}
            SMSG_TRADE_STATUSTradeStatus::TRADE_REJECTED => {}
            SMSG_TRADE_STATUSTradeStatus::TARGET_TO_FAR => {}
            SMSG_TRADE_STATUSTradeStatus::WRONG_FACTION => {}
            SMSG_TRADE_STATUSTradeStatus::CLOSE_WINDOW {
                inventory_result,
                target_error,
                item_limit_category_id,
            } => {
                // inventory_result: InventoryResult
                inventory_result.write_u32_le(w)?;

                // target_error: u8
                w.write_all(&target_error.to_le_bytes())?;

                // item_limit_category_id: u32
                w.write_all(&item_limit_category_id.to_le_bytes())?;

            }
            SMSG_TRADE_STATUSTradeStatus::UNKNOWN_13 => {}
            SMSG_TRADE_STATUSTradeStatus::IGNORE_YOU => {}
            SMSG_TRADE_STATUSTradeStatus::YOU_STUNNED => {}
            SMSG_TRADE_STATUSTradeStatus::TARGET_STUNNED => {}
            SMSG_TRADE_STATUSTradeStatus::YOU_DEAD => {}
            SMSG_TRADE_STATUSTradeStatus::TARGET_DEAD => {}
            SMSG_TRADE_STATUSTradeStatus::YOU_LOGOUT => {}
            SMSG_TRADE_STATUSTradeStatus::TARGET_LOGOUT => {}
            SMSG_TRADE_STATUSTradeStatus::TRIAL_ACCOUNT => {}
            SMSG_TRADE_STATUSTradeStatus::ONLY_CONJURED {
                slot,
            } => {
                // slot: u8
                w.write_all(&slot.to_le_bytes())?;

            }
            SMSG_TRADE_STATUSTradeStatus::NOT_ON_TAPLIST {
                slot,
            } => {
                // slot: u8
                w.write_all(&slot.to_le_bytes())?;

            }
        }

        Ok(())
    }
}

impl VariableSized for SMSG_TRADE_STATUS {
    fn size(&self) -> usize {
        self.status.size() // status: TradeStatus and subfields
    }
}

impl MaximumPossibleSized for SMSG_TRADE_STATUS {
    fn maximum_possible_size() -> usize {
        TradeStatus::maximum_possible_size() // status: TradeStatus
    }
}

#[derive(Debug)]
pub enum SMSG_TRADE_STATUSError {
    Io(std::io::Error),
    InventoryResult(InventoryResultError),
    TradeStatus(TradeStatusError),
}

impl std::error::Error for SMSG_TRADE_STATUSError {}
impl std::fmt::Display for SMSG_TRADE_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InventoryResult(i) => i.fmt(f),
            Self::TradeStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRADE_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<InventoryResultError> for SMSG_TRADE_STATUSError {
    fn from(e: InventoryResultError) -> Self {
        Self::InventoryResult(e)
    }
}

impl From<TradeStatusError> for SMSG_TRADE_STATUSError {
    fn from(e: TradeStatusError) -> Self {
        Self::TradeStatus(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_TRADE_STATUSTradeStatus {
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
        target_error: u8,
        item_limit_category_id: u32,
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

impl From<&TradeStatus> for SMSG_TRADE_STATUSTradeStatus {
    fn from(e: &TradeStatus) -> Self {
        match &e {
            TradeStatus::BUSY => Self::BUSY,
            TradeStatus::BEGIN_TRADE => Self::BEGIN_TRADE {
                unknown1: Default::default(),
            },
            TradeStatus::OPEN_WINDOW => Self::OPEN_WINDOW,
            TradeStatus::TRADE_CANCELED => Self::TRADE_CANCELED,
            TradeStatus::TRADE_ACCEPT => Self::TRADE_ACCEPT,
            TradeStatus::BUSY_2 => Self::BUSY_2,
            TradeStatus::NO_TARGET => Self::NO_TARGET,
            TradeStatus::BACK_TO_TRADE => Self::BACK_TO_TRADE,
            TradeStatus::TRADE_COMPLETE => Self::TRADE_COMPLETE,
            TradeStatus::TRADE_REJECTED => Self::TRADE_REJECTED,
            TradeStatus::TARGET_TO_FAR => Self::TARGET_TO_FAR,
            TradeStatus::WRONG_FACTION => Self::WRONG_FACTION,
            TradeStatus::CLOSE_WINDOW => Self::CLOSE_WINDOW {
                inventory_result: Default::default(),
                target_error: Default::default(),
                item_limit_category_id: Default::default(),
            },
            TradeStatus::UNKNOWN_13 => Self::UNKNOWN_13,
            TradeStatus::IGNORE_YOU => Self::IGNORE_YOU,
            TradeStatus::YOU_STUNNED => Self::YOU_STUNNED,
            TradeStatus::TARGET_STUNNED => Self::TARGET_STUNNED,
            TradeStatus::YOU_DEAD => Self::YOU_DEAD,
            TradeStatus::TARGET_DEAD => Self::TARGET_DEAD,
            TradeStatus::YOU_LOGOUT => Self::YOU_LOGOUT,
            TradeStatus::TARGET_LOGOUT => Self::TARGET_LOGOUT,
            TradeStatus::TRIAL_ACCOUNT => Self::TRIAL_ACCOUNT,
            TradeStatus::ONLY_CONJURED => Self::ONLY_CONJURED {
                slot: Default::default(),
            },
            TradeStatus::NOT_ON_TAPLIST => Self::NOT_ON_TAPLIST {
                slot: Default::default(),
            },
        }
    }
}

impl From<&SMSG_TRADE_STATUSTradeStatus> for TradeStatus {
    fn from(v: &SMSG_TRADE_STATUSTradeStatus) -> Self {
        match &v {
            SMSG_TRADE_STATUSTradeStatus::BUSY => Self::BUSY,
            SMSG_TRADE_STATUSTradeStatus::BEGIN_TRADE { .. } => Self::BEGIN_TRADE,
            SMSG_TRADE_STATUSTradeStatus::OPEN_WINDOW => Self::OPEN_WINDOW,
            SMSG_TRADE_STATUSTradeStatus::TRADE_CANCELED => Self::TRADE_CANCELED,
            SMSG_TRADE_STATUSTradeStatus::TRADE_ACCEPT => Self::TRADE_ACCEPT,
            SMSG_TRADE_STATUSTradeStatus::BUSY_2 => Self::BUSY_2,
            SMSG_TRADE_STATUSTradeStatus::NO_TARGET => Self::NO_TARGET,
            SMSG_TRADE_STATUSTradeStatus::BACK_TO_TRADE => Self::BACK_TO_TRADE,
            SMSG_TRADE_STATUSTradeStatus::TRADE_COMPLETE => Self::TRADE_COMPLETE,
            SMSG_TRADE_STATUSTradeStatus::TRADE_REJECTED => Self::TRADE_REJECTED,
            SMSG_TRADE_STATUSTradeStatus::TARGET_TO_FAR => Self::TARGET_TO_FAR,
            SMSG_TRADE_STATUSTradeStatus::WRONG_FACTION => Self::WRONG_FACTION,
            SMSG_TRADE_STATUSTradeStatus::CLOSE_WINDOW { .. } => Self::CLOSE_WINDOW,
            SMSG_TRADE_STATUSTradeStatus::UNKNOWN_13 => Self::UNKNOWN_13,
            SMSG_TRADE_STATUSTradeStatus::IGNORE_YOU => Self::IGNORE_YOU,
            SMSG_TRADE_STATUSTradeStatus::YOU_STUNNED => Self::YOU_STUNNED,
            SMSG_TRADE_STATUSTradeStatus::TARGET_STUNNED => Self::TARGET_STUNNED,
            SMSG_TRADE_STATUSTradeStatus::YOU_DEAD => Self::YOU_DEAD,
            SMSG_TRADE_STATUSTradeStatus::TARGET_DEAD => Self::TARGET_DEAD,
            SMSG_TRADE_STATUSTradeStatus::YOU_LOGOUT => Self::YOU_LOGOUT,
            SMSG_TRADE_STATUSTradeStatus::TARGET_LOGOUT => Self::TARGET_LOGOUT,
            SMSG_TRADE_STATUSTradeStatus::TRIAL_ACCOUNT => Self::TRIAL_ACCOUNT,
            SMSG_TRADE_STATUSTradeStatus::ONLY_CONJURED { .. } => Self::ONLY_CONJURED,
            SMSG_TRADE_STATUSTradeStatus::NOT_ON_TAPLIST { .. } => Self::NOT_ON_TAPLIST,
        }
    }
}

impl Default for SMSG_TRADE_STATUSTradeStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::BUSY
    }
}

impl SMSG_TRADE_STATUSTradeStatus {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: TradeStatus = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_TRADE_STATUSTradeStatus {
    fn size(&self) -> usize {
        match self {
            Self::BUSY =>  {
                4
            }
            Self::BEGIN_TRADE  {
                unknown1,
            } => {
                4
                + 8 // unknown1: Guid
            }
            Self::OPEN_WINDOW =>  {
                4
            }
            Self::TRADE_CANCELED =>  {
                4
            }
            Self::TRADE_ACCEPT =>  {
                4
            }
            Self::BUSY_2 =>  {
                4
            }
            Self::NO_TARGET =>  {
                4
            }
            Self::BACK_TO_TRADE =>  {
                4
            }
            Self::TRADE_COMPLETE =>  {
                4
            }
            Self::TRADE_REJECTED =>  {
                4
            }
            Self::TARGET_TO_FAR =>  {
                4
            }
            Self::WRONG_FACTION =>  {
                4
            }
            Self::CLOSE_WINDOW  {
                inventory_result,
                target_error,
                item_limit_category_id,
            } => {
                4
                + 4 // inventory_result: InventoryResult upcasted to u32
                + 1 // target_error: u8
                + 4 // item_limit_category_id: u32
            }
            Self::UNKNOWN_13 =>  {
                4
            }
            Self::IGNORE_YOU =>  {
                4
            }
            Self::YOU_STUNNED =>  {
                4
            }
            Self::TARGET_STUNNED =>  {
                4
            }
            Self::YOU_DEAD =>  {
                4
            }
            Self::TARGET_DEAD =>  {
                4
            }
            Self::YOU_LOGOUT =>  {
                4
            }
            Self::TARGET_LOGOUT =>  {
                4
            }
            Self::TRIAL_ACCOUNT =>  {
                4
            }
            Self::ONLY_CONJURED  {
                slot,
            } => {
                4
                + 1 // slot: u8
            }
            Self::NOT_ON_TAPLIST  {
                slot,
            } => {
                4
                + 1 // slot: u8
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_TRADE_STATUSTradeStatus {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

