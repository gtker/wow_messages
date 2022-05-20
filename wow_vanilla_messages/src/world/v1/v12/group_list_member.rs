use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GroupListMember {
    pub name: String,
    pub guid: Guid,
    pub is_online: u8,
}

impl GroupListMember {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupListMemberError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        // is_online: u8
        let is_online = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            guid,
            is_online,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        self.guid.write(w)?;

        // is_online: u8
        w.write_all(&self.is_online.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GroupListMemberError> {
        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // is_online: u8
        let is_online = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            name,
            guid,
            is_online,
        })
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guid: Guid
        self.guid.tokio_write(w).await?;

        // is_online: u8
        w.write_all(&self.is_online.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GroupListMemberError> {
        // name: CString
        let name = crate::util::astd_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // is_online: u8
        let is_online = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            name,
            guid,
            is_online,
        })
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guid: Guid
        self.guid.astd_write(w).await?;

        // is_online: u8
        w.write_all(&self.is_online.to_le_bytes()).await?;

        Ok(())
    }

}

impl GroupListMember {
    pub fn size(&self) -> usize {
        0
        + self.name.len() + 1 // name: CString
        + 8 // guid: Guid
        + 1 // is_online: u8
    }
}

#[derive(Debug)]
pub enum GroupListMemberError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for GroupListMemberError {}
impl std::fmt::Display for GroupListMemberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for GroupListMemberError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for GroupListMemberError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

