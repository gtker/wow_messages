use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ForcedReaction;
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
pub struct SMSG_SET_FORCED_REACTIONS {
    pub reactions: Vec<ForcedReaction>,
}

impl ServerMessageWrite for SMSG_SET_FORCED_REACTIONS {}

impl MessageBody for SMSG_SET_FORCED_REACTIONS {
    const OPCODE: u16 = 0x02a5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_reactions: u32
        let amount_of_reactions = crate::util::read_u32_le(r)?;

        // reactions: ForcedReaction[amount_of_reactions]
        let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
        for i in 0..amount_of_reactions {
            reactions.push(ForcedReaction::read(r)?);
        }

        Ok(Self {
            reactions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SET_FORCED_REACTIONS {
    fn size(&self) -> usize {
        4 // amount_of_reactions: u32
        + self.reactions.iter().fold(0, |acc, x| acc + ForcedReaction::size()) // reactions: ForcedReaction[amount_of_reactions]
    }
}

impl MaximumPossibleSized for SMSG_SET_FORCED_REACTIONS {
    fn maximum_possible_size() -> usize {
        4 // amount_of_reactions: u32
        + 4294967295 * ForcedReaction::maximum_possible_size() // reactions: ForcedReaction[amount_of_reactions]
    }
}

