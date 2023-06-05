use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    GroupLootSetting, ItemQuality,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_loot_method.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_loot_method.wowm#L1):
/// ```text
/// cmsg CMSG_LOOT_METHOD = 0x007A {
///     (u32)GroupLootSetting loot_setting;
///     Guid loot_master;
///     (u32)ItemQuality loot_threshold;
/// }
/// ```
pub struct CMSG_LOOT_METHOD {
    pub loot_setting: GroupLootSetting,
    pub loot_master: Guid,
    pub loot_threshold: ItemQuality,
}

impl crate::private::Sealed for CMSG_LOOT_METHOD {}
impl crate::Message for CMSG_LOOT_METHOD {
    const OPCODE: u32 = 0x007a;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        w.write_all(&u32::from(self.loot_setting.as_int()).to_le_bytes())?;

        // loot_master: Guid
        w.write_all(&self.loot_master.guid().to_le_bytes())?;

        // loot_threshold: ItemQuality
        w.write_all(&u32::from(self.loot_threshold.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x007A, size: body_size });
        }

        // loot_setting: GroupLootSetting
        let loot_setting = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // loot_master: Guid
        let loot_master = crate::util::read_guid(&mut r)?;

        // loot_threshold: ItemQuality
        let loot_threshold = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            loot_setting,
            loot_master,
            loot_threshold,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOOT_METHOD {}

