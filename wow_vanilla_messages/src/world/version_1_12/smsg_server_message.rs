use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::ServerMessageType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SERVER_MESSAGE {
    pub message_type: ServerMessageType,
    pub message: String,
}

impl ServerMessage for SMSG_SERVER_MESSAGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_type: ServerMessageType
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0291;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // message_type: ServerMessageType
        let message_type: ServerMessageType = crate::util::read_u32_le(r)?.try_into()?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            message_type,
            message,
        })
    }

}

impl SMSG_SERVER_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // message_type: ServerMessageType
        + self.message.len() + 1 // message: CString
    }
}

