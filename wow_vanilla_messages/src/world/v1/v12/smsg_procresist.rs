use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{LogFormat, LogFormatError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_procresist.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_procresist.wowm#L8):
/// ```text
/// smsg SMSG_PROCRESIST = 0x260 {
///     Guid guid;
///     Guid target_guid;
///     u32 spell_id;
///     LogFormat log_format;
/// }
/// ```
pub struct SMSG_PROCRESIST {
    pub guid: Guid,
    pub target_guid: Guid,
    pub spell_id: u32,
    pub log_format: LogFormat,
}

impl WorldServerMessageWrite for SMSG_PROCRESIST {
    const OPCODE: u16 = 0x260;

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
impl WorldMessageBody for SMSG_PROCRESIST {
    type Error = SMSG_PROCRESISTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // log_format: LogFormat
        let log_format = LogFormat::read(r)?;

        Ok(Self {
            guid,
            target_guid,
            spell_id,
            log_format,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

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
        + 4 // spell_id: u32
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

