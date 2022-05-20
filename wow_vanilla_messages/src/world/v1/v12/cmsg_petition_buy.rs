use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_PETITION_BUY {
    pub npc: Guid,
    pub skip1: u32,
    pub skip2: Guid,
    pub name: String,
    pub skip3: u32,
    pub skip4: u32,
    pub skip5: u32,
    pub skip6: u32,
    pub skip7: u32,
    pub skip8: u32,
    pub skip9: u32,
    pub skip10: u32,
    pub skip11: u32,
    pub skip12: u32,
    pub skip13: u16,
    pub skip14: u8,
    pub index: u32,
    pub skip15: u32,
}

impl ClientMessageWrite for CMSG_PETITION_BUY {}

impl MessageBody for CMSG_PETITION_BUY {
    const OPCODE: u16 = 0x01bd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_PETITION_BUYError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // skip1: u32
        let skip1 = crate::util::read_u32_le(r)?;

        // skip2: Guid
        let skip2 = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // skip3: u32
        let skip3 = crate::util::read_u32_le(r)?;

        // skip4: u32
        let skip4 = crate::util::read_u32_le(r)?;

        // skip5: u32
        let skip5 = crate::util::read_u32_le(r)?;

        // skip6: u32
        let skip6 = crate::util::read_u32_le(r)?;

        // skip7: u32
        let skip7 = crate::util::read_u32_le(r)?;

        // skip8: u32
        let skip8 = crate::util::read_u32_le(r)?;

        // skip9: u32
        let skip9 = crate::util::read_u32_le(r)?;

        // skip10: u32
        let skip10 = crate::util::read_u32_le(r)?;

        // skip11: u32
        let skip11 = crate::util::read_u32_le(r)?;

        // skip12: u32
        let skip12 = crate::util::read_u32_le(r)?;

        // skip13: u16
        let skip13 = crate::util::read_u16_le(r)?;

        // skip14: u8
        let skip14 = crate::util::read_u8_le(r)?;

        // index: u32
        let index = crate::util::read_u32_le(r)?;

        // skip15: u32
        let skip15 = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            skip1,
            skip2,
            name,
            skip3,
            skip4,
            skip5,
            skip6,
            skip7,
            skip8,
            skip9,
            skip10,
            skip11,
            skip12,
            skip13,
            skip14,
            index,
            skip15,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // skip1: u32
        w.write_all(&self.skip1.to_le_bytes())?;

