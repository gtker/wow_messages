use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_petition_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_petition_query.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_QUERY = 0x01C6 {
///     u32 guild_guid;
///     Guid petition_guid;
/// }
/// ```
pub struct CMSG_PETITION_QUERY {
    pub guild_guid: u32,
    pub petition_guid: Guid,
}

impl ClientMessage for CMSG_PETITION_QUERY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_guid: u32
        w.write_all(&self.guild_guid.to_le_bytes())?;

        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c6;

    fn client_size(&self) -> u16 {
        18
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guild_guid: u32
        let guild_guid = crate::util::read_u32_le(r)?;

        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        Ok(Self {
            guild_guid,
            petition_guid,
        })
    }

}

