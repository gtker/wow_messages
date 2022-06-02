use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::MonsterMoveType;
use crate::world::version_1_12::Vector3d;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L11):
/// ```text
/// smsg SMSG_MONSTER_MOVE = 0x00DD {
///     PackedGuid guid;
///     Vector3d position;
///     u32 spline_id;
///     MonsterMoveType move_type;
/// }
/// ```
pub struct SMSG_MONSTER_MOVE {
    pub guid: Guid,
    pub position: Vector3d,
    pub spline_id: u32,
    pub move_type: MonsterMoveType,
}

impl ServerMessage for SMSG_MONSTER_MOVE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00dd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            position,
            spline_id,
            move_type,
        })
    }

}

impl SMSG_MONSTER_MOVE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 12 // position: Vector3d
        + 4 // spline_id: u32
        + 1 // move_type: MonsterMoveType
    }
}

