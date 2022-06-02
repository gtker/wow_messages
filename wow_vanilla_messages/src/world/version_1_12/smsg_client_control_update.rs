use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_client_control_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_client_control_update.wowm#L3):
/// ```text
/// smsg SMSG_CLIENT_CONTROL_UPDATE = 0x0159 {
///     PackedGuid guid;
///     u8 allow_movement;
/// }
/// ```
pub struct SMSG_CLIENT_CONTROL_UPDATE {
    pub guid: Guid,
    pub allow_movement: u8,
}

impl ServerMessage for SMSG_CLIENT_CONTROL_UPDATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // allow_movement: u8
        w.write_all(&self.allow_movement.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0159;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // allow_movement: u8
        let allow_movement = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            allow_movement,
        })
    }

}

impl SMSG_CLIENT_CONTROL_UPDATE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 1 // allow_movement: u8
    }
}

