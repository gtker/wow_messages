use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:467`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm#L467):
/// ```text
/// enum GuildCommand : u8 {
///     CREATE = 0x00;
///     INVITE = 0x01;
///     QUIT = 0x03;
///     FOUNDER = 0x0E;
///     UNKNOWN19 = 0x13;
///     UNKNOWN20 = 0x14;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildCommand {
    CREATE,
    INVITE,
    QUIT,
    FOUNDER,
    UNKNOWN19,
    UNKNOWN20,
}

impl ReadableAndWritable for GuildCommand {
    type Error = GuildCommandError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl GuildCommand {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildCommandError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildCommandError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildCommandError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildCommandError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildCommandError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildCommandError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CREATE => 0x0,
            Self::INVITE => 0x1,
            Self::QUIT => 0x3,
            Self::FOUNDER => 0xe,
            Self::UNKNOWN19 => 0x13,
            Self::UNKNOWN20 => 0x14,
        }
    }

    pub const fn new() -> Self {
        Self::CREATE
    }

}

impl ConstantSized for GuildCommand {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GuildCommand {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GuildCommand {
    fn default() -> Self {
        Self::CREATE
    }
}

impl std::fmt::Display for GuildCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CREATE => f.write_str("CREATE"),
            Self::INVITE => f.write_str("INVITE"),
            Self::QUIT => f.write_str("QUIT"),
            Self::FOUNDER => f.write_str("FOUNDER"),
            Self::UNKNOWN19 => f.write_str("UNKNOWN19"),
            Self::UNKNOWN20 => f.write_str("UNKNOWN20"),
        }
    }
}

impl TryFrom<u8> for GuildCommand {
    type Error = TryFromGuildCommandError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CREATE),
            1 => Ok(Self::INVITE),
            3 => Ok(Self::QUIT),
            14 => Ok(Self::FOUNDER),
            19 => Ok(Self::UNKNOWN19),
            20 => Ok(Self::UNKNOWN20),
            _ => Err(TryFromGuildCommandError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGuildCommandError {
    value: u8,
}

impl TryFromGuildCommandError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GuildCommandError {
    Read(std::io::Error),
    TryFrom(TryFromGuildCommandError),
}

impl std::error::Error for GuildCommandError {}
impl std::fmt::Display for TryFromGuildCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GuildCommand': '{}'", self.value))
    }
}

impl std::fmt::Display for GuildCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GuildCommandError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGuildCommandError> for GuildCommandError {
    fn from(value: TryFromGuildCommandError) -> Self {
        Self::TryFrom(value)
    }
}

