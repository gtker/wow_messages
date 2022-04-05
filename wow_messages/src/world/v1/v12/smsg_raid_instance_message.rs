use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{RaidInstanceMessage, RaidInstanceMessageError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:414`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L414):
/// ```text
/// smsg SMSG_RAID_INSTANCE_MESSAGE = 0x2FA {
///     RaidInstanceMessage message_type;
///     Map map;
///     u32 time_left;
/// }
/// ```
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub time_left: u32,
}

impl WorldServerMessageWrite for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u16 = 0x2fa;

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
impl WorldMessageBody for SMSG_RAID_INSTANCE_MESSAGE {
    type Error = SMSG_RAID_INSTANCE_MESSAGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: RaidInstanceMessage
        let message_type = RaidInstanceMessage::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(r)?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        self.message_type.write(w)?;

        // map: Map
        self.map.write(w)?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_RAID_INSTANCE_MESSAGE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_RAID_INSTANCE_MESSAGE {
    fn maximum_possible_size() -> usize {
        RaidInstanceMessage::size() // message_type: RaidInstanceMessage
        + Map::size() // map: Map
        + 4 // time_left: u32
    }
}

#[derive(Debug)]
pub enum SMSG_RAID_INSTANCE_MESSAGEError {
    Io(std::io::Error),
    Map(MapError),
    RaidInstanceMessage(RaidInstanceMessageError),
}

impl std::error::Error for SMSG_RAID_INSTANCE_MESSAGEError {}
impl std::fmt::Display for SMSG_RAID_INSTANCE_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::RaidInstanceMessage(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<RaidInstanceMessageError> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e: RaidInstanceMessageError) -> Self {
        Self::RaidInstanceMessage(e)
    }
}

