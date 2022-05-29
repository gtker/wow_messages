use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_ADD_IGNORE {
    pub ignore_name: String,
}

impl ClientMessage for CMSG_ADD_IGNORE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // ignore_name: CString
        w.write_all(self.ignore_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x006c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // ignore_name: CString
        let ignore_name = crate::util::read_c_string_to_vec(r)?;
        let ignore_name = String::from_utf8(ignore_name)?;

        Ok(Self {
            ignore_name,
        })
    }

}

impl CMSG_ADD_IGNORE {
    pub(crate) fn size(&self) -> usize {
        self.ignore_name.len() + 1 // ignore_name: CString
    }
}

