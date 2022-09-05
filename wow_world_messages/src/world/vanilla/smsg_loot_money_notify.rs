use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_MONEY_NOTIFY = 0x0163 {
///     u32 amount;
/// }
/// ```
pub struct SMSG_LOOT_MONEY_NOTIFY {
    pub amount: u32,
}

impl crate::Message for SMSG_LOOT_MONEY_NOTIFY {
    const OPCODE: u32 = 0x0163;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // amount: u32
        let amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            amount,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LOOT_MONEY_NOTIFY {}

