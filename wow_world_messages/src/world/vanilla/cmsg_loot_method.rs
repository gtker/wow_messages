use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::GroupLootSetting;
use crate::world::vanilla::ItemQuality;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_loot_method.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_loot_method.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_METHOD = 0x007A {
///     GroupLootSetting loot_setting;
///     Guid loot_master;
///     ItemQuality loot_threshold;
/// }
/// ```
pub struct CMSG_LOOT_METHOD {
    pub loot_setting: GroupLootSetting,
    pub loot_master: Guid,
    pub loot_threshold: ItemQuality,
}

impl crate::Message for CMSG_LOOT_METHOD {
    const OPCODE: u32 = 0x007a;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        w.write_all(&(self.loot_setting.as_int() as u32).to_le_bytes())?;

        // loot_master: Guid
        w.write_all(&self.loot_master.guid().to_le_bytes())?;

        // loot_threshold: ItemQuality
        w.write_all(&(self.loot_threshold.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // loot_setting: GroupLootSetting
        let loot_setting: GroupLootSetting = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // loot_master: Guid
        let loot_master = Guid::read(r)?;

        // loot_threshold: ItemQuality
        let loot_threshold: ItemQuality = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            loot_setting,
            loot_master,
            loot_threshold,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_LOOT_METHOD {}

