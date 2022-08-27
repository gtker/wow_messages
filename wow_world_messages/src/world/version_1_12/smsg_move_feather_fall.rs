use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_feather_fall.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_feather_fall.wowm#L3):
/// ```text
/// smsg SMSG_MOVE_FEATHER_FALL = 0x00F2 {
///     PackedGuid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_MOVE_FEATHER_FALL {
    pub guid: Guid,
    pub counter: u32,
}

impl ServerMessage for SMSG_MOVE_FEATHER_FALL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00f2;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}

impl SMSG_MOVE_FEATHER_FALL {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // counter: u32
    }
}

