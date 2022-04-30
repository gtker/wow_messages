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
pub struct CMSG_SUMMON_RESPONSE {
    pub summoner_guid: Guid,
}

impl ClientMessageWrite for CMSG_SUMMON_RESPONSE {
    const OPCODE: u32 = 0x2ac;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for CMSG_SUMMON_RESPONSE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // summoner_guid: Guid
        let summoner_guid = Guid::read(r)?;

        Ok(Self {
            summoner_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // summoner_guid: Guid
        self.summoner_guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_SUMMON_RESPONSE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_SUMMON_RESPONSE {
    fn maximum_possible_size() -> usize {
        8 // summoner_guid: Guid
    }
}

