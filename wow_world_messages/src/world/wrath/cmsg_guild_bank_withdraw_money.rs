use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_withdraw_money.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_withdraw_money.wowm#L8):
/// ```text
/// cmsg CMSG_GUILD_BANK_WITHDRAW_MONEY = 0x03ED {
///     Guid bank;
///     u32 money;
/// }
/// ```
pub struct CMSG_GUILD_BANK_WITHDRAW_MONEY {
    pub bank: Guid,
    pub money: u32,
}

impl crate::Message for CMSG_GUILD_BANK_WITHDRAW_MONEY {
    const OPCODE: u32 = 0x03ed;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // money: u32
        w.write_all(&self.money.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03ED, size: body_size as u32 });
        }

        // bank: Guid
        let bank = Guid::read(r)?;

        // money: u32
        let money = crate::util::read_u32_le(r)?;

        Ok(Self {
            bank,
            money,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GUILD_BANK_WITHDRAW_MONEY {}

