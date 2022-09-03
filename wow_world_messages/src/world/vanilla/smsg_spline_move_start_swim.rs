use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_START_SWIM = 0x030B {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_START_SWIM {
    pub guid: Guid,
}

impl crate::Message for SMSG_SPLINE_MOVE_START_SWIM {
    const OPCODE: u32 = 0x030b;

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
impl ServerMessage for SMSG_SPLINE_MOVE_START_SWIM {}

impl SMSG_SPLINE_MOVE_START_SWIM {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

