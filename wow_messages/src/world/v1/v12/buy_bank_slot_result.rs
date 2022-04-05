use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:372`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L372):
/// ```text
/// enum BuyBankSlotResult : u32 {
///     FAILED_TOO_MANY = 0;
///     INSUFFICIENT_FUNDS = 1;
///     NOTBANKER = 2;
///     OK = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BuyBankSlotResult {
    FAILED_TOO_MANY,
    INSUFFICIENT_FUNDS,
    NOTBANKER,
    OK,
}

impl ReadableAndWritable for BuyBankSlotResult {
    type Error = BuyBankSlotResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl BuyBankSlotResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyBankSlotResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyBankSlotResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyBankSlotResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::FAILED_TOO_MANY => 0x0,
            Self::INSUFFICIENT_FUNDS => 0x1,
            Self::NOTBANKER => 0x2,
            Self::OK => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::FAILED_TOO_MANY
    }

}

impl ConstantSized for BuyBankSlotResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BuyBankSlotResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for BuyBankSlotResult {
    fn default() -> Self {
        Self::FAILED_TOO_MANY
    }
}

impl std::fmt::Display for BuyBankSlotResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FAILED_TOO_MANY => f.write_str("FAILED_TOO_MANY"),
            Self::INSUFFICIENT_FUNDS => f.write_str("INSUFFICIENT_FUNDS"),
            Self::NOTBANKER => f.write_str("NOTBANKER"),
            Self::OK => f.write_str("OK"),
        }
    }
}

impl TryFrom<u32> for BuyBankSlotResult {
    type Error = TryFromBuyBankSlotResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FAILED_TOO_MANY),
            1 => Ok(Self::INSUFFICIENT_FUNDS),
            2 => Ok(Self::NOTBANKER),
            3 => Ok(Self::OK),
            _ => Err(TryFromBuyBankSlotResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBuyBankSlotResultError {
    value: u32,
}

impl TryFromBuyBankSlotResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BuyBankSlotResultError {
    Read(std::io::Error),
    TryFrom(TryFromBuyBankSlotResultError),
}

impl std::error::Error for BuyBankSlotResultError {}
impl std::fmt::Display for TryFromBuyBankSlotResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BuyBankSlotResult': '{}'", self.value))
    }
}

impl std::fmt::Display for BuyBankSlotResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BuyBankSlotResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBuyBankSlotResultError> for BuyBankSlotResultError {
    fn from(value: TryFromBuyBankSlotResultError) -> Self {
        Self::TryFrom(value)
    }
}

