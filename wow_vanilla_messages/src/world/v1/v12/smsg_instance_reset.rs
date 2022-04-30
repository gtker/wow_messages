use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
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
pub struct SMSG_INSTANCE_RESET {
    pub map: Map,
}

impl ServerMessageWrite for SMSG_INSTANCE_RESET {
    const OPCODE: u16 = 0x31e;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_INSTANCE_RESET {
    type Error = SMSG_INSTANCE_RESETError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        Ok(Self {
            map,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_INSTANCE_RESET {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_INSTANCE_RESET {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
    }
}

#[derive(Debug)]
pub enum SMSG_INSTANCE_RESETError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for SMSG_INSTANCE_RESETError {}
impl std::fmt::Display for SMSG_INSTANCE_RESETError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_INSTANCE_RESETError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_INSTANCE_RESETError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

