use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_master_give.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_master_give.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_MASTER_GIVE = 0x02A3 {
///     Guid loot;
///     u8 slot_id;
///     Guid player;
/// }
/// ```
pub struct CMSG_LOOT_MASTER_GIVE {
    pub loot: Guid,
    pub slot_id: u8,
    pub player: Guid,
}

impl crate::Message for CMSG_LOOT_MASTER_GIVE {
    const OPCODE: u32 = 0x02a3;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // loot: Guid
        w.write_all(&self.loot.guid().to_le_bytes())?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A3, size: body_size as u32 });
        }

        // loot: Guid
        let loot = Guid::read(r)?;

        // slot_id: u8
        let slot_id = crate::util::read_u8_le(r)?;

        // player: Guid
        let player = Guid::read(r)?;

        Ok(Self {
            loot,
            slot_id,
            player,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_LOOT_MASTER_GIVE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_LOOT_MASTER_GIVE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_LOOT_MASTER_GIVE {}

