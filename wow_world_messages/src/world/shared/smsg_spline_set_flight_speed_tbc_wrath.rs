use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_flight_speed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_flight_speed.wowm#L1):
/// ```text
/// smsg SMSG_SPLINE_SET_FLIGHT_SPEED = 0x0385 {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
pub struct SMSG_SPLINE_SET_FLIGHT_SPEED {
    pub guid: Guid,
    pub speed: f32,
}

impl crate::Message for SMSG_SPLINE_SET_FLIGHT_SPEED {
    const OPCODE: u32 = 0x0385;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0385, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            speed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_SET_FLIGHT_SPEED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_SET_FLIGHT_SPEED {}

impl SMSG_SPLINE_SET_FLIGHT_SPEED {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // speed: f32
    }
}

