use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{MeetingStoneStatus, MeetingStoneStatusError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_MEETINGSTONE_SETQUEUE {
    pub area: Area,
    pub status: MeetingStoneStatus,
}

impl ServerMessageWrite for SMSG_MEETINGSTONE_SETQUEUE {}

impl MessageBody for SMSG_MEETINGSTONE_SETQUEUE {
    const OPCODE: u16 = 0x0295;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_MEETINGSTONE_SETQUEUEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // status: MeetingStoneStatus
        let status: MeetingStoneStatus = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            area,
            status,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // status: MeetingStoneStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

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
            // area: Area
            let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // status: MeetingStoneStatus
            let status: MeetingStoneStatus = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                area,
                status,
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
            // area: Area
            w.write_all(&(self.area.as_int() as u32).to_le_bytes()).await?;

            // status: MeetingStoneStatus
            w.write_all(&(self.status.as_int() as u8).to_le_bytes()).await?;

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
            // area: Area
            let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // status: MeetingStoneStatus
            let status: MeetingStoneStatus = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                area,
                status,
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
            // area: Area
            w.write_all(&(self.area.as_int() as u32).to_le_bytes()).await?;

            // status: MeetingStoneStatus
            w.write_all(&(self.status.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_MEETINGSTONE_SETQUEUE {}

impl MaximumPossibleSized for SMSG_MEETINGSTONE_SETQUEUE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // area: Area
        + 1 // status: MeetingStoneStatus
    }
}

#[derive(Debug)]
pub enum SMSG_MEETINGSTONE_SETQUEUEError {
    Io(std::io::Error),
    Area(AreaError),
    MeetingStoneStatus(MeetingStoneStatusError),
}

impl std::error::Error for SMSG_MEETINGSTONE_SETQUEUEError {}
impl std::fmt::Display for SMSG_MEETINGSTONE_SETQUEUEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::MeetingStoneStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<MeetingStoneStatusError> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e: MeetingStoneStatusError) -> Self {
        Self::MeetingStoneStatus(e)
    }
}

