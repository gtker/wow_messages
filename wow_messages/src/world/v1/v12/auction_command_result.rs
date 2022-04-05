use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:1814`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L1814):
/// ```text
/// enum AuctionCommandResult : u32 {
///     OK = 0;
///     ERR_INVENTORY = 1;
///     ERR_DATABASE = 2;
///     ERR_NOT_ENOUGH_MONEY = 3;
///     ERR_ITEM_NOT_FOUND = 4;
///     ERR_HIGHER_BID = 5;
///     ERR_BID_INCREMENT = 7;
///     ERR_BID_OWN = 10;
///     ERR_RESTRICTED_ACCOUNT = 13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum AuctionCommandResult {
    OK,
    ERR_INVENTORY,
    ERR_DATABASE,
    ERR_NOT_ENOUGH_MONEY,
    ERR_ITEM_NOT_FOUND,
    ERR_HIGHER_BID,
    ERR_BID_INCREMENT,
    ERR_BID_OWN,
    ERR_RESTRICTED_ACCOUNT,
}

impl ReadableAndWritable for AuctionCommandResult {
    type Error = AuctionCommandResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl AuctionCommandResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AuctionCommandResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AuctionCommandResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AuctionCommandResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::OK => 0x0,
            Self::ERR_INVENTORY => 0x1,
            Self::ERR_DATABASE => 0x2,
            Self::ERR_NOT_ENOUGH_MONEY => 0x3,
            Self::ERR_ITEM_NOT_FOUND => 0x4,
            Self::ERR_HIGHER_BID => 0x5,
            Self::ERR_BID_INCREMENT => 0x7,
            Self::ERR_BID_OWN => 0xa,
            Self::ERR_RESTRICTED_ACCOUNT => 0xd,
        }
    }

    pub const fn new() -> Self {
        Self::OK
    }

}

impl ConstantSized for AuctionCommandResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for AuctionCommandResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for AuctionCommandResult {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for AuctionCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::ERR_INVENTORY => f.write_str("ERR_INVENTORY"),
            Self::ERR_DATABASE => f.write_str("ERR_DATABASE"),
            Self::ERR_NOT_ENOUGH_MONEY => f.write_str("ERR_NOT_ENOUGH_MONEY"),
            Self::ERR_ITEM_NOT_FOUND => f.write_str("ERR_ITEM_NOT_FOUND"),
            Self::ERR_HIGHER_BID => f.write_str("ERR_HIGHER_BID"),
            Self::ERR_BID_INCREMENT => f.write_str("ERR_BID_INCREMENT"),
            Self::ERR_BID_OWN => f.write_str("ERR_BID_OWN"),
            Self::ERR_RESTRICTED_ACCOUNT => f.write_str("ERR_RESTRICTED_ACCOUNT"),
        }
    }
}

impl TryFrom<u32> for AuctionCommandResult {
    type Error = TryFromAuctionCommandResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::ERR_INVENTORY),
            2 => Ok(Self::ERR_DATABASE),
            3 => Ok(Self::ERR_NOT_ENOUGH_MONEY),
            4 => Ok(Self::ERR_ITEM_NOT_FOUND),
            5 => Ok(Self::ERR_HIGHER_BID),
            7 => Ok(Self::ERR_BID_INCREMENT),
            10 => Ok(Self::ERR_BID_OWN),
            13 => Ok(Self::ERR_RESTRICTED_ACCOUNT),
            _ => Err(TryFromAuctionCommandResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromAuctionCommandResultError {
    value: u32,
}

impl TryFromAuctionCommandResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum AuctionCommandResultError {
    Read(std::io::Error),
    TryFrom(TryFromAuctionCommandResultError),
}

impl std::error::Error for AuctionCommandResultError {}
impl std::fmt::Display for TryFromAuctionCommandResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'AuctionCommandResult': '{}'", self.value))
    }
}

impl std::fmt::Display for AuctionCommandResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for AuctionCommandResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromAuctionCommandResultError> for AuctionCommandResultError {
    fn from(value: TryFromAuctionCommandResultError) -> Self {
        Self::TryFrom(value)
    }
}

