use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_ATTACKSTOP {
    pub player: Guid,
    pub enemy: Guid,
    pub unknown1: u32,
}

impl ServerMessageWrite for SMSG_ATTACKSTOP {
    const OPCODE: u16 = 0x144;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as ServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as ServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_ATTACKSTOP {
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed(w)?;

        // enemy: PackedGuid
        self.enemy.write_packed(w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_ATTACKSTOP {
    fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + self.enemy.size() // enemy: PackedGuid
        + 4 // unknown1: u32
    }
}

impl MaximumPossibleSized for SMSG_ATTACKSTOP {
    fn maximum_possible_size() -> usize {
        9 // player: PackedGuid
        + 9 // enemy: PackedGuid
        + 4 // unknown1: u32
    }
}

