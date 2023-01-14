use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_move_set_collision_hgt.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_move_set_collision_hgt.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_SET_COLLISION_HGT = 0x0516 {
///     PackedGuid unit;
///     u32 packet_counter;
///     f32 collision_height;
/// }
/// ```
pub struct SMSG_MOVE_SET_COLLISION_HGT {
    pub unit: Guid,
    pub packet_counter: u32,
    pub collision_height: f32,
}

impl crate::Message for SMSG_MOVE_SET_COLLISION_HGT {
    const OPCODE: u32 = 0x0516;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        // packet_counter: u32
        w.write_all(&self.packet_counter.to_le_bytes())?;

        // collision_height: f32
        w.write_all(&self.collision_height.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0516, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // packet_counter: u32
        let packet_counter = crate::util::read_u32_le(r)?;

        // collision_height: f32
        let collision_height = crate::util::read_f32_le(r)?;
        Ok(Self {
            unit,
            packet_counter,
            collision_height,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_MOVE_SET_COLLISION_HGT {}

impl SMSG_MOVE_SET_COLLISION_HGT {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + 4 // packet_counter: u32
        + 4 // collision_height: f32
    }
}

