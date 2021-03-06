use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/cmsg_areatrigger.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/cmsg_areatrigger.wowm#L3):
/// ```text
/// cmsg CMSG_AREATRIGGER = 0x00B4 {
///     u32 trigger_id;
/// }
/// ```
pub struct CMSG_AREATRIGGER {
    pub trigger_id: u32,
}

impl ClientMessage for CMSG_AREATRIGGER {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // trigger_id: u32
        w.write_all(&self.trigger_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00b4;

    fn client_size(&self) -> u16 {
        10
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // trigger_id: u32
        let trigger_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            trigger_id,
        })
    }

}

