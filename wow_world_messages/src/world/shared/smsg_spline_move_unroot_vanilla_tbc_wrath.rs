use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_unroot.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_unroot.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_UNROOT = 0x0304 {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_UNROOT {
    pub guid: Guid,
}

impl crate::Message for SMSG_SPLINE_MOVE_UNROOT {
    const OPCODE: u32 = 0x0304;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0304, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_MOVE_UNROOT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_MOVE_UNROOT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_UNROOT {}

impl SMSG_SPLINE_MOVE_UNROOT {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

