use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_AREA_TRIGGER_MESSAGE {
    pub length: u32,
    pub message: String,
}

impl ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // length: u32
        w.write_all(&self.length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x02b8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // length: u32
        let length = crate::util::read_u32_le(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            length,
            message,
        })
    }

}

impl SMSG_AREA_TRIGGER_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // length: u32
        + self.message.len() + 1 // message: CString
    }
}

