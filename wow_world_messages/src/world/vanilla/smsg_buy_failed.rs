use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::BuyResult;
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_buy_failed.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_buy_failed.wowm#L15):
/// ```text
/// smsg SMSG_BUY_FAILED = 0x01A5 {
///     Guid guid;
///     u32 item_id;
///     BuyResult result;
/// }
/// ```
pub struct SMSG_BUY_FAILED {
    pub guid: Guid,
    pub item_id: u32,
    pub result: BuyResult,
}

impl crate::Message for SMSG_BUY_FAILED {
    const OPCODE: u32 = 0x01a5;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // result: BuyResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // result: BuyResult
        let result: BuyResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            item_id,
            result,
        })
    }

}
impl ServerMessage for SMSG_BUY_FAILED {}

