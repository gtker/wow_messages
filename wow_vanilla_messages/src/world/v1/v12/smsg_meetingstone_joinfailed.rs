use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{MeetingStoneFailure, MeetingStoneFailureError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
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

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
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
        self.reason.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: MeetingStoneFailure
        let reason = MeetingStoneFailure::tokio_read(r).await?;

        Ok(Self {
            reason,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        self.reason.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: MeetingStoneFailure
        let reason = MeetingStoneFailure::astd_read(r).await?;

        Ok(Self {
            reason,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        self.reason.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_MEETINGSTONE_JOINFAILED {}

impl MaximumPossibleSized for SMSG_MEETINGSTONE_JOINFAILED {
    fn maximum_possible_size() -> usize {
        MeetingStoneFailure::size() // reason: MeetingStoneFailure
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

