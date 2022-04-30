use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_MOVE_TELEPORT_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub time_in_msecs: u32,
}

impl ClientMessageWrite for MSG_MOVE_TELEPORT_ACK {}

impl ServerMessageWrite for MSG_MOVE_TELEPORT_ACK {}

impl MessageBody for MSG_MOVE_TELEPORT_ACK {
    const OPCODE: u16 = 0x00c7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // time_in_msecs: u32
        let time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            time_in_msecs,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time_in_msecs: u32
        w.write_all(&self.time_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for MSG_MOVE_TELEPORT_ACK {}

impl MaximumPossibleSized for MSG_MOVE_TELEPORT_ACK {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + 4 // time_in_msecs: u32
    }
}

