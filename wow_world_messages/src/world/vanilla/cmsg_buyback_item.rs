use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::BuybackSlot;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm#L19):
/// ```text
/// cmsg CMSG_BUYBACK_ITEM = 0x0290 {
///     Guid guid;
///     BuybackSlot slot;
/// }
/// ```
pub struct CMSG_BUYBACK_ITEM {
    pub guid: Guid,
    pub slot: BuybackSlot,
}

impl crate::Message for CMSG_BUYBACK_ITEM {
    const OPCODE: u32 = 0x0290;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // slot: BuybackSlot
        w.write_all(&(self.slot.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // slot: BuybackSlot
        let slot: BuybackSlot = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_BUYBACK_ITEM {}

