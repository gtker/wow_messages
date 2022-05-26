use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PLAYED_TIME {
    pub total_played_time: u32,
    pub level_played_time: u32,
}

impl SMSG_PLAYED_TIME {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // total_played_time: u32
        w.write_all(&self.total_played_time.to_le_bytes())?;

        // level_played_time: u32
        w.write_all(&self.level_played_time.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PLAYED_TIME {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // total_played_time: u32
        w.write_all(&self.total_played_time.to_le_bytes())?;

        // level_played_time: u32
        w.write_all(&self.level_played_time.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01cd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // total_played_time: u32
        let total_played_time = crate::util::read_u32_le(r)?;

        // level_played_time: u32
        let level_played_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            total_played_time,
            level_played_time,
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
            // total_played_time: u32
            let total_played_time = crate::util::tokio_read_u32_le(r).await?;

            // level_played_time: u32
            let level_played_time = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                total_played_time,
                level_played_time,
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
            // total_played_time: u32
            let total_played_time = crate::util::astd_read_u32_le(r).await?;

            // level_played_time: u32
            let level_played_time = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                total_played_time,
                level_played_time,
            })
        })
    }

}

