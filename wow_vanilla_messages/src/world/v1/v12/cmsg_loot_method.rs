use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{GroupLootSetting, GroupLootSettingError};
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LOOT_METHOD {
    pub loot_setting: GroupLootSetting,
    pub loot_master: Guid,
    pub loot_threshold: ItemQuality,
}

impl ClientMessageWrite for CMSG_LOOT_METHOD {
    const OPCODE: u32 = 0x7a;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for CMSG_LOOT_METHOD {
    type Error = CMSG_LOOT_METHODError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_setting: GroupLootSetting
        let loot_setting = GroupLootSetting::read_u32_le(r)?;

        // loot_master: Guid
        let loot_master = Guid::read(r)?;

        // loot_threshold: ItemQuality
        let loot_threshold = ItemQuality::read_u32_le(r)?;

        Ok(Self {
            loot_setting,
            loot_master,
            loot_threshold,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        self.loot_setting.write_u32_le(w)?;

        // loot_master: Guid
        self.loot_master.write(w)?;

        // loot_threshold: ItemQuality
        self.loot_threshold.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_LOOT_METHOD {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_LOOT_METHOD {
    fn maximum_possible_size() -> usize {
        4 // loot_setting: GroupLootSetting upcasted to u32
        + 8 // loot_master: Guid
        + 4 // loot_threshold: ItemQuality upcasted to u32
    }
}

#[derive(Debug)]
pub enum CMSG_LOOT_METHODError {
    Io(std::io::Error),
    GroupLootSetting(GroupLootSettingError),
    ItemQuality(ItemQualityError),
}

impl std::error::Error for CMSG_LOOT_METHODError {}
impl std::fmt::Display for CMSG_LOOT_METHODError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::GroupLootSetting(i) => i.fmt(f),
            Self::ItemQuality(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_LOOT_METHODError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<GroupLootSettingError> for CMSG_LOOT_METHODError {
    fn from(e: GroupLootSettingError) -> Self {
        Self::GroupLootSetting(e)
    }
}

impl From<ItemQualityError> for CMSG_LOOT_METHODError {
    fn from(e: ItemQualityError) -> Self {
        Self::ItemQuality(e)
    }
}

