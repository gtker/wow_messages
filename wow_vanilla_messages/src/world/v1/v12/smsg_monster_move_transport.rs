use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::{MonsterMoveType, MonsterMoveTypeError};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm#L3):
/// ```text
/// smsg SMSG_MONSTER_MOVE_TRANSPORT = 0x2AE {
///     PackedGuid transport;
///     f32 position_x;
///     f32 position_y;
///     f32 position_z;
///     u32 spline_id;
///     MonsterMoveType move_type;
/// }
/// ```
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub transport: Guid,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub spline_id: u32,
    pub move_type: MonsterMoveType,
}

impl WorldServerMessageWrite for SMSG_MONSTER_MOVE_TRANSPORT {
    const OPCODE: u16 = 0x2ae;

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
impl WorldMessageBody for SMSG_MONSTER_MOVE_TRANSPORT {
    type Error = SMSG_MONSTER_MOVE_TRANSPORTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // transport: PackedGuid
        let transport = Guid::read_packed(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type = MonsterMoveType::read(r)?;

        Ok(Self {
            transport,
            position_x,
            position_y,
            position_z,
            spline_id,
            move_type,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // transport: PackedGuid
        self.transport.write_packed(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        self.move_type.write(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_MONSTER_MOVE_TRANSPORT {
    fn size(&self) -> usize {
        self.transport.size() // transport: PackedGuid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // spline_id: u32
        + MonsterMoveType::size() // move_type: MonsterMoveType
    }
}

impl MaximumPossibleSized for SMSG_MONSTER_MOVE_TRANSPORT {
    fn maximum_possible_size() -> usize {
        9 // transport: PackedGuid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // spline_id: u32
        + MonsterMoveType::maximum_possible_size() // move_type: MonsterMoveType
    }
}

#[derive(Debug)]
pub enum SMSG_MONSTER_MOVE_TRANSPORTError {
    Io(std::io::Error),
    MonsterMoveType(MonsterMoveTypeError),
}

impl std::error::Error for SMSG_MONSTER_MOVE_TRANSPORTError {}
impl std::fmt::Display for SMSG_MONSTER_MOVE_TRANSPORTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::MonsterMoveType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MONSTER_MOVE_TRANSPORTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MonsterMoveTypeError> for SMSG_MONSTER_MOVE_TRANSPORTError {
    fn from(e: MonsterMoveTypeError) -> Self {
        Self::MonsterMoveType(e)
    }
}

