use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FriendResult, FriendResultError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:171`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L171):
/// ```text
/// smsg SMSG_FRIEND_STATUS = 0x68 {
///     FriendResult result;
///     u64 guid;
/// }
/// ```
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: u64,
}

impl WorldServerMessageWrite for SMSG_FRIEND_STATUS {
    const OPCODE: u16 = 0x68;

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
impl WorldMessageBody for SMSG_FRIEND_STATUS {
    type Error = SMSG_FRIEND_STATUSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: FriendResult
        let result = FriendResult::read(r)?;

        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            result,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: FriendResult
        self.result.write(w)?;

        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_FRIEND_STATUS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_FRIEND_STATUS {
    fn maximum_possible_size() -> usize {
        FriendResult::size() // result: FriendResult
        + 8 // guid: u64
    }
}

#[derive(Debug)]
pub enum SMSG_FRIEND_STATUSError {
    Io(std::io::Error),
    FriendResult(FriendResultError),
}

impl std::error::Error for SMSG_FRIEND_STATUSError {}
impl std::fmt::Display for SMSG_FRIEND_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::FriendResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_FRIEND_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FriendResultError> for SMSG_FRIEND_STATUSError {
    fn from(e: FriendResultError) -> Self {
        Self::FriendResult(e)
    }
}

