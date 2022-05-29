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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // is_online: u8
        w.write_all(&self.is_online.to_le_bytes())?;

        Ok(())
    }
}

impl GroupListMember {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

impl GroupListMember {
    pub(crate) fn size(&self) -> usize {
        0
        + self.name.len() + 1 // name: CString
        + 8 // guid: Guid
        + 1 // is_online: u8
    }
}

