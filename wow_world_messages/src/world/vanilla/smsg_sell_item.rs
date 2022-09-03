use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SellItemResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl crate::Message for SMSG_SELL_ITEM {
    const OPCODE: u32 = 0x01a1;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // result: SellItemResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // result: SellItemResult
        let result: SellItemResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            item,
            result,
        })
    }

}
impl ServerMessage for SMSG_SELL_ITEM {}

