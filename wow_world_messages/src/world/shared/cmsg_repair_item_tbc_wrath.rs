use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_repair_item.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_repair_item.wowm#L8):
/// ```text
/// cmsg CMSG_REPAIR_ITEM = 0x02A8 {
///     Guid npc;
///     Guid item;
///     Bool from_guild_bank;
/// }
/// ```
pub struct CMSG_REPAIR_ITEM {
    pub npc: Guid,
    pub item: Guid,
    pub from_guild_bank: bool,
}

impl crate::Message for CMSG_REPAIR_ITEM {
    const OPCODE: u32 = 0x02a8;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // from_guild_bank: Bool
        w.write_all(u8::from(self.from_guild_bank).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A8, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(&mut r)?;

        // item: Guid
        let item = Guid::read(&mut r)?;

        // from_guild_bank: Bool
        let from_guild_bank = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            npc,
            item,
            from_guild_bank,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_REPAIR_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REPAIR_ITEM {}

