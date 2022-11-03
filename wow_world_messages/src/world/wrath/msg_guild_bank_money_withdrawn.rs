use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm#L3):
/// ```text
/// msg MSG_GUILD_BANK_MONEY_WITHDRAWN = 0x03FE {
/// }
/// ```
pub struct MSG_GUILD_BANK_MONEY_WITHDRAWN {
}

impl crate::Message for MSG_GUILD_BANK_MONEY_WITHDRAWN {
    const OPCODE: u32 = 0x03fe;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FE, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_GUILD_BANK_MONEY_WITHDRAWN {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_GUILD_BANK_MONEY_WITHDRAWN {}

