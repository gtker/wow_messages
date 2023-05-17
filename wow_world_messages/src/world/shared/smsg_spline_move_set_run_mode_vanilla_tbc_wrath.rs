use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_SET_RUN_MODE = 0x030D {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_SPLINE_MOVE_SET_RUN_MODE {}
impl crate::Message for SMSG_SPLINE_MOVE_SET_RUN_MODE {
    const OPCODE: u32 = 0x030d;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x030D, size: body_size });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

impl SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

