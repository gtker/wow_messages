use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{MeetingStoneFailure, MeetingStoneFailureError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_MEETINGSTONE_JOINFAILED {
    pub reason: MeetingStoneFailure,
}

impl ServerMessageWrite for SMSG_MEETINGSTONE_JOINFAILED {}

impl MessageBody for SMSG_MEETINGSTONE_JOINFAILED {
    const OPCODE: u16 = 0x02bb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_MEETINGSTONE_JOINFAILEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: MeetingStoneFailure
        let reason = MeetingStoneFailure::read(r)?;

        Ok(Self {
            reason,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        crate::util::write_u8_le(w, self.reason.as_int() as u8)?;

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
            // reason: MeetingStoneFailure
            let reason = MeetingStoneFailure::tokio_read(r).await?;

            Ok(Self {
                reason,
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
            // reason: MeetingStoneFailure
            crate::util::tokio_write_u8_le(w, self.reason.as_int() as u8).await?;

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
            // reason: MeetingStoneFailure
            let reason = MeetingStoneFailure::astd_read(r).await?;

            Ok(Self {
                reason,
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
            // reason: MeetingStoneFailure
            crate::util::astd_write_u8_le(w, self.reason.as_int() as u8).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_MEETINGSTONE_JOINFAILED {}

impl MaximumPossibleSized for SMSG_MEETINGSTONE_JOINFAILED {
    fn maximum_possible_size() -> usize {
        0
        + 1 // reason: MeetingStoneFailure
    }
}

#[derive(Debug)]
pub enum SMSG_MEETINGSTONE_JOINFAILEDError {
    Io(std::io::Error),
    MeetingStoneFailure(MeetingStoneFailureError),
}

impl std::error::Error for SMSG_MEETINGSTONE_JOINFAILEDError {}
impl std::fmt::Display for SMSG_MEETINGSTONE_JOINFAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::MeetingStoneFailure(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MEETINGSTONE_JOINFAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MeetingStoneFailureError> for SMSG_MEETINGSTONE_JOINFAILEDError {
    fn from(e: MeetingStoneFailureError) -> Self {
        Self::MeetingStoneFailure(e)
    }
}