        // skip2: Guid
        w.write_all(&self.skip2.guid().to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // skip3: u32
        w.write_all(&self.skip3.to_le_bytes())?;

        // skip4: u32
        w.write_all(&self.skip4.to_le_bytes())?;

        // skip5: u32
        w.write_all(&self.skip5.to_le_bytes())?;

        // skip6: u32
        w.write_all(&self.skip6.to_le_bytes())?;

        // skip7: u32
        w.write_all(&self.skip7.to_le_bytes())?;

        // skip8: u32
        w.write_all(&self.skip8.to_le_bytes())?;

        // skip9: u32
        w.write_all(&self.skip9.to_le_bytes())?;

        // skip10: u32
        w.write_all(&self.skip10.to_le_bytes())?;

        // skip11: u32
        w.write_all(&self.skip11.to_le_bytes())?;

        // skip12: u32
        w.write_all(&self.skip12.to_le_bytes())?;

        // skip13: u16
        w.write_all(&self.skip13.to_le_bytes())?;

        // skip14: u8
        w.write_all(&self.skip14.to_le_bytes())?;

        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // skip15: u32
        w.write_all(&self.skip15.to_le_bytes())?;

        Ok(())
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
            // npc: Guid
            let npc = Guid::tokio_read(r).await?;

            // skip1: u32
            let skip1 = crate::util::tokio_read_u32_le(r).await?;

            // skip2: Guid
            let skip2 = Guid::tokio_read(r).await?;

            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // skip3: u32
            let skip3 = crate::util::tokio_read_u32_le(r).await?;

            // skip4: u32
            let skip4 = crate::util::tokio_read_u32_le(r).await?;

            // skip5: u32
            let skip5 = crate::util::tokio_read_u32_le(r).await?;

            // skip6: u32
            let skip6 = crate::util::tokio_read_u32_le(r).await?;

            // skip7: u32
            let skip7 = crate::util::tokio_read_u32_le(r).await?;

            // skip8: u32
            let skip8 = crate::util::tokio_read_u32_le(r).await?;

            // skip9: u32
            let skip9 = crate::util::tokio_read_u32_le(r).await?;

            // skip10: u32
            let skip10 = crate::util::tokio_read_u32_le(r).await?;

            // skip11: u32
            let skip11 = crate::util::tokio_read_u32_le(r).await?;

            // skip12: u32
            let skip12 = crate::util::tokio_read_u32_le(r).await?;

            // skip13: u16
            let skip13 = crate::util::tokio_read_u16_le(r).await?;

            // skip14: u8
            let skip14 = crate::util::tokio_read_u8_le(r).await?;

            // index: u32
            let index = crate::util::tokio_read_u32_le(r).await?;

            // skip15: u32
            let skip15 = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                npc,
                skip1,
                skip2,
                name,
                skip3,
                skip4,
                skip5,
                skip6,
                skip7,
                skip8,
                skip9,
                skip10,
                skip11,
                skip12,
                skip13,
                skip14,
                index,
                skip15,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // npc: Guid
            w.write_all(&self.npc.guid().to_le_bytes()).await?;

            // skip1: u32
            w.write_all(&self.skip1.to_le_bytes()).await?;

            // skip2: Guid
            w.write_all(&self.skip2.guid().to_le_bytes()).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // skip3: u32
            w.write_all(&self.skip3.to_le_bytes()).await?;

            // skip4: u32
            w.write_all(&self.skip4.to_le_bytes()).await?;

            // skip5: u32
            w.write_all(&self.skip5.to_le_bytes()).await?;

            // skip6: u32
            w.write_all(&self.skip6.to_le_bytes()).await?;

            // skip7: u32
            w.write_all(&self.skip7.to_le_bytes()).await?;

            // skip8: u32
            w.write_all(&self.skip8.to_le_bytes()).await?;

            // skip9: u32
            w.write_all(&self.skip9.to_le_bytes()).await?;

            // skip10: u32
            w.write_all(&self.skip10.to_le_bytes()).await?;

            // skip11: u32
            w.write_all(&self.skip11.to_le_bytes()).await?;

            // skip12: u32
            w.write_all(&self.skip12.to_le_bytes()).await?;

            // skip13: u16
            w.write_all(&self.skip13.to_le_bytes()).await?;

            // skip14: u8
            w.write_all(&self.skip14.to_le_bytes()).await?;

            // index: u32
            w.write_all(&self.index.to_le_bytes()).await?;

            // skip15: u32
            w.write_all(&self.skip15.to_le_bytes()).await?;

            Ok(())
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
            // npc: Guid
            let npc = Guid::astd_read(r).await?;

            // skip1: u32
            let skip1 = crate::util::astd_read_u32_le(r).await?;

            // skip2: Guid
            let skip2 = Guid::astd_read(r).await?;

            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // skip3: u32
            let skip3 = crate::util::astd_read_u32_le(r).await?;

            // skip4: u32
            let skip4 = crate::util::astd_read_u32_le(r).await?;

            // skip5: u32
            let skip5 = crate::util::astd_read_u32_le(r).await?;

            // skip6: u32
            let skip6 = crate::util::astd_read_u32_le(r).await?;

            // skip7: u32
            let skip7 = crate::util::astd_read_u32_le(r).await?;

            // skip8: u32
            let skip8 = crate::util::astd_read_u32_le(r).await?;

            // skip9: u32
            let skip9 = crate::util::astd_read_u32_le(r).await?;

            // skip10: u32
            let skip10 = crate::util::astd_read_u32_le(r).await?;

            // skip11: u32
            let skip11 = crate::util::astd_read_u32_le(r).await?;

            // skip12: u32
            let skip12 = crate::util::astd_read_u32_le(r).await?;

            // skip13: u16
            let skip13 = crate::util::astd_read_u16_le(r).await?;

            // skip14: u8
            let skip14 = crate::util::astd_read_u8_le(r).await?;

            // index: u32
            let index = crate::util::astd_read_u32_le(r).await?;

            // skip15: u32
            let skip15 = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                npc,
                skip1,
                skip2,
                name,
                skip3,
                skip4,
                skip5,
                skip6,
                skip7,
                skip8,
                skip9,
                skip10,
                skip11,
                skip12,
                skip13,
                skip14,
                index,
                skip15,
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
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // npc: Guid
            w.write_all(&self.npc.guid().to_le_bytes()).await?;

            // skip1: u32
            w.write_all(&self.skip1.to_le_bytes()).await?;

            // skip2: Guid
            w.write_all(&self.skip2.guid().to_le_bytes()).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // skip3: u32
            w.write_all(&self.skip3.to_le_bytes()).await?;

            // skip4: u32
            w.write_all(&self.skip4.to_le_bytes()).await?;

            // skip5: u32
            w.write_all(&self.skip5.to_le_bytes()).await?;

            // skip6: u32
            w.write_all(&self.skip6.to_le_bytes()).await?;

            // skip7: u32
            w.write_all(&self.skip7.to_le_bytes()).await?;

            // skip8: u32
            w.write_all(&self.skip8.to_le_bytes()).await?;

            // skip9: u32
            w.write_all(&self.skip9.to_le_bytes()).await?;

            // skip10: u32
            w.write_all(&self.skip10.to_le_bytes()).await?;

            // skip11: u32
            w.write_all(&self.skip11.to_le_bytes()).await?;

            // skip12: u32
            w.write_all(&self.skip12.to_le_bytes()).await?;

            // skip13: u16
            w.write_all(&self.skip13.to_le_bytes()).await?;

            // skip14: u8
            w.write_all(&self.skip14.to_le_bytes()).await?;

            // index: u32
            w.write_all(&self.index.to_le_bytes()).await?;

            // skip15: u32
            w.write_all(&self.skip15.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_PETITION_BUY {
    pub fn size(&self) -> usize {
        0
        + 8 // npc: Guid
        + 4 // skip1: u32
        + 8 // skip2: Guid
        + self.name.len() + 1 // name: CString
        + 4 // skip3: u32
        + 4 // skip4: u32
        + 4 // skip5: u32
        + 4 // skip6: u32
        + 4 // skip7: u32
        + 4 // skip8: u32
        + 4 // skip9: u32
        + 4 // skip10: u32
        + 4 // skip11: u32
        + 4 // skip12: u32
        + 2 // skip13: u16
        + 1 // skip14: u8
        + 4 // index: u32
        + 4 // skip15: u32
    }
}

#[derive(Debug)]
pub enum CMSG_PETITION_BUYError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_PETITION_BUYError {}
impl std::fmt::Display for CMSG_PETITION_BUYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_PETITION_BUYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_PETITION_BUYError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

