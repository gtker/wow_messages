use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_gravity_disable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_gravity_disable.wowm#L1):
/// ```text
/// smsg SMSG_SPLINE_MOVE_GRAVITY_DISABLE = 0x04D3 {
///     PackedGuid unit;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_GRAVITY_DISABLE {
    pub unit: Guid,
}

impl crate::Message for SMSG_SPLINE_MOVE_GRAVITY_DISABLE {
    const OPCODE: u32 = 0x04d3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D3, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        Ok(Self {
            unit,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_GRAVITY_DISABLE {}

impl SMSG_SPLINE_MOVE_GRAVITY_DISABLE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
    }
}

