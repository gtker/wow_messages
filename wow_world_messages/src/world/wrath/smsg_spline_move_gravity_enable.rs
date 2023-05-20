use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_gravity_enable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_gravity_enable.wowm#L1):
/// ```text
/// smsg SMSG_SPLINE_MOVE_GRAVITY_ENABLE = 0x04D4 {
///     PackedGuid unit;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_GRAVITY_ENABLE {
    pub unit: Guid,
}

impl crate::private::Sealed for SMSG_SPLINE_MOVE_GRAVITY_ENABLE {}
impl crate::Message for SMSG_SPLINE_MOVE_GRAVITY_ENABLE {
    const OPCODE: u32 = 0x04d4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D4, size: body_size });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        Ok(Self {
            unit,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_GRAVITY_ENABLE {}

impl SMSG_SPLINE_MOVE_GRAVITY_ENABLE {
    pub(crate) const fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
    }
}

