use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{InstanceResetFailedReason, InstanceResetFailedReasonError};
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
pub struct SMSG_INSTANCE_RESET_FAILED {
    pub reason: InstanceResetFailedReason,
    pub map: Map,
}

impl ServerMessageWrite for SMSG_INSTANCE_RESET_FAILED {
    const OPCODE: u16 = 0x31f;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_INSTANCE_RESET_FAILED {
    type Error = SMSG_INSTANCE_RESET_FAILEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: InstanceResetFailedReason
        let reason = InstanceResetFailedReason::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        Ok(Self {
            reason,
            map,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: InstanceResetFailedReason
        self.reason.write(w)?;

        // map: Map
        self.map.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_INSTANCE_RESET_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_INSTANCE_RESET_FAILED {
    fn maximum_possible_size() -> usize {
        InstanceResetFailedReason::size() // reason: InstanceResetFailedReason
        + Map::size() // map: Map
    }
}

#[derive(Debug)]
pub enum SMSG_INSTANCE_RESET_FAILEDError {
    Io(std::io::Error),
    InstanceResetFailedReason(InstanceResetFailedReasonError),
    Map(MapError),
}

impl std::error::Error for SMSG_INSTANCE_RESET_FAILEDError {}
impl std::fmt::Display for SMSG_INSTANCE_RESET_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InstanceResetFailedReason(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_INSTANCE_RESET_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<InstanceResetFailedReasonError> for SMSG_INSTANCE_RESET_FAILEDError {
    fn from(e: InstanceResetFailedReasonError) -> Self {
        Self::InstanceResetFailedReason(e)
    }
}

impl From<MapError> for SMSG_INSTANCE_RESET_FAILEDError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

