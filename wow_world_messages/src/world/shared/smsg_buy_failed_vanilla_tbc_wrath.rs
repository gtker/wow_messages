use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::buy_result_vanilla_tbc_wrath::BuyResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Some TBC and Wrath emus have a u32 before `result` that is only included if the value is > 0, but the emus never call it with anything other than 0.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_failed.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_failed.wowm#L15):
/// ```text
/// smsg SMSG_BUY_FAILED = 0x01A5 {
///     Guid guid;
///     u32 item;
///     BuyResult result;
/// }
/// ```
pub struct SMSG_BUY_FAILED {
    pub guid: Guid,
    pub item: u32,
    pub result: BuyResult,
}

impl crate::private::Sealed for SMSG_BUY_FAILED {}
impl crate::Message for SMSG_BUY_FAILED {
    const OPCODE: u32 = 0x01a5;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // result: BuyResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A5, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // result: BuyResult
        let result: BuyResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BUY_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BUY_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BUY_FAILED {}

