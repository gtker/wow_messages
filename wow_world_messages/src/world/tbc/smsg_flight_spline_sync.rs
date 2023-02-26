use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_flight_spline_sync.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_flight_spline_sync.wowm#L1):
/// ```text
/// smsg SMSG_FLIGHT_SPLINE_SYNC = 0x0388 {
///     f32 elapsed_value;
///     Guid guid;
/// }
/// ```
pub struct SMSG_FLIGHT_SPLINE_SYNC {
    pub elapsed_value: f32,
    pub guid: Guid,
}

impl crate::Message for SMSG_FLIGHT_SPLINE_SYNC {
    const OPCODE: u32 = 0x0388;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // elapsed_value: f32
        w.write_all(&self.elapsed_value.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0388, size: body_size as u32 });
        }

        // elapsed_value: f32
        let elapsed_value = crate::util::read_f32_le(r)?;
        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            elapsed_value,
            guid,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FLIGHT_SPLINE_SYNC {}

