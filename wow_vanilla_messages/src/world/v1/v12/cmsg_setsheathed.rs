use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::SheathState;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl ClientMessage for CMSG_SETSHEATHED {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sheathed: SheathState
        w.write_all(&(self.sheathed.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01e0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sheathed: SheathState
        let sheathed: SheathState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            sheathed,
        })
    }

}

