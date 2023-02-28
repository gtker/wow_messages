use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_repair_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_repair_item.wowm#L1):
/// ```text
/// cmsg CMSG_REPAIR_ITEM = 0x02A8 {
///     Guid npc;
///     Guid item;
/// }
/// ```
pub struct CMSG_REPAIR_ITEM {
    pub npc: Guid,
    pub item: Guid,
}

impl crate::Message for CMSG_REPAIR_ITEM {
    const OPCODE: u32 = 0x02a8;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A8, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        Ok(Self {
            npc,
            item,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_REPAIR_ITEM {}

