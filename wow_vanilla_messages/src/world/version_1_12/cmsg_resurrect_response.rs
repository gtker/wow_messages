use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/cmsg_resurrect_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/cmsg_resurrect_response.wowm#L3):
/// ```text
/// cmsg CMSG_RESURRECT_RESPONSE = 0x015C {
///     Guid guid;
///     u8 status;
/// }
/// ```
pub struct CMSG_RESURRECT_RESPONSE {
    pub guid: Guid,
    pub status: u8,
}

impl ClientMessage for CMSG_RESURRECT_RESPONSE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: u8
        w.write_all(&self.status.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x015c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // status: u8
        let status = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            status,
        })
    }

}

