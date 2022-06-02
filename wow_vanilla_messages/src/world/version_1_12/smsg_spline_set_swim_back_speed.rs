use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_swim_back_speed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_swim_back_speed.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_SET_SWIM_BACK_SPEED = 0x0302 {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
pub struct SMSG_SPLINE_SET_SWIM_BACK_SPEED {
    pub guid: Guid,
    pub speed: f32,
}

impl ServerMessage for SMSG_SPLINE_SET_SWIM_BACK_SPEED {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0302;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            speed,
        })
    }

}

impl SMSG_SPLINE_SET_SWIM_BACK_SPEED {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // speed: f32
    }
}

