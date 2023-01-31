use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_guild_bank_text.wowm#L7):
/// ```text
/// cmsg MSG_QUERY_GUILD_BANK_TEXT_Client = 0x040A {
///     u8 tab;
/// }
/// ```
pub struct MSG_QUERY_GUILD_BANK_TEXT_Client {
    pub tab: u8,
}

impl crate::Message for MSG_QUERY_GUILD_BANK_TEXT_Client {
    const OPCODE: u32 = 0x040a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x040A, size: body_size as u32 });
        }

        // tab: u8
        let tab = crate::util::read_u8_le(r)?;

        Ok(Self {
            tab,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_QUERY_GUILD_BANK_TEXT_Client {}

