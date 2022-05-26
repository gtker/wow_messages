use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SET_ACTION_BUTTON {
    pub button: u8,
    pub action_type: u32,
}

impl ClientMessage for CMSG_SET_ACTION_BUTTON {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action_type: u32
        w.write_all(&self.action_type.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0128;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // button: u8
        let button = crate::util::read_u8_le(r)?;

        // action_type: u32
        let action_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            button,
            action_type,
        })
    }

}

