use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_response.wowm#L12):
/// ```text
/// enum LogoutSpeed : u8 {
///     DELAYED = 0;
///     INSTANT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogoutSpeed {
    DELAYED,
    INSTANT,
}

impl ReadableAndWritable for LogoutSpeed {
    type Error = LogoutSpeedError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl LogoutSpeed {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutSpeedError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutSpeedError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutSpeedError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutSpeedError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutSpeedError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutSpeedError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::DELAYED => 0x0,
            Self::INSTANT => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::DELAYED
    }

}

impl ConstantSized for LogoutSpeed {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for LogoutSpeed {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for LogoutSpeed {
    fn default() -> Self {
        Self::DELAYED
    }
}

impl std::fmt::Display for LogoutSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DELAYED => f.write_str("DELAYED"),
            Self::INSTANT => f.write_str("INSTANT"),
        }
    }
}

impl TryFrom<u8> for LogoutSpeed {
    type Error = TryFromLogoutSpeedError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DELAYED),
            1 => Ok(Self::INSTANT),
            _ => Err(TryFromLogoutSpeedError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromLogoutSpeedError {
    value: u8,
}

impl TryFromLogoutSpeedError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum LogoutSpeedError {
    Read(std::io::Error),
    TryFrom(TryFromLogoutSpeedError),
}

impl std::error::Error for LogoutSpeedError {}
impl std::fmt::Display for TryFromLogoutSpeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LogoutSpeed': '{}'", self.value))
    }
}

impl std::fmt::Display for LogoutSpeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for LogoutSpeedError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromLogoutSpeedError> for LogoutSpeedError {
    fn from(value: TryFromLogoutSpeedError) -> Self {
        Self::TryFrom(value)
    }
}

