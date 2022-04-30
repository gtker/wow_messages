use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{TransferAbortReason, TransferAbortReasonError};
use crate::{WorldServerMessageWrite, MessageBody};
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
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: TransferAbortReason,
}

impl WorldServerMessageWrite for SMSG_TRANSFER_ABORTED {
    const OPCODE: u16 = 0x40;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl SMSG_TRANSFER_ABORTED {
    pub const PADDING_VALUE: u8 = 0x00;

}

impl MessageBody for SMSG_TRANSFER_ABORTED {
    type Error = SMSG_TRANSFER_ABORTEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // reason: TransferAbortReason
        let reason = TransferAbortReason::read(r)?;

        // padding: u8
        let _padding = crate::util::read_u8_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            map,
            reason,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // reason: TransferAbortReason
        self.reason.write(w)?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TRANSFER_ABORTED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TRANSFER_ABORTED {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
        + TransferAbortReason::size() // reason: TransferAbortReason
        + 1 // padding: u8
    }
}

#[derive(Debug)]
pub enum SMSG_TRANSFER_ABORTEDError {
    Io(std::io::Error),
    Map(MapError),
    TransferAbortReason(TransferAbortReasonError),
}

impl std::error::Error for SMSG_TRANSFER_ABORTEDError {}
impl std::fmt::Display for SMSG_TRANSFER_ABORTEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::TransferAbortReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRANSFER_ABORTEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_TRANSFER_ABORTEDError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<TransferAbortReasonError> for SMSG_TRANSFER_ABORTEDError {
    fn from(e: TransferAbortReasonError) -> Self {
        Self::TransferAbortReason(e)
    }
}

