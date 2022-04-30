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
pub struct SMSG_SHOWTAXINODES {
    pub unknown1: u32,
    pub guid: Guid,
    pub nearest_node: u32,
    pub nodes: Vec<u32>,
}

impl ServerMessageWrite for SMSG_SHOWTAXINODES {
    const OPCODE: u16 = 0x1a9;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_SHOWTAXINODES {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        // nearest_node: u32
        let nearest_node = crate::util::read_u32_le(r)?;

        // nodes: u32[-]
        let mut current_size = {
            4 // unknown1: u32
            + 8 // guid: Guid
            + 4 // nearest_node: u32
        };
        let mut nodes = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            nodes.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unknown1,
            guid,
            nearest_node,
            nodes,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        // nearest_node: u32
        w.write_all(&self.nearest_node.to_le_bytes())?;

        // nodes: u32[-]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SHOWTAXINODES {
    fn size(&self) -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[-]
    }
}

impl MaximumPossibleSized for SMSG_SHOWTAXINODES {
    fn maximum_possible_size() -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + 65536 // nodes: u32[-]
    }
}

