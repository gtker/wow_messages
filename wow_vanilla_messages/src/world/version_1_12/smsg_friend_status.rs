use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::FriendResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: Guid,
}

impl ServerMessage for SMSG_FRIEND_STATUS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: FriendResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0068;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: FriendResult
        let result: FriendResult = crate::util::read_u8_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            result,
            guid,
        })
    }

}

