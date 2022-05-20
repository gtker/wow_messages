use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct GroupListMember {
    pub name: String,
    pub guid: Guid,
    pub is_online: u8,
}

impl GroupListMember {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // is_online: u8
        w.write_all(&self.is_online.to_le_bytes())?;

        Ok(w)
    }

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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

