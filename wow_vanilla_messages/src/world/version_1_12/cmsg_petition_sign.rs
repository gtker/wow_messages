use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_sign.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_sign.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_SIGN = 0x01C0 {
///     Guid petition_guid;
///     u8 unknown1;
/// }
/// ```
pub struct CMSG_PETITION_SIGN {
    pub petition_guid: Guid,
    pub unknown1: u8,
}

impl ClientMessage for CMSG_PETITION_SIGN {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c0;

    fn client_size(&self) -> u16 {
        15
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        Ok(Self {
            petition_guid,
            unknown1,
        })
    }

}

