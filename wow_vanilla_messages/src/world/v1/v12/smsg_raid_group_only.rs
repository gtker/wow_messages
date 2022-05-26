use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::RaidGroupError;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_RAID_GROUP_ONLY {
    pub homebind_timer: u32,
    pub error: RaidGroupError,
}

impl ServerMessage for SMSG_RAID_GROUP_ONLY {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // homebind_timer: u32
        w.write_all(&self.homebind_timer.to_le_bytes())?;

        // error: RaidGroupError
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0286;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // homebind_timer: u32
        let homebind_timer = crate::util::read_u32_le(r)?;

        // error: RaidGroupError
        let error: RaidGroupError = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            homebind_timer,
            error,
        })
    }

}

