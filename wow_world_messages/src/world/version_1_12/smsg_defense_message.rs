use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Map;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_defense_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_defense_message.wowm#L3):
/// ```text
/// smsg SMSG_DEFENSE_MESSAGE = 0x033B {
///     Map map;
///     SizedCString message;
/// }
/// ```
pub struct SMSG_DEFENSE_MESSAGE {
    pub map: Map,
    pub message: String,
}

impl ServerMessage for SMSG_DEFENSE_MESSAGE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x033b;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // message: SizedCString
        let message = crate::util::read_u32_le(r)?;
        let message = crate::util::read_sized_c_string_to_vec(r, message)?;
        let message = String::from_utf8(message)?;;
        Ok(Self {
            map,
            message,
        })
    }

}

impl SMSG_DEFENSE_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + self.message.len() + 5 // message: SizedCString
    }
}

