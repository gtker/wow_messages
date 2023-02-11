use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_query_tab.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_query_tab.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_BANK_QUERY_TAB = 0x03E6 {
///     Guid bank;
///     u8 tab;
///     Bool full_update;
/// }
/// ```
pub struct CMSG_GUILD_BANK_QUERY_TAB {
    pub bank: Guid,
    pub tab: u8,
    pub full_update: bool,
}

impl crate::Message for CMSG_GUILD_BANK_QUERY_TAB {
    const OPCODE: u32 = 0x03e6;

    fn size_without_header(&self) -> u32 {
        10
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // tab: u8
        w.write_all(&self.tab.to_le_bytes())?;

        // full_update: Bool
        w.write_all(u8::from(self.full_update).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 10 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E6, size: body_size as u32 });
        }

        // bank: Guid
        let bank = Guid::read(r)?;

        // tab: u8
        let tab = crate::util::read_u8_le(r)?;

        // full_update: Bool
        let full_update = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            bank,
            tab,
            full_update,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_BANK_QUERY_TAB {}

