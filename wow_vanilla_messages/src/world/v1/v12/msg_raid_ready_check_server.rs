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
pub struct MSG_RAID_READY_CHECK_Server {
    pub state_check: Option<MSG_RAID_READY_CHECK_Server_state_check>,
}

impl ServerMessageWrite for MSG_RAID_READY_CHECK_Server {}

impl MessageBody for MSG_RAID_READY_CHECK_Server {
    const OPCODE: u16 = 0x0322;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional state_check
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
        };
        let state_check = if current_size < body_size as usize {
            // guid: Guid
            let guid = Guid::read(r)?;

            // state: u8
            let state = crate::util::read_u8_le(r)?;

            Some(MSG_RAID_READY_CHECK_Server_state_check {
                guid,
                state,
            })
        } else {
            None
        };

        Ok(Self {
            state_check,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // optional state_check
        if let Some(v) = &self.state_check {
            // guid: Guid
            v.guid.write(w)?;

            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }
}

impl VariableSized for MSG_RAID_READY_CHECK_Server {
    fn size(&self) -> usize {
        {
            if let Some(v) = &self.state_check {
                v.size()
            } else {
                0
            }
        } // optional state_check
    }
}

impl MaximumPossibleSized for MSG_RAID_READY_CHECK_Server {
    fn maximum_possible_size() -> usize {
        65536 // optional state_check
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSG_RAID_READY_CHECK_Server_state_check {
    pub guid: Guid,
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_Server_state_check {
    pub fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // state: u8
    }
}

