use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GroupLootSetting, GroupLootSettingError};
use crate::world::v1::v12::{ItemQuality, ItemQualityError};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_loot_method.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_loot_method.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_METHOD = 0x7A {
///     GroupLootSetting loot_setting;
///     u64 loot_master;
///     ItemQuality loot_threshold;
/// }
/// ```
pub struct CMSG_LOOT_METHOD {
    pub loot_setting: GroupLootSetting,
    pub loot_master: u64,
    pub loot_threshold: ItemQuality,
}

impl WorldClientMessageWrite for CMSG_LOOT_METHOD {
    const OPCODE: u32 = 0x7a;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_LOOT_METHOD {
    type Error = CMSG_LOOT_METHODError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_setting: GroupLootSetting
        let loot_setting = GroupLootSetting::read_u32_le(r)?;

        // loot_master: u64
        let loot_master = crate::util::read_u64_le(r)?;

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

        // loot_master: u64
        w.write_all(&self.loot_master.to_le_bytes())?;

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
        + 8 // loot_master: u64
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

