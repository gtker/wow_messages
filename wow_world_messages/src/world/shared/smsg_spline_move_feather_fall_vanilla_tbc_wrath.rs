use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_feather_fall.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_feather_fall.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_FEATHER_FALL = 0x0305 {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_FEATHER_FALL {
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_SPLINE_MOVE_FEATHER_FALL {}
impl crate::Message for SMSG_SPLINE_MOVE_FEATHER_FALL {
    const OPCODE: u32 = 0x0305;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0305, size: body_size });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_MOVE_FEATHER_FALL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_MOVE_FEATHER_FALL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_FEATHER_FALL {}

impl SMSG_SPLINE_MOVE_FEATHER_FALL {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

