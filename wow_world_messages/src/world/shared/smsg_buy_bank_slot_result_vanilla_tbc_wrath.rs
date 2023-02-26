use wow_world_base::shared::buy_bank_slot_result_vanilla_tbc_wrath::BuyBankSlotResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_bank_slot_result.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_bank_slot_result.wowm#L10):
/// ```text
/// smsg SMSG_BUY_BANK_SLOT_RESULT = 0x01BA {
///     BuyBankSlotResult result;
/// }
/// ```
pub struct SMSG_BUY_BANK_SLOT_RESULT {
    pub result: BuyBankSlotResult,
}

impl crate::Message for SMSG_BUY_BANK_SLOT_RESULT {
    const OPCODE: u32 = 0x01ba;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // result: BuyBankSlotResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BA, size: body_size as u32 });
        }

        // result: BuyBankSlotResult
        let result: BuyBankSlotResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BUY_BANK_SLOT_RESULT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BUY_BANK_SLOT_RESULT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BUY_BANK_SLOT_RESULT {}

