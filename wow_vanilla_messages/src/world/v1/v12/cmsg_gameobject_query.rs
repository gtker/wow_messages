use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
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
#[derive(Copy)]
pub struct CMSG_GAMEOBJECT_QUERY {
    pub entry_id: u32,
    pub guid: Guid,
}

impl ClientMessageWrite for CMSG_GAMEOBJECT_QUERY {}

impl MessageBody for CMSG_GAMEOBJECT_QUERY {
    const OPCODE: u16 = 0x005e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // entry_id: u32
        let entry_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            entry_id,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // entry_id: u32
        w.write_all(&self.entry_id.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_GAMEOBJECT_QUERY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_GAMEOBJECT_QUERY {
    fn maximum_possible_size() -> usize {
        4 // entry_id: u32
        + 8 // guid: Guid
    }
}

