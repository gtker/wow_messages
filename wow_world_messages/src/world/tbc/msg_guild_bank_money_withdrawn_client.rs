use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm#L1):
/// ```text
/// cmsg MSG_GUILD_BANK_MONEY_WITHDRAWN_Client = 0x03FD {
/// }
/// ```
pub struct MSG_GUILD_BANK_MONEY_WITHDRAWN_Client {
}

impl crate::Message for MSG_GUILD_BANK_MONEY_WITHDRAWN_Client {
    const OPCODE: u32 = 0x03fd;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FD, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_GUILD_BANK_MONEY_WITHDRAWN_Client {}

