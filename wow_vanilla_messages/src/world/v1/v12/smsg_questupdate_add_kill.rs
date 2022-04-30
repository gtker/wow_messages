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
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    pub create_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: Guid,
}

impl ServerMessageWrite for SMSG_QUESTUPDATE_ADD_KILL {}

impl MessageBody for SMSG_QUESTUPDATE_ADD_KILL {
    const OPCODE: u16 = 0x0199;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // create_id: u32
        let create_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            create_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // create_id: u32
        w.write_all(&self.create_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUESTUPDATE_ADD_KILL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_QUESTUPDATE_ADD_KILL {
    fn maximum_possible_size() -> usize {
        4 // quest_id: u32
        + 4 // create_id: u32
        + 4 // kill_count: u32
        + 4 // required_kill_count: u32
        + 8 // guid: Guid
    }
}

