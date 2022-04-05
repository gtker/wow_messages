use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidInfo, RaidInfoError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:4082`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L4082):
/// ```text
/// smsg SMSG_RAID_INSTANCE_INFO = 0x2CC {
///     u32 amount_of_raid_infos;
///     RaidInfo[amount_of_raid_infos] raid_infos;
/// }
/// ```
pub struct SMSG_RAID_INSTANCE_INFO {
    pub amount_of_raid_infos: u32,
    pub raid_infos: Vec<RaidInfo>,
}

impl WorldServerMessageWrite for SMSG_RAID_INSTANCE_INFO {
    const OPCODE: u16 = 0x2cc;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_RAID_INSTANCE_INFO {
    type Error = SMSG_RAID_INSTANCE_INFOError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::read_u32_le(r)?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);
        for i in 0..amount_of_raid_infos {
            raid_infos.push(RaidInfo::read(r)?);
        }

        Ok(Self {
            amount_of_raid_infos,
            raid_infos,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes())?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_RAID_INSTANCE_INFO {
    fn size(&self) -> usize {
        4 // amount_of_raid_infos: u32
        + self.raid_infos.iter().fold(0, |acc, x| acc + RaidInfo::size()) // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

impl MaximumPossibleSized for SMSG_RAID_INSTANCE_INFO {
    fn maximum_possible_size() -> usize {
        4 // amount_of_raid_infos: u32
        + 4294967295 * RaidInfo::maximum_possible_size() // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

#[derive(Debug)]
pub enum SMSG_RAID_INSTANCE_INFOError {
    Io(std::io::Error),
    RaidInfo(RaidInfoError),
}

impl std::error::Error for SMSG_RAID_INSTANCE_INFOError {}
impl std::fmt::Display for SMSG_RAID_INSTANCE_INFOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidInfo(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_INSTANCE_INFOError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidInfoError> for SMSG_RAID_INSTANCE_INFOError {
    fn from(e: RaidInfoError) -> Self {
        Self::RaidInfo(e)
    }
}

