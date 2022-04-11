use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{MeetingStoneStatus, MeetingStoneStatusError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_MEETINGSTONE_SETQUEUE {
    pub area: Area,
    pub status: MeetingStoneStatus,
}

impl WorldServerMessageWrite for SMSG_MEETINGSTONE_SETQUEUE {
    const OPCODE: u16 = 0x295;

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
impl WorldMessageBody for SMSG_MEETINGSTONE_SETQUEUE {
    type Error = SMSG_MEETINGSTONE_SETQUEUEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // area: Area
        let area = Area::read(r)?;

        // status: MeetingStoneStatus
        let status = MeetingStoneStatus::read(r)?;

        Ok(Self {
            area,
            status,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // area: Area
        self.area.write(w)?;

        // status: MeetingStoneStatus
        self.status.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_MEETINGSTONE_SETQUEUE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_MEETINGSTONE_SETQUEUE {
    fn maximum_possible_size() -> usize {
        Area::size() // area: Area
        + MeetingStoneStatus::size() // status: MeetingStoneStatus
    }
}

#[derive(Debug)]
pub enum SMSG_MEETINGSTONE_SETQUEUEError {
    Io(std::io::Error),
    Area(AreaError),
    MeetingStoneStatus(MeetingStoneStatusError),
}

impl std::error::Error for SMSG_MEETINGSTONE_SETQUEUEError {}
impl std::fmt::Display for SMSG_MEETINGSTONE_SETQUEUEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::MeetingStoneStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<MeetingStoneStatusError> for SMSG_MEETINGSTONE_SETQUEUEError {
    fn from(e: MeetingStoneStatusError) -> Self {
        Self::MeetingStoneStatus(e)
    }
}

