use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{NpcTextUpdate, NpcTextUpdateError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_NPC_TEXT_UPDATE {
    pub text_id: u32,
    pub probability: f32,
    pub texts: [NpcTextUpdate; 8],
}

impl ServerMessageWrite for SMSG_NPC_TEXT_UPDATE {}

impl SMSG_NPC_TEXT_UPDATE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(4360);
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_NPC_TEXT_UPDATE {
    const OPCODE: u16 = 0x0180;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_NPC_TEXT_UPDATEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // text_id: u32
        let text_id = crate::util::read_u32_le(r)?;

        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: NpcTextUpdate[8]
        let mut texts = Vec::with_capacity(8 as usize);
        for i in 0..8 {
            texts.push(NpcTextUpdate::read(r)?);
        }
        let texts = texts.try_into().unwrap();

        Ok(Self {
            text_id,
            probability,
            texts,
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
            // text_id: u32
            let text_id = crate::util::tokio_read_u32_le(r).await?;

            // probability: f32
            let probability = crate::util::tokio_read_f32_le(r).await?;
            // texts: NpcTextUpdate[8]
            let mut texts = Vec::with_capacity(8 as usize);
            for i in 0..8 {
                texts.push(NpcTextUpdate::tokio_read(r).await?);
            }
            let texts = texts.try_into().unwrap();

            Ok(Self {
                text_id,
                probability,
                texts,
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
            // text_id: u32
            let text_id = crate::util::astd_read_u32_le(r).await?;

            // probability: f32
            let probability = crate::util::astd_read_f32_le(r).await?;
            // texts: NpcTextUpdate[8]
            let mut texts = Vec::with_capacity(8 as usize);
            for i in 0..8 {
                texts.push(NpcTextUpdate::astd_read(r).await?);
            }
            let texts = texts.try_into().unwrap();

            Ok(Self {
                text_id,
                probability,
                texts,
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

impl SMSG_NPC_TEXT_UPDATE {
    pub fn size(&self) -> usize {
        0
        + 4 // text_id: u32
        + 4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

#[derive(Debug)]
pub enum SMSG_NPC_TEXT_UPDATEError {
    Io(std::io::Error),
    NpcTextUpdate(NpcTextUpdateError),
}

impl std::error::Error for SMSG_NPC_TEXT_UPDATEError {}
impl std::fmt::Display for SMSG_NPC_TEXT_UPDATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::NpcTextUpdate(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NPC_TEXT_UPDATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<NpcTextUpdateError> for SMSG_NPC_TEXT_UPDATEError {
    fn from(e: NpcTextUpdateError) -> Self {
        Self::NpcTextUpdate(e)
    }
}

