use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct QuestItem {
    pub quest_id: u32,
    pub quest_icon: u32,
    pub level: u32,
    pub title: String,
}

impl QuestItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestItemError> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_icon: u32
        let quest_icon = crate::util::read_u32_le(r)?;

        // level: u32
        let level = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        Ok(Self {
            quest_id,
            quest_icon,
            level,
            title,
        })
    }

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_icon: u32
        w.write_all(&self.quest_icon.to_le_bytes())?;

        // level: u32
        w.write_all(&self.level.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestItemError> {
        // quest_id: u32
        let quest_id = crate::util::tokio_read_u32_le(r).await?;

        // quest_icon: u32
        let quest_icon = crate::util::tokio_read_u32_le(r).await?;

        // level: u32
        let level = crate::util::tokio_read_u32_le(r).await?;

        // title: CString
        let title = crate::util::tokio_read_c_string_to_vec(r).await?;
        let title = String::from_utf8(title)?;

        Ok(Self {
            quest_id,
            quest_icon,
            level,
            title,
        })
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes()).await?;

        // quest_icon: u32
        w.write_all(&self.quest_icon.to_le_bytes()).await?;

        // level: u32
        w.write_all(&self.level.to_le_bytes()).await?;

        // title: CString
        w.write_all(self.title.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, QuestItemError> {
        // quest_id: u32
        let quest_id = crate::util::astd_read_u32_le(r).await?;

        // quest_icon: u32
        let quest_icon = crate::util::astd_read_u32_le(r).await?;

        // level: u32
        let level = crate::util::astd_read_u32_le(r).await?;

        // title: CString
        let title = crate::util::astd_read_c_string_to_vec(r).await?;
        let title = String::from_utf8(title)?;

        Ok(Self {
            quest_id,
            quest_icon,
            level,
            title,
        })
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes()).await?;

        // quest_icon: u32
        w.write_all(&self.quest_icon.to_le_bytes()).await?;

        // level: u32
        w.write_all(&self.level.to_le_bytes()).await?;

        // title: CString
        w.write_all(self.title.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

}

impl VariableSized for QuestItem {
    fn size(&self) -> usize {
        0
        + 4 // quest_id: u32
        + 4 // quest_icon: u32
        + 4 // level: u32
        + self.title.len() + 1 // title: CString
    }
}

impl MaximumPossibleSized for QuestItem {
    fn maximum_possible_size() -> usize {
        0
        + 4 // quest_id: u32
        + 4 // quest_icon: u32
        + 4 // level: u32
        + 256 // title: CString
    }
}

#[derive(Debug)]
pub enum QuestItemError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for QuestItemError {}
impl std::fmt::Display for QuestItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for QuestItemError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for QuestItemError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

