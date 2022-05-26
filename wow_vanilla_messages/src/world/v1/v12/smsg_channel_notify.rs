use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ChatNotify;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CHANNEL_NOTIFY {
    pub notify_type: ChatNotify,
    pub channel_name: String,
}

impl ServerMessage for SMSG_CHANNEL_NOTIFY {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // notify_type: ChatNotify
        w.write_all(&(self.notify_type.as_int() as u8).to_le_bytes())?;

        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0099;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // notify_type: ChatNotify
        let notify_type: ChatNotify = crate::util::read_u8_le(r)?.try_into()?;

        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            notify_type,
            channel_name,
        })
    }

}

impl SMSG_CHANNEL_NOTIFY {
    pub fn size(&self) -> usize {
        0
        + 1 // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

