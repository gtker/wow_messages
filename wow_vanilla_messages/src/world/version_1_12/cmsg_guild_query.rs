use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_guild_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_guild_query.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_QUERY = 0x0054 {
///     u32 guild_id;
/// }
/// ```
pub struct CMSG_GUILD_QUERY {
    pub guild_id: u32,
}

impl ClientMessage for CMSG_GUILD_QUERY {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0054;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guild_id,
        })
    }

}

