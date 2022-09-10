use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_turn_rate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_turn_rate.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_SET_TURN_RATE = 0x0303 {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
pub struct SMSG_SPLINE_SET_TURN_RATE {
    pub guid: Guid,
    pub speed: f32,
}

impl crate::Message for SMSG_SPLINE_SET_TURN_RATE {
    const OPCODE: u32 = 0x0303;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            speed,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_SPLINE_SET_TURN_RATE {}

impl SMSG_SPLINE_SET_TURN_RATE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // speed: f32
    }
}

