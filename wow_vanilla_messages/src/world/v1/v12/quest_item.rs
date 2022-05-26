use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct QuestItem {
    pub quest_id: u32,
    pub quest_icon: u32,
    pub level: u32,
    pub title: String,
}

impl QuestItem {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
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

        Ok(w)
    }
}

impl QuestItem {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

impl QuestItem {
    pub fn size(&self) -> usize {
        0
        + 4 // quest_id: u32
        + 4 // quest_icon: u32
        + 4 // level: u32
        + self.title.len() + 1 // title: CString
    }
}

