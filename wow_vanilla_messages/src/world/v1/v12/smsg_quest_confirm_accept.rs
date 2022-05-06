use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_QUEST_CONFIRM_ACCEPT {
    pub quest_id: u32,
    pub quest_title: String,
    pub guid: Guid,
}

impl ServerMessageWrite for SMSG_QUEST_CONFIRM_ACCEPT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_QUEST_CONFIRM_ACCEPT {
    const OPCODE: u16 = 0x019c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUEST_CONFIRM_ACCEPTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_title: CString
        let quest_title = crate::util::read_c_string_to_vec(r)?;
        let quest_title = String::from_utf8(quest_title)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_title: CString
        w.write_all(self.quest_title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::tokio_read_u32_le(r).await?;

        // quest_title: CString
        let quest_title = crate::util::tokio_read_c_string_to_vec(r).await?;
        let quest_title = String::from_utf8(quest_title)?;

        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes()).await?;

        // quest_title: CString
        w.write_all(self.quest_title.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guid: Guid
        self.guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::astd_read_u32_le(r).await?;

        // quest_title: CString
        let quest_title = crate::util::astd_read_c_string_to_vec(r).await?;
        let quest_title = String::from_utf8(quest_title)?;

        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes()).await?;

        // quest_title: CString
        w.write_all(self.quest_title.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guid: Guid
        self.guid.astd_write(w).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_QUEST_CONFIRM_ACCEPT {
    fn size(&self) -> usize {
        0
        + 4 // quest_id: u32
        + self.quest_title.len() + 1 // quest_title: CString
        + 8 // guid: Guid
    }
}

impl MaximumPossibleSized for SMSG_QUEST_CONFIRM_ACCEPT {
    fn maximum_possible_size() -> usize {
        0
        + 4 // quest_id: u32
        + 256 // quest_title: CString
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum SMSG_QUEST_CONFIRM_ACCEPTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUEST_CONFIRM_ACCEPTError {}
impl std::fmt::Display for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

