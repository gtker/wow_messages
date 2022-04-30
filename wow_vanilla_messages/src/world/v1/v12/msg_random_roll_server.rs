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
#[derive(Copy)]
pub struct MSG_RANDOM_ROLL_Server {
    pub minimum: u32,
    pub maximum: u32,
    pub actual_roll: u32,
    pub guid: Guid,
}

impl ServerMessageWrite for MSG_RANDOM_ROLL_Server {}

impl MessageBody for MSG_RANDOM_ROLL_Server {
    const OPCODE: u16 = 0x01fb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum: u32
        let minimum = crate::util::read_u32_le(r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(r)?;

        // actual_roll: u32
        let actual_roll = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            minimum,
            maximum,
            actual_roll,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        // actual_roll: u32
        w.write_all(&self.actual_roll.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for MSG_RANDOM_ROLL_Server {}

impl MaximumPossibleSized for MSG_RANDOM_ROLL_Server {
    fn maximum_possible_size() -> usize {
        4 // minimum: u32
        + 4 // maximum: u32
        + 4 // actual_roll: u32
        + 8 // guid: Guid
    }
}

