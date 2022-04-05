use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{StableResult, StableResultError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_stable_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_stable_result.wowm#L3):
/// ```text
/// smsg SMSG_STABLE_RESULT = 0x273 {
///     StableResult result;
/// }
/// ```
pub struct SMSG_STABLE_RESULT {
    pub result: StableResult,
}

impl WorldServerMessageWrite for SMSG_STABLE_RESULT {
    const OPCODE: u16 = 0x273;

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
impl WorldMessageBody for SMSG_STABLE_RESULT {
    type Error = SMSG_STABLE_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: StableResult
        let result = StableResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: StableResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_STABLE_RESULT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_STABLE_RESULT {
    fn maximum_possible_size() -> usize {
        StableResult::size() // result: StableResult
    }
}

#[derive(Debug)]
pub enum SMSG_STABLE_RESULTError {
    Io(std::io::Error),
    StableResult(StableResultError),
}

impl std::error::Error for SMSG_STABLE_RESULTError {}
impl std::fmt::Display for SMSG_STABLE_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::StableResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_STABLE_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<StableResultError> for SMSG_STABLE_RESULTError {
    fn from(e: StableResultError) -> Self {
        Self::StableResult(e)
    }
}

