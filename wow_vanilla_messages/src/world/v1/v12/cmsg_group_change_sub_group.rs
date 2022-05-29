use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GROUP_CHANGE_SUB_GROUP {
    pub name: String,
    pub group_number: u8,
}

impl ClientMessage for CMSG_GROUP_CHANGE_SUB_GROUP {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // group_number: u8
        w.write_all(&self.group_number.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x027e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // group_number: u8
        let group_number = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            group_number,
        })
    }

}

impl CMSG_GROUP_CHANGE_SUB_GROUP {
    pub(crate) fn size(&self) -> usize {
        0
        + self.name.len() + 1 // name: CString
        + 1 // group_number: u8
    }
}

