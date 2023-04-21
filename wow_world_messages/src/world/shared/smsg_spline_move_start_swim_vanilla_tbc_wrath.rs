use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_START_SWIM = 0x030B {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_START_SWIM {
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_SPLINE_MOVE_START_SWIM {}
impl crate::Message for SMSG_SPLINE_MOVE_START_SWIM {
    const OPCODE: u32 = 0x030b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x030B, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

impl SMSG_SPLINE_MOVE_START_SWIM {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

