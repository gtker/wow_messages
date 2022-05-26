use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_RESISTLOG {
    pub guid1: Guid,
    pub guid2: Guid,
    pub unknown1: u32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: u32,
    pub unknown5: u32,
}

impl ServerMessage for SMSG_RESISTLOG {}

impl SMSG_RESISTLOG {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 36], std::io::Error> {
        let mut array_w = [0u8; 36];
        let mut w = array_w.as_mut_slice();
        // guid1: Guid
        w.write_all(&self.guid1.guid().to_le_bytes())?;

        // guid2: Guid
        w.write_all(&self.guid2.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_RESISTLOG {
    const OPCODE: u16 = 0x01d6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        36
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid1: Guid
        let guid1 = Guid::read(r)?;

        // guid2: Guid
        let guid2 = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(r)?;
        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(r)?;
        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // guid1: Guid
            let guid1 = Guid::tokio_read(r).await?;

            // guid2: Guid
            let guid2 = Guid::tokio_read(r).await?;

            // unknown1: u32
            let unknown1 = crate::util::tokio_read_u32_le(r).await?;

            // unknown2: f32
            let unknown2 = crate::util::tokio_read_f32_le(r).await?;
            // unknown3: f32
            let unknown3 = crate::util::tokio_read_f32_le(r).await?;
            // unknown4: u32
            let unknown4 = crate::util::tokio_read_u32_le(r).await?;

            // unknown5: u32
            let unknown5 = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid1,
                guid2,
                unknown1,
                unknown2,
                unknown3,
                unknown4,
                unknown5,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // guid1: Guid
            let guid1 = Guid::astd_read(r).await?;

            // guid2: Guid
            let guid2 = Guid::astd_read(r).await?;

            // unknown1: u32
            let unknown1 = crate::util::astd_read_u32_le(r).await?;

            // unknown2: f32
            let unknown2 = crate::util::astd_read_f32_le(r).await?;
            // unknown3: f32
            let unknown3 = crate::util::astd_read_f32_le(r).await?;
            // unknown4: u32
            let unknown4 = crate::util::astd_read_u32_le(r).await?;

            // unknown5: u32
            let unknown5 = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guid1,
                guid2,
                unknown1,
                unknown2,
                unknown3,
                unknown4,
                unknown5,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

