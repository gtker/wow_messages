use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::Vector3d;
use crate::world::vanilla::MonsterMoveType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move_transport.wowm#L3):
/// ```text
/// smsg SMSG_MONSTER_MOVE_TRANSPORT = 0x02AE {
///     PackedGuid transport;
///     Vector3d position;
///     u32 spline_id;
///     MonsterMoveType move_type;
/// }
/// ```
pub struct SMSG_MONSTER_MOVE_TRANSPORT {
    pub transport: Guid,
    pub position: Vector3d,
    pub spline_id: u32,
    pub move_type: MonsterMoveType,
}

impl crate::Message for SMSG_MONSTER_MOVE_TRANSPORT {
    const OPCODE: u32 = 0x02ae;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // transport: PackedGuid
        self.transport.write_packed_guid_into_vec(w);

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // transport: PackedGuid
        let transport = Guid::read_packed(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(r)?;

        // move_type: MonsterMoveType
        let move_type: MonsterMoveType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            transport,
            position,
            spline_id,
            move_type,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_MONSTER_MOVE_TRANSPORT {}

impl SMSG_MONSTER_MOVE_TRANSPORT {
    pub(crate) fn size(&self) -> usize {
        self.transport.size() // transport: Guid
        + 12 // position: Vector3d
        + 4 // spline_id: u32
        + 1 // move_type: MonsterMoveType
    }
}

