use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{GroupLootSetting, GroupLootSettingError};
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::{ClientMessageWrite, MessageBody};
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

impl ClientMessageWrite for CMSG_LOOT_METHOD {}

impl CMSG_LOOT_METHOD {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // loot_setting: GroupLootSetting
        w.write_all(&(self.loot_setting.as_int() as u32).to_le_bytes())?;

        // loot_master: Guid
        w.write_all(&self.loot_master.guid().to_le_bytes())?;

        // loot_threshold: ItemQuality
        w.write_all(&(self.loot_threshold.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for CMSG_LOOT_METHOD {
    const OPCODE: u16 = 0x007a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_LOOT_METHODError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // loot_setting: GroupLootSetting
            let loot_setting: GroupLootSetting = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // loot_master: Guid
            let loot_master = Guid::tokio_read(r).await?;

            // loot_threshold: ItemQuality
            let loot_threshold: ItemQuality = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                loot_setting,
                loot_master,
                loot_threshold,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // loot_setting: GroupLootSetting
            let loot_setting: GroupLootSetting = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // loot_master: Guid
            let loot_master = Guid::astd_read(r).await?;

            // loot_threshold: ItemQuality
            let loot_threshold: ItemQuality = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                loot_setting,
                loot_master,
                loot_threshold,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl CMSG_LOOT_METHOD {
    pub(crate) fn size() -> usize {
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

