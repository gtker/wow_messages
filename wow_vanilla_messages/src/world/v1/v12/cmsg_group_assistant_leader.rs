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
pub struct CMSG_GROUP_ASSISTANT_LEADER {
    pub guid: Guid,
    pub set_assistant: u8,
}

impl ClientMessageWrite for CMSG_GROUP_ASSISTANT_LEADER {
    const OPCODE: u32 = 0x28f;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for CMSG_GROUP_ASSISTANT_LEADER {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // set_assistant: u8
        let set_assistant = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            set_assistant,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // set_assistant: u8
        w.write_all(&self.set_assistant.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_GROUP_ASSISTANT_LEADER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_GROUP_ASSISTANT_LEADER {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 1 // set_assistant: u8
    }
}

