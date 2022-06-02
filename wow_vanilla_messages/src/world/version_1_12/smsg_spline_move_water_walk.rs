use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_water_walk.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_water_walk.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_WATER_WALK = 0x0309 {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_WATER_WALK {
    pub guid: Guid,
}

impl ServerMessage for SMSG_SPLINE_MOVE_WATER_WALK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        Ok(())
    }
    const OPCODE: u16 = 0x0309;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}

impl SMSG_SPLINE_MOVE_WATER_WALK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

