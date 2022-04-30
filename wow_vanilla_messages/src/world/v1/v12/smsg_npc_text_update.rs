use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{NpcTextUpdate, NpcTextUpdateError};
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
pub struct SMSG_NPC_TEXT_UPDATE {
    pub text_id: u32,
    pub probability: f32,
    pub texts: [NpcTextUpdate; 8],
}

impl ServerMessageWrite for SMSG_NPC_TEXT_UPDATE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_NPC_TEXT_UPDATE {
    const OPCODE: u16 = 0x0180;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_NPC_TEXT_UPDATEError;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes()).await?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes()).await?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes()).await?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes()).await?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_NPC_TEXT_UPDATE {
    fn size(&self) -> usize {
        4 // text_id: u32
        + 4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

impl MaximumPossibleSized for SMSG_NPC_TEXT_UPDATE {
    fn maximum_possible_size() -> usize {
        4 // text_id: u32
        + 4 // probability: f32
        + 8 * NpcTextUpdate::maximum_possible_size() // texts: NpcTextUpdate[8]
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

