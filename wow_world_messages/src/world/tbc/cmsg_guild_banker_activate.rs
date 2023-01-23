use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_guild_banker_activate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_guild_banker_activate.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_BANKER_ACTIVATE = 0x03E5 {
///     Guid bank;
///     Bool full_update;
/// }
/// ```
pub struct CMSG_GUILD_BANKER_ACTIVATE {
    pub bank: Guid,
    pub full_update: bool,
}

impl crate::Message for CMSG_GUILD_BANKER_ACTIVATE {
    const OPCODE: u32 = 0x03e5;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // full_update: Bool
        w.write_all(u8::from(self.full_update).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03E5, size: body_size as u32 });
        }

        // bank: Guid
        let bank = Guid::read(r)?;

        // full_update: Bool
        let full_update = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            bank,
            full_update,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GUILD_BANKER_ACTIVATE {}

