use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L3):
/// ```text
/// enum GroupLootSetting : u8 {
///     FREE_FOR_ALL = 0;
///     ROUND_ROBIN = 1;
///     MASTER_LOOT = 2;
///     GROUP_LOOT = 3;
///     NEED_BEFORE_GREED = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GroupLootSetting {
    FREE_FOR_ALL,
    ROUND_ROBIN,
    MASTER_LOOT,
    GROUP_LOOT,
    NEED_BEFORE_GREED,
}

impl ReadableAndWritable for GroupLootSetting {
    type Error = GroupLootSettingError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl GroupLootSetting {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupLootSettingError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupLootSettingError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupLootSettingError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupLootSettingError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupLootSettingError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GroupLootSettingError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::FREE_FOR_ALL => 0x0,
            Self::ROUND_ROBIN => 0x1,
            Self::MASTER_LOOT => 0x2,
            Self::GROUP_LOOT => 0x3,
            Self::NEED_BEFORE_GREED => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::FREE_FOR_ALL
    }

}

impl ConstantSized for GroupLootSetting {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GroupLootSetting {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GroupLootSetting {
    fn default() -> Self {
        Self::FREE_FOR_ALL
    }
}

impl std::fmt::Display for GroupLootSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FREE_FOR_ALL => f.write_str("FREE_FOR_ALL"),
            Self::ROUND_ROBIN => f.write_str("ROUND_ROBIN"),
            Self::MASTER_LOOT => f.write_str("MASTER_LOOT"),
            Self::GROUP_LOOT => f.write_str("GROUP_LOOT"),
            Self::NEED_BEFORE_GREED => f.write_str("NEED_BEFORE_GREED"),
        }
    }
}

impl TryFrom<u8> for GroupLootSetting {
    type Error = TryFromGroupLootSettingError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FREE_FOR_ALL),
            1 => Ok(Self::ROUND_ROBIN),
            2 => Ok(Self::MASTER_LOOT),
            3 => Ok(Self::GROUP_LOOT),
            4 => Ok(Self::NEED_BEFORE_GREED),
            _ => Err(TryFromGroupLootSettingError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGroupLootSettingError {
    value: u8,
}

impl TryFromGroupLootSettingError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GroupLootSettingError {
    Read(std::io::Error),
    TryFrom(TryFromGroupLootSettingError),
}

impl std::error::Error for GroupLootSettingError {}
impl std::fmt::Display for TryFromGroupLootSettingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GroupLootSetting': '{}'", self.value))
    }
}

impl std::fmt::Display for GroupLootSettingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GroupLootSettingError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGroupLootSettingError> for GroupLootSettingError {
    fn from(value: TryFromGroupLootSettingError) -> Self {
        Self::TryFrom(value)
    }
}

