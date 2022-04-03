use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidGroupError, RaidGroupErrorError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:58`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm#L58):
/// ```text
/// smsg SMSG_RAID_GROUP_ONLY = 0x286 {
///     u32 homebind_timer;
///     RaidGroupError error;
/// }
/// ```
pub struct SMSG_RAID_GROUP_ONLY {
    pub homebind_timer: u32,
    pub error: RaidGroupError,
}

impl WorldServerMessageWrite for SMSG_RAID_GROUP_ONLY {
    const OPCODE: u16 = 0x286;

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
impl WorldMessageBody for SMSG_RAID_GROUP_ONLY {
    type Error = SMSG_RAID_GROUP_ONLYError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // homebind_timer: u32
        let homebind_timer = crate::util::read_u32_le(r)?;

        // error: RaidGroupError
        let error = RaidGroupError::read(r)?;

        Ok(Self {
            homebind_timer,
            error,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // homebind_timer: u32
        w.write_all(&self.homebind_timer.to_le_bytes())?;

        // error: RaidGroupError
        self.error.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_RAID_GROUP_ONLY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_RAID_GROUP_ONLY {
    fn maximum_possible_size() -> usize {
        4 // homebind_timer: u32
        + RaidGroupError::size() // error: RaidGroupError
    }
}

#[derive(Debug)]
pub enum SMSG_RAID_GROUP_ONLYError {
    Io(std::io::Error),
    RaidGroupError(RaidGroupErrorError),
}

impl std::error::Error for SMSG_RAID_GROUP_ONLYError {}
impl std::fmt::Display for SMSG_RAID_GROUP_ONLYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidGroupError(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_GROUP_ONLYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidGroupErrorError> for SMSG_RAID_GROUP_ONLYError {
    fn from(e: RaidGroupErrorError) -> Self {
        Self::RaidGroupError(e)
    }
}

