use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm):
/// ```text
/// enum InventoryType : u8 {
///     NON_EQUIP = 0;
///     HEAD = 1;
///     NECK_OR_RELIC = 2;
///     SHOULDERS = 3;
///     BODY = 4;
///     CHEST = 5;
///     WAIST = 6;
///     LEGS = 7;
///     FEET = 8;
///     WRISTS = 9;
///     HANDS = 10;
///     FINGER = 11;
///     TRINKET = 12;
///     WEAPON = 13;
///     SHIELD = 14;
///     RANGED = 15;
///     CLOAK = 16;
///     TWO_HANDED_WEAPON = 17;
///     BAG = 18;
///     TABARD = 19;
///     ROBE = 20;
///     WEAPON_MAIN_HAND = 21;
///     WEAPON_OFF_HAND = 22;
///     HOLDABLE = 23;
///     AMMO = 24;
///     THROWN = 25;
///     RANGED_RIGHT = 26;
///     QUIVER = 27;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InventoryType {
    NON_EQUIP,
    HEAD,
    NECK_OR_RELIC,
    SHOULDERS,
    BODY,
    CHEST,
    WAIST,
    LEGS,
    FEET,
    WRISTS,
    HANDS,
    FINGER,
    TRINKET,
    WEAPON,
    SHIELD,
    RANGED,
    CLOAK,
    TWO_HANDED_WEAPON,
    BAG,
    TABARD,
    ROBE,
    WEAPON_MAIN_HAND,
    WEAPON_OFF_HAND,
    HOLDABLE,
    AMMO,
    THROWN,
    RANGED_RIGHT,
    QUIVER,
}

impl ReadableAndWritable for InventoryType {
    type Error = InventoryTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl InventoryType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InventoryTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::NON_EQUIP => 0x0,
            Self::HEAD => 0x1,
            Self::NECK_OR_RELIC => 0x2,
            Self::SHOULDERS => 0x3,
            Self::BODY => 0x4,
            Self::CHEST => 0x5,
            Self::WAIST => 0x6,
            Self::LEGS => 0x7,
            Self::FEET => 0x8,
            Self::WRISTS => 0x9,
            Self::HANDS => 0xa,
            Self::FINGER => 0xb,
            Self::TRINKET => 0xc,
            Self::WEAPON => 0xd,
            Self::SHIELD => 0xe,
            Self::RANGED => 0xf,
            Self::CLOAK => 0x10,
            Self::TWO_HANDED_WEAPON => 0x11,
            Self::BAG => 0x12,
            Self::TABARD => 0x13,
            Self::ROBE => 0x14,
            Self::WEAPON_MAIN_HAND => 0x15,
            Self::WEAPON_OFF_HAND => 0x16,
            Self::HOLDABLE => 0x17,
            Self::AMMO => 0x18,
            Self::THROWN => 0x19,
            Self::RANGED_RIGHT => 0x1a,
            Self::QUIVER => 0x1b,
        }
    }

    pub const fn new() -> Self {
        Self::NON_EQUIP
    }

}

impl ConstantSized for InventoryType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for InventoryType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for InventoryType {
    fn default() -> Self {
        Self::NON_EQUIP
    }
}

impl std::fmt::Display for InventoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NON_EQUIP => f.write_str("NON_EQUIP"),
            Self::HEAD => f.write_str("HEAD"),
            Self::NECK_OR_RELIC => f.write_str("NECK_OR_RELIC"),
            Self::SHOULDERS => f.write_str("SHOULDERS"),
            Self::BODY => f.write_str("BODY"),
            Self::CHEST => f.write_str("CHEST"),
            Self::WAIST => f.write_str("WAIST"),
            Self::LEGS => f.write_str("LEGS"),
            Self::FEET => f.write_str("FEET"),
            Self::WRISTS => f.write_str("WRISTS"),
            Self::HANDS => f.write_str("HANDS"),
            Self::FINGER => f.write_str("FINGER"),
            Self::TRINKET => f.write_str("TRINKET"),
            Self::WEAPON => f.write_str("WEAPON"),
            Self::SHIELD => f.write_str("SHIELD"),
            Self::RANGED => f.write_str("RANGED"),
            Self::CLOAK => f.write_str("CLOAK"),
            Self::TWO_HANDED_WEAPON => f.write_str("TWO_HANDED_WEAPON"),
            Self::BAG => f.write_str("BAG"),
            Self::TABARD => f.write_str("TABARD"),
            Self::ROBE => f.write_str("ROBE"),
            Self::WEAPON_MAIN_HAND => f.write_str("WEAPON_MAIN_HAND"),
            Self::WEAPON_OFF_HAND => f.write_str("WEAPON_OFF_HAND"),
            Self::HOLDABLE => f.write_str("HOLDABLE"),
            Self::AMMO => f.write_str("AMMO"),
            Self::THROWN => f.write_str("THROWN"),
            Self::RANGED_RIGHT => f.write_str("RANGED_RIGHT"),
            Self::QUIVER => f.write_str("QUIVER"),
        }
    }
}

impl TryFrom<u8> for InventoryType {
    type Error = TryFromInventoryTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NON_EQUIP),
            1 => Ok(Self::HEAD),
            2 => Ok(Self::NECK_OR_RELIC),
            3 => Ok(Self::SHOULDERS),
            4 => Ok(Self::BODY),
            5 => Ok(Self::CHEST),
            6 => Ok(Self::WAIST),
            7 => Ok(Self::LEGS),
            8 => Ok(Self::FEET),
            9 => Ok(Self::WRISTS),
            10 => Ok(Self::HANDS),
            11 => Ok(Self::FINGER),
            12 => Ok(Self::TRINKET),
            13 => Ok(Self::WEAPON),
            14 => Ok(Self::SHIELD),
            15 => Ok(Self::RANGED),
            16 => Ok(Self::CLOAK),
            17 => Ok(Self::TWO_HANDED_WEAPON),
            18 => Ok(Self::BAG),
            19 => Ok(Self::TABARD),
            20 => Ok(Self::ROBE),
            21 => Ok(Self::WEAPON_MAIN_HAND),
            22 => Ok(Self::WEAPON_OFF_HAND),
            23 => Ok(Self::HOLDABLE),
            24 => Ok(Self::AMMO),
            25 => Ok(Self::THROWN),
            26 => Ok(Self::RANGED_RIGHT),
            27 => Ok(Self::QUIVER),
            _ => Err(TryFromInventoryTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromInventoryTypeError {
    value: u8,
}

impl TryFromInventoryTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum InventoryTypeError {
    Read(std::io::Error),
    TryFrom(TryFromInventoryTypeError),
}

impl std::error::Error for InventoryTypeError {}
impl std::fmt::Display for TryFromInventoryTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'InventoryType': '{}'", self.value))
    }
}

impl std::fmt::Display for InventoryTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for InventoryTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromInventoryTypeError> for InventoryTypeError {
    fn from(value: TryFromInventoryTypeError) -> Self {
        Self::TryFrom(value)
    }
}

