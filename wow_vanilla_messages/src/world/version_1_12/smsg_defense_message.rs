use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Map;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_defense_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_defense_message.wowm#L3):
/// ```text
/// smsg SMSG_DEFENSE_MESSAGE = 0x033B {
///     Map map;
///     u32 message_length;
///     CString message;
/// }
/// ```
pub struct SMSG_DEFENSE_MESSAGE {
    pub map: Map,
    pub message_length: u32,
    pub message: String,
}

impl ServerMessage for SMSG_DEFENSE_MESSAGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // message_length: u32
        w.write_all(&self.message_length.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x033b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // message_length: u32
        let message_length = crate::util::read_u32_le(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            map,
            message_length,
            message,
        })
    }

}

impl SMSG_DEFENSE_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 4 // message_length: u32
        + self.message.len() + 1 // message: CString
    }
}

