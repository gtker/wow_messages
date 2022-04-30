use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{QuestItem, QuestItemError};
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
pub struct SMSG_QUESTGIVER_QUEST_LIST {
    pub npc: Guid,
    pub title: String,
    pub emote_delay: u32,
    pub emote: u32,
    pub quest_items: Vec<QuestItem>,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_QUEST_LIST {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_QUESTGIVER_QUEST_LIST {
    const OPCODE: u16 = 0x0185;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUESTGIVER_QUEST_LISTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // emote_delay: u32
        let emote_delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // amount_of_entries: u8
        let amount_of_entries = crate::util::read_u8_le(r)?;

        // quest_items: QuestItem[amount_of_entries]
        let mut quest_items = Vec::with_capacity(amount_of_entries as usize);
        for i in 0..amount_of_entries {
            quest_items.push(QuestItem::read(r)?);
        }

        Ok(Self {
            npc,
            title,
            emote_delay,
            emote,
            quest_items,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // amount_of_entries: u8
        w.write_all(&(self.quest_items.len() as u8).to_le_bytes())?;

        // quest_items: QuestItem[amount_of_entries]
        for i in self.quest_items.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::tokio_read(r).await?;

        // title: CString
        let title = crate::util::tokio_read_c_string_to_vec(r).await?;
        let title = String::from_utf8(title)?;

        // emote_delay: u32
        let emote_delay = crate::util::tokio_read_u32_le(r).await?;

        // emote: u32
        let emote = crate::util::tokio_read_u32_le(r).await?;

        // amount_of_entries: u8
        let amount_of_entries = crate::util::tokio_read_u8_le(r).await?;

        // quest_items: QuestItem[amount_of_entries]
        let mut quest_items = Vec::with_capacity(amount_of_entries as usize);
        for i in 0..amount_of_entries {
            quest_items.push(QuestItem::tokio_read(r).await?);
        }

        Ok(Self {
            npc,
            title,
            emote_delay,
            emote,
            quest_items,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.tokio_write(w).await?;

        // title: CString
        w.write_all(self.title.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes()).await?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes()).await?;

        // amount_of_entries: u8
        w.write_all(&(self.quest_items.len() as u8).to_le_bytes()).await?;

        // quest_items: QuestItem[amount_of_entries]
        for i in self.quest_items.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::astd_read(r).await?;

        // title: CString
        let title = crate::util::astd_read_c_string_to_vec(r).await?;
        let title = String::from_utf8(title)?;

        // emote_delay: u32
        let emote_delay = crate::util::astd_read_u32_le(r).await?;

        // emote: u32
        let emote = crate::util::astd_read_u32_le(r).await?;

        // amount_of_entries: u8
        let amount_of_entries = crate::util::astd_read_u8_le(r).await?;

        // quest_items: QuestItem[amount_of_entries]
        let mut quest_items = Vec::with_capacity(amount_of_entries as usize);
        for i in 0..amount_of_entries {
            quest_items.push(QuestItem::astd_read(r).await?);
        }

        Ok(Self {
            npc,
            title,
            emote_delay,
            emote,
            quest_items,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.astd_write(w).await?;

        // title: CString
        w.write_all(self.title.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // emote_delay: u32
        w.write_all(&self.emote_delay.to_le_bytes()).await?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes()).await?;

        // amount_of_entries: u8
        w.write_all(&(self.quest_items.len() as u8).to_le_bytes()).await?;

        // quest_items: QuestItem[amount_of_entries]
        for i in self.quest_items.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_QUESTGIVER_QUEST_LIST {
    fn size(&self) -> usize {
        8 // npc: Guid
        + self.title.len() + 1 // title: CString and Null Terminator
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 1 // amount_of_entries: u8
        + self.quest_items.iter().fold(0, |acc, x| acc + x.size()) // quest_items: QuestItem[amount_of_entries]
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_QUEST_LIST {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 256 // title: CString
        + 4 // emote_delay: u32
        + 4 // emote: u32
        + 1 // amount_of_entries: u8
        + 255 * QuestItem::maximum_possible_size() // quest_items: QuestItem[amount_of_entries]
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_QUEST_LISTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    QuestItem(QuestItemError),
}

impl std::error::Error for SMSG_QUESTGIVER_QUEST_LISTError {}
impl std::fmt::Display for SMSG_QUESTGIVER_QUEST_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::QuestItem(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_QUEST_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUESTGIVER_QUEST_LISTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<QuestItemError> for SMSG_QUESTGIVER_QUEST_LISTError {
    fn from(e: QuestItemError) -> Self {
        Self::QuestItem(e)
    }
}

