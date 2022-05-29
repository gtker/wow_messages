use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_ATTACKSTOP {
    pub player: Guid,
    pub enemy: Guid,
    pub unknown1: u32,
}

impl ServerMessage for SMSG_ATTACKSTOP {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player: PackedGuid
        w.write_all(&self.player.packed_guid())?;

        // enemy: PackedGuid
        w.write_all(&self.enemy.packed_guid())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0144;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // enemy: PackedGuid
        let enemy = Guid::read_packed(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

}

impl SMSG_ATTACKSTOP {
    pub(crate) fn size(&self) -> usize {
        0
        + self.player.size() // player: Guid
        + self.enemy.size() // enemy: Guid
        + 4 // unknown1: u32
    }
}

