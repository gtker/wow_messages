use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Area;
use crate::world::v1::v12::MeetingStoneStatus;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_MEETINGSTONE_SETQUEUE {
    pub area: Area,
    pub status: MeetingStoneStatus,
}

impl SMSG_MEETINGSTONE_SETQUEUE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // status: MeetingStoneStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_MEETINGSTONE_SETQUEUE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // status: MeetingStoneStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0295;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    type Error = crate::errors::ParseError;

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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

}

