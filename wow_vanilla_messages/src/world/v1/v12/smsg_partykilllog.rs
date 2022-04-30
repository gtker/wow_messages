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
pub struct SMSG_PARTYKILLLOG {
    pub player_with_killing_blow: Guid,
    pub victim: Guid,
}

impl ServerMessageWrite for SMSG_PARTYKILLLOG {}

impl MessageBody for SMSG_PARTYKILLLOG {
    const OPCODE: u16 = 0x01f5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_with_killing_blow: Guid
        let player_with_killing_blow = Guid::read(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_with_killing_blow: Guid
        self.player_with_killing_blow.write(w)?;

        // victim: Guid
        self.victim.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PARTYKILLLOG {}

impl MaximumPossibleSized for SMSG_PARTYKILLLOG {
    fn maximum_possible_size() -> usize {
        8 // player_with_killing_blow: Guid
        + 8 // victim: Guid
    }
}

