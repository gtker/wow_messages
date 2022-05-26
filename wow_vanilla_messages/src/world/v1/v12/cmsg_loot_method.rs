use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::GroupLootSetting;
use crate::world::v1::v12::ItemQuality;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LOOT_METHOD {
    pub loot_setting: GroupLootSetting,
    pub loot_master: Guid,
    pub loot_threshold: ItemQuality,
}

impl CMSG_LOOT_METHOD {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        w.write_all(&(self.loot_setting.as_int() as u32).to_le_bytes())?;

        // loot_master: Guid
        w.write_all(&self.loot_master.guid().to_le_bytes())?;

        // loot_threshold: ItemQuality
        w.write_all(&(self.loot_threshold.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMSG_LOOT_METHOD {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        w.write_all(&(self.loot_setting.as_int() as u32).to_le_bytes())?;

        // loot_master: Guid
        w.write_all(&self.loot_master.guid().to_le_bytes())?;

        // loot_threshold: ItemQuality
        w.write_all(&(self.loot_threshold.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x007a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        10
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

