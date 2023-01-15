use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_bank_money_withdrawn.wowm#L15):
/// ```text
/// smsg MSG_GUILD_BANK_MONEY_WITHDRAWN_Server = 0x03FE {
///     u32 remaining_withdraw_amount;
/// }
/// ```
pub struct MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    pub remaining_withdraw_amount: u32,
}

impl crate::Message for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {
    const OPCODE: u32 = 0x03fe;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // remaining_withdraw_amount: u32
        w.write_all(&self.remaining_withdraw_amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FE, size: body_size as u32 });
        }

        // remaining_withdraw_amount: u32
        let remaining_withdraw_amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            remaining_withdraw_amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_GUILD_BANK_MONEY_WITHDRAWN_Server {}

