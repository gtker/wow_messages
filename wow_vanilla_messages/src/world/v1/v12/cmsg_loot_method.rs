use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{GroupLootSetting, GroupLootSettingError};
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
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

impl ClientMessageWrite for CMSG_LOOT_METHOD {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_LOOT_METHOD {
    const OPCODE: u16 = 0x007a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_LOOT_METHODError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        self.loot_setting.write_u32_le(w)?;

        // loot_master: Guid
        self.loot_master.write(w)?;

        // loot_threshold: ItemQuality
        self.loot_threshold.write_u32_le(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_setting: GroupLootSetting
        let loot_setting = GroupLootSetting::tokio_read_u32_le(r).await?;

        // loot_master: Guid
        let loot_master = Guid::tokio_read(r).await?;

        // loot_threshold: ItemQuality
        let loot_threshold = ItemQuality::tokio_read_u32_le(r).await?;

        Ok(Self {
            loot_setting,
            loot_master,
            loot_threshold,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        self.loot_setting.tokio_write_u32_le(w).await?;

        // loot_master: Guid
        self.loot_master.tokio_write(w).await?;

        // loot_threshold: ItemQuality
        self.loot_threshold.tokio_write_u32_le(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_setting: GroupLootSetting
        let loot_setting = GroupLootSetting::astd_read_u32_le(r).await?;

        // loot_master: Guid
        let loot_master = Guid::astd_read(r).await?;

        // loot_threshold: ItemQuality
        let loot_threshold = ItemQuality::astd_read_u32_le(r).await?;

        Ok(Self {
            loot_setting,
            loot_master,
            loot_threshold,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        self.loot_setting.astd_write_u32_le(w).await?;

        // loot_master: Guid
        self.loot_master.astd_write(w).await?;

        // loot_threshold: ItemQuality
        self.loot_threshold.astd_write_u32_le(w).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_LOOT_METHOD {}

impl MaximumPossibleSized for CMSG_LOOT_METHOD {
    fn maximum_possible_size() -> usize {
        0
        + 1 // loot_setting: GroupLootSetting
        + 8 // loot_master: Guid
        + 1 // loot_threshold: ItemQuality
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

