use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{PvpRank, PvpRankError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new5.wowm:183`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new5.wowm#L183):
/// ```text
/// smsg SMSG_PVP_CREDIT = 0x28C {
///     u32 honor_points;
///     Guid victim;
///     PvpRank rank;
/// }
/// ```
pub struct SMSG_PVP_CREDIT {
    pub honor_points: u32,
    pub victim: Guid,
    pub rank: PvpRank,
}

impl WorldServerMessageWrite for SMSG_PVP_CREDIT {
    const OPCODE: u16 = 0x28c;

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
impl WorldMessageBody for SMSG_PVP_CREDIT {
    type Error = SMSG_PVP_CREDITError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // honor_points: u32
        let honor_points = crate::util::read_u32_le(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        // rank: PvpRank
        let rank = PvpRank::read_u32_le(r)?;

        Ok(Self {
            honor_points,
            victim,
            rank,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // honor_points: u32
        w.write_all(&self.honor_points.to_le_bytes())?;

        // victim: Guid
        self.victim.write(w)?;

        // rank: PvpRank
        self.rank.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PVP_CREDIT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PVP_CREDIT {
    fn maximum_possible_size() -> usize {
        4 // honor_points: u32
        + 8 // victim: Guid
        + 4 // rank: PvpRank upcasted to u32
    }
}

#[derive(Debug)]
pub enum SMSG_PVP_CREDITError {
    Io(std::io::Error),
    PvpRank(PvpRankError),
}

impl std::error::Error for SMSG_PVP_CREDITError {}
impl std::fmt::Display for SMSG_PVP_CREDITError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PvpRank(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PVP_CREDITError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PvpRankError> for SMSG_PVP_CREDITError {
    fn from(e: PvpRankError) -> Self {
        Self::PvpRank(e)
    }
}

