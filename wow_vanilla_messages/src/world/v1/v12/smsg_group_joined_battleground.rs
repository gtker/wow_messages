use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BgTypeId, BgTypeIdError};
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
#[derive(Copy)]
pub struct SMSG_GROUP_JOINED_BATTLEGROUND {
    pub id: BgTypeId,
}

impl ServerMessageWrite for SMSG_GROUP_JOINED_BATTLEGROUND {
    const OPCODE: u16 = 0x2e8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_GROUP_JOINED_BATTLEGROUND {
    type Error = SMSG_GROUP_JOINED_BATTLEGROUNDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: BgTypeId
        let id = BgTypeId::read(r)?;

        Ok(Self {
            id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: BgTypeId
        self.id.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_GROUP_JOINED_BATTLEGROUND {
    fn maximum_possible_size() -> usize {
        BgTypeId::size() // id: BgTypeId
    }
}

#[derive(Debug)]
pub enum SMSG_GROUP_JOINED_BATTLEGROUNDError {
    Io(std::io::Error),
    BgTypeId(BgTypeIdError),
}

impl std::error::Error for SMSG_GROUP_JOINED_BATTLEGROUNDError {}
impl std::fmt::Display for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BgTypeId(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BgTypeIdError> for SMSG_GROUP_JOINED_BATTLEGROUNDError {
    fn from(e: BgTypeIdError) -> Self {
        Self::BgTypeId(e)
    }
}

