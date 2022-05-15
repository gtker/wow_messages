use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{DuelWinnerReason, DuelWinnerReasonError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_DUEL_WINNER {
    pub reason: DuelWinnerReason,
    pub opponent_name: String,
    pub initiator_name: String,
}

impl ServerMessageWrite for SMSG_DUEL_WINNER {}

impl MessageBody for SMSG_DUEL_WINNER {
    const OPCODE: u16 = 0x016b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_DUEL_WINNERError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: DuelWinnerReason
        let reason: DuelWinnerReason = crate::util::read_u8_le(r)?.try_into()?;

        // opponent_name: CString
        let opponent_name = crate::util::read_c_string_to_vec(r)?;
        let opponent_name = String::from_utf8(opponent_name)?;

        // initiator_name: CString
        let initiator_name = crate::util::read_c_string_to_vec(r)?;
        let initiator_name = String::from_utf8(initiator_name)?;

        Ok(Self {
            reason,
            opponent_name,
            initiator_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: DuelWinnerReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // opponent_name: CString
        w.write_all(self.opponent_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // initiator_name: CString
        w.write_all(self.initiator_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

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
            // reason: DuelWinnerReason
            let reason: DuelWinnerReason = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // opponent_name: CString
            let opponent_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let opponent_name = String::from_utf8(opponent_name)?;

            // initiator_name: CString
            let initiator_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let initiator_name = String::from_utf8(initiator_name)?;

            Ok(Self {
                reason,
                opponent_name,
                initiator_name,
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
            // reason: DuelWinnerReason
            w.write_all(&(self.reason.as_int() as u8).to_le_bytes()).await?;

            // opponent_name: CString
            w.write_all(self.opponent_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // initiator_name: CString
            w.write_all(self.initiator_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

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
            // reason: DuelWinnerReason
            let reason: DuelWinnerReason = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // opponent_name: CString
            let opponent_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let opponent_name = String::from_utf8(opponent_name)?;

            // initiator_name: CString
            let initiator_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let initiator_name = String::from_utf8(initiator_name)?;

            Ok(Self {
                reason,
                opponent_name,
                initiator_name,
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
            // reason: DuelWinnerReason
            w.write_all(&(self.reason.as_int() as u8).to_le_bytes()).await?;

            // opponent_name: CString
            w.write_all(self.opponent_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // initiator_name: CString
            w.write_all(self.initiator_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_DUEL_WINNER {
    fn size(&self) -> usize {
        0
        + 1 // reason: DuelWinnerReason
        + self.opponent_name.len() + 1 // opponent_name: CString
        + self.initiator_name.len() + 1 // initiator_name: CString
    }
}

impl MaximumPossibleSized for SMSG_DUEL_WINNER {
    fn maximum_possible_size() -> usize {
        0
        + 1 // reason: DuelWinnerReason
        + 256 // opponent_name: CString
        + 256 // initiator_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_DUEL_WINNERError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    DuelWinnerReason(DuelWinnerReasonError),
}

impl std::error::Error for SMSG_DUEL_WINNERError {}
impl std::fmt::Display for SMSG_DUEL_WINNERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::DuelWinnerReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_DUEL_WINNERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_DUEL_WINNERError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<DuelWinnerReasonError> for SMSG_DUEL_WINNERError {
    fn from(e: DuelWinnerReasonError) -> Self {
        Self::DuelWinnerReason(e)
    }
}

