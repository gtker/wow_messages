use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_open_container.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_open_container.wowm#L3):
/// ```text
/// smsg SMSG_OPEN_CONTAINER = 0x0113 {
///     Guid guid;
/// }
/// ```
pub struct SMSG_OPEN_CONTAINER {
    pub guid: Guid,
}

impl ServerMessage for SMSG_OPEN_CONTAINER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0113;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}

