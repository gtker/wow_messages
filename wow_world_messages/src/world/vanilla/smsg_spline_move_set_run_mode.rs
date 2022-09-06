use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_SET_RUN_MODE = 0x030D {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub guid: Guid,
}

impl crate::Message for SMSG_SPLINE_MOVE_SET_RUN_MODE {
    const OPCODE: u32 = 0x030d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

impl SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

