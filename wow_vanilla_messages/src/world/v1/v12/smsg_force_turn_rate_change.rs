use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_FORCE_TURN_RATE_CHANGE {
    pub guid: Guid,
    pub move_event: u32,
    pub speed: f32,
}

impl ServerMessageWrite for SMSG_FORCE_TURN_RATE_CHANGE {}

impl MessageBody for SMSG_FORCE_TURN_RATE_CHANGE {
    const OPCODE: u16 = 0x02de;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // move_event: u32
        let move_event = crate::util::read_u32_le(r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            move_event,
            speed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // move_event: u32
        w.write_all(&self.move_event.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_FORCE_TURN_RATE_CHANGE {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // move_event: u32
        + 4 // speed: f32
    }
}

impl MaximumPossibleSized for SMSG_FORCE_TURN_RATE_CHANGE {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
        + 4 // move_event: u32
        + 4 // speed: f32
    }
}

