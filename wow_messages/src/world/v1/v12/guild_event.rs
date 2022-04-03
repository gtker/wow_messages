use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:443`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm#L443):
/// ```text
/// enum GuildEvent : u8 {
///     PROMOTION = 0;
///     DEMOTION = 1;
///     MOTD = 2;
///     JOINED = 3;
///     LEFT = 4;
///     REMOVED = 5;
///     LEADER_IS = 6;
///     LEADER_CHANGED = 7;
///     DISBANDED = 8;
///     TABARD_CHANGED = 9;
///     UNKNOWN10 = 10;
///     ROSTER_UPDATE = 11;
///     SIGNED_ON = 12;
///     SIGNED_OFF = 13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEvent {
    PROMOTION,
    DEMOTION,
    MOTD,
    JOINED,
    LEFT,
    REMOVED,
    LEADER_IS,
    LEADER_CHANGED,
    DISBANDED,
    TABARD_CHANGED,
    UNKNOWN10,
    ROSTER_UPDATE,
    SIGNED_ON,
    SIGNED_OFF,
}

impl ReadableAndWritable for GuildEvent {
    type Error = GuildEventError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl GuildEvent {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::PROMOTION => 0x0,
            Self::DEMOTION => 0x1,
            Self::MOTD => 0x2,
            Self::JOINED => 0x3,
            Self::LEFT => 0x4,
            Self::REMOVED => 0x5,
            Self::LEADER_IS => 0x6,
            Self::LEADER_CHANGED => 0x7,
            Self::DISBANDED => 0x8,
            Self::TABARD_CHANGED => 0x9,
            Self::UNKNOWN10 => 0xa,
            Self::ROSTER_UPDATE => 0xb,
            Self::SIGNED_ON => 0xc,
            Self::SIGNED_OFF => 0xd,
        }
    }

    pub const fn new() -> Self {
        Self::PROMOTION
    }

}

impl ConstantSized for GuildEvent {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GuildEvent {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GuildEvent {
    fn default() -> Self {
        Self::PROMOTION
    }
}

impl std::fmt::Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PROMOTION => f.write_str("PROMOTION"),
            Self::DEMOTION => f.write_str("DEMOTION"),
            Self::MOTD => f.write_str("MOTD"),
            Self::JOINED => f.write_str("JOINED"),
            Self::LEFT => f.write_str("LEFT"),
            Self::REMOVED => f.write_str("REMOVED"),
            Self::LEADER_IS => f.write_str("LEADER_IS"),
            Self::LEADER_CHANGED => f.write_str("LEADER_CHANGED"),
            Self::DISBANDED => f.write_str("DISBANDED"),
            Self::TABARD_CHANGED => f.write_str("TABARD_CHANGED"),
            Self::UNKNOWN10 => f.write_str("UNKNOWN10"),
            Self::ROSTER_UPDATE => f.write_str("ROSTER_UPDATE"),
            Self::SIGNED_ON => f.write_str("SIGNED_ON"),
            Self::SIGNED_OFF => f.write_str("SIGNED_OFF"),
        }
    }
}

impl TryFrom<u8> for GuildEvent {
    type Error = TryFromGuildEventError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PROMOTION),
            1 => Ok(Self::DEMOTION),
            2 => Ok(Self::MOTD),
            3 => Ok(Self::JOINED),
            4 => Ok(Self::LEFT),
            5 => Ok(Self::REMOVED),
            6 => Ok(Self::LEADER_IS),
            7 => Ok(Self::LEADER_CHANGED),
            8 => Ok(Self::DISBANDED),
            9 => Ok(Self::TABARD_CHANGED),
            10 => Ok(Self::UNKNOWN10),
            11 => Ok(Self::ROSTER_UPDATE),
            12 => Ok(Self::SIGNED_ON),
            13 => Ok(Self::SIGNED_OFF),
            _ => Err(TryFromGuildEventError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGuildEventError {
    value: u8,
}

impl TryFromGuildEventError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GuildEventError {
    Read(std::io::Error),
    TryFrom(TryFromGuildEventError),
}

impl std::error::Error for GuildEventError {}
impl std::fmt::Display for TryFromGuildEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GuildEvent': '{}'", self.value))
    }
}

impl std::fmt::Display for GuildEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GuildEventError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGuildEventError> for GuildEventError {
    fn from(value: TryFromGuildEventError) -> Self {
        Self::TryFrom(value)
    }
}

