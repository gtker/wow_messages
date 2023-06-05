use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::sell_item_result_vanilla_tbc::SellItemResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_sell_item.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_sell_item.wowm#L22):
/// ```text
/// smsg SMSG_SELL_ITEM = 0x01A1 {
///     Guid guid;
///     Guid item;
///     SellItemResult result;
/// }
/// ```
pub struct SMSG_SELL_ITEM {
    pub guid: Guid,
    pub item: Guid,
    pub result: SellItemResult,
}

impl crate::private::Sealed for SMSG_SELL_ITEM {}
impl crate::Message for SMSG_SELL_ITEM {
    const OPCODE: u32 = 0x01a1;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // result: SellItemResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01A1, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // result: SellItemResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SELL_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SELL_ITEM {}

