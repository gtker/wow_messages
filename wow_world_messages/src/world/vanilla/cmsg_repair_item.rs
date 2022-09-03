use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_repair_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_repair_item.wowm#L3):
/// ```text
/// cmsg CMSG_REPAIR_ITEM = 0x02A8 {
///     Guid npc_guid;
///     Guid item_guid;
/// }
/// ```
pub struct CMSG_REPAIR_ITEM {
    pub npc_guid: Guid,
    pub item_guid: Guid,
}

impl crate::Message for CMSG_REPAIR_ITEM {
    const OPCODE: u32 = 0x02a8;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc_guid: Guid
        w.write_all(&self.npc_guid.guid().to_le_bytes())?;

        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        Ok(Self {
            npc_guid,
            item_guid,
        })
    }

}
impl ClientMessage for CMSG_REPAIR_ITEM {}

