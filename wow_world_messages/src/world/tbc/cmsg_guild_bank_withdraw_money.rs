use crate::Guid;
use crate::tbc::Gold;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_withdraw_money.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/cmsg_guild_bank_withdraw_money.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_BANK_WITHDRAW_MONEY = 0x03EC {
///     Guid bank;
///     Gold money;
/// }
/// ```
pub struct CMSG_GUILD_BANK_WITHDRAW_MONEY {
    pub bank: Guid,
    pub money: Gold,
}

impl crate::Message for CMSG_GUILD_BANK_WITHDRAW_MONEY {
    const OPCODE: u32 = 0x03ec;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // bank: Guid
        w.write_all(&self.bank.guid().to_le_bytes())?;

        // money: Gold
        w.write_all(u32::from(self.money.as_int()).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EC, size: body_size as u32 });
        }

        // bank: Guid
        let bank = Guid::read(r)?;

        // money: Gold
        let money = Gold::new(crate::util::read_u32_le(r)?);
        Ok(Self {
            bank,
            money,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_BANK_WITHDRAW_MONEY {}

