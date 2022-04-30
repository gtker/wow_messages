use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{LogFormat, LogFormatError};
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
pub struct SMSG_PROCRESIST {
    pub guid: Guid,
    pub target_guid: Guid,
    pub id: u32,
    pub log_format: LogFormat,
}

impl ServerMessageWrite for SMSG_PROCRESIST {}

impl MessageBody for SMSG_PROCRESIST {
    const OPCODE: u16 = 0x0260;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PROCRESISTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // log_format: LogFormat
        let log_format = LogFormat::read(r)?;

        Ok(Self {
            guid,
            target_guid,
            id,
            log_format,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // log_format: LogFormat
        self.log_format.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PROCRESIST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PROCRESIST {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 8 // target_guid: Guid
        + 4 // id: u32
        + LogFormat::size() // log_format: LogFormat
    }
}

#[derive(Debug)]
pub enum SMSG_PROCRESISTError {
    Io(std::io::Error),
    LogFormat(LogFormatError),
}

impl std::error::Error for SMSG_PROCRESISTError {}
impl std::fmt::Display for SMSG_PROCRESISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LogFormat(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PROCRESISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LogFormatError> for SMSG_PROCRESISTError {
    fn from(e: LogFormatError) -> Self {
        Self::LogFormat(e)
    }
}

