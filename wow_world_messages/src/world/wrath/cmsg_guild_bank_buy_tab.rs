use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_buy_tab.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_buy_tab.wowm#L8):
/// ```text
/// cmsg CMSG_GUILD_BANK_BUY_TAB = 0x03EA {
///     Guid banker;
///     u8 tab;
/// }
/// ```
pub struct CMSG_GUILD_BANK_BUY_TAB {
    pub banker: Guid,
    pub tab: u8,
}

impl crate::Message for CMSG_GUILD_BANK_BUY_TAB {
    const OPCODE: u32 = 0x03ea;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // banker: Guid
        w.write_all(&self.banker.guid().to_le_bytes())?;

        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EA, size: body_size as u32 });
        }

        // banker: Guid
        let banker = Guid::read(&mut r)?;

        // tab: u8
        let tab = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            banker,
            tab,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_BANK_BUY_TAB {}

