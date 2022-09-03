use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_cancel_channelling.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_cancel_channelling.wowm#L3):
/// ```text
/// cmsg CMSG_CANCEL_CHANNELLING = 0x013B {
///     u32 id;
/// }
/// ```
pub struct CMSG_CANCEL_CHANNELLING {
    pub id: u32,
}

impl crate::Message for CMSG_CANCEL_CHANNELLING {
    const OPCODE: u32 = 0x013b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
        })
    }

}
impl ClientMessage for CMSG_CANCEL_CHANNELLING {}

