use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/needs_else_if_else.wowm:471`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/needs_else_if_else.wowm#L471):
/// ```text
/// enum AuctionCommandAction : u32 {
///     STARTED = 0;
///     REMOVED = 1;
///     BID_PLACED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum AuctionCommandAction {
    STARTED,
    REMOVED,
    BID_PLACED,
}

impl ReadableAndWritable for AuctionCommandAction {
    type Error = AuctionCommandActionError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl AuctionCommandAction {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AuctionCommandActionError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AuctionCommandActionError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AuctionCommandActionError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::STARTED => 0x0,
            Self::REMOVED => 0x1,
            Self::BID_PLACED => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::STARTED
    }

}

impl ConstantSized for AuctionCommandAction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for AuctionCommandAction {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for AuctionCommandAction {
    fn default() -> Self {
        Self::STARTED
    }
}

impl std::fmt::Display for AuctionCommandAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STARTED => f.write_str("STARTED"),
            Self::REMOVED => f.write_str("REMOVED"),
            Self::BID_PLACED => f.write_str("BID_PLACED"),
        }
    }
}

impl TryFrom<u32> for AuctionCommandAction {
    type Error = TryFromAuctionCommandActionError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STARTED),
            1 => Ok(Self::REMOVED),
            2 => Ok(Self::BID_PLACED),
            _ => Err(TryFromAuctionCommandActionError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromAuctionCommandActionError {
    value: u32,
}

impl TryFromAuctionCommandActionError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum AuctionCommandActionError {
    Read(std::io::Error),
    TryFrom(TryFromAuctionCommandActionError),
}

impl std::error::Error for AuctionCommandActionError {}
impl std::fmt::Display for TryFromAuctionCommandActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'AuctionCommandAction': '{}'", self.value))
    }
}

impl std::fmt::Display for AuctionCommandActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for AuctionCommandActionError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromAuctionCommandActionError> for AuctionCommandActionError {
    fn from(value: TryFromAuctionCommandActionError) -> Self {
        Self::TryFrom(value)
    }
}

