use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:1055`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L1055):
/// ```text
/// enum ServerMessageType : u32 {
///     SHUTDOWN_TIME = 1;
///     RESTART_TIME = 2;
///     CUSTOM = 3;
///     SHUTDOWN_CANCELLED = 4;
///     RESTART_CANCELLED = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ServerMessageType {
    SHUTDOWN_TIME,
    RESTART_TIME,
    CUSTOM,
    SHUTDOWN_CANCELLED,
    RESTART_CANCELLED,
}

impl ReadableAndWritable for ServerMessageType {
    type Error = ServerMessageTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl ServerMessageType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ServerMessageTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::SHUTDOWN_TIME => 0x1,
            Self::RESTART_TIME => 0x2,
            Self::CUSTOM => 0x3,
            Self::SHUTDOWN_CANCELLED => 0x4,
            Self::RESTART_CANCELLED => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::SHUTDOWN_TIME
    }

}

impl ConstantSized for ServerMessageType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ServerMessageType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for ServerMessageType {
    fn default() -> Self {
        Self::SHUTDOWN_TIME
    }
}

impl std::fmt::Display for ServerMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SHUTDOWN_TIME => f.write_str("SHUTDOWN_TIME"),
            Self::RESTART_TIME => f.write_str("RESTART_TIME"),
            Self::CUSTOM => f.write_str("CUSTOM"),
            Self::SHUTDOWN_CANCELLED => f.write_str("SHUTDOWN_CANCELLED"),
            Self::RESTART_CANCELLED => f.write_str("RESTART_CANCELLED"),
        }
    }
}

impl TryFrom<u32> for ServerMessageType {
    type Error = TryFromServerMessageTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SHUTDOWN_TIME),
            2 => Ok(Self::RESTART_TIME),
            3 => Ok(Self::CUSTOM),
            4 => Ok(Self::SHUTDOWN_CANCELLED),
            5 => Ok(Self::RESTART_CANCELLED),
            _ => Err(TryFromServerMessageTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromServerMessageTypeError {
    value: u32,
}

impl TryFromServerMessageTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ServerMessageTypeError {
    Read(std::io::Error),
    TryFrom(TryFromServerMessageTypeError),
}

impl std::error::Error for ServerMessageTypeError {}
impl std::fmt::Display for TryFromServerMessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ServerMessageType': '{}'", self.value))
    }
}

impl std::fmt::Display for ServerMessageTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ServerMessageTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromServerMessageTypeError> for ServerMessageTypeError {
    fn from(value: TryFromServerMessageTypeError) -> Self {
        Self::TryFrom(value)
    }
}

