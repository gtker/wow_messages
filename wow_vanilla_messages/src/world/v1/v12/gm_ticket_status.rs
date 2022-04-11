use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketStatus {
    DBERROR,
    HASTEXT,
    DEFAULT,
}

impl ReadableAndWritable for GmTicketStatus {
    type Error = GmTicketStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl GmTicketStatus {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::DBERROR => 0x0,
            Self::HASTEXT => 0x6,
            Self::DEFAULT => 0xa,
        }
    }

    pub const fn new() -> Self {
        Self::DBERROR
    }

}

impl ConstantSized for GmTicketStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GmTicketStatus {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for GmTicketStatus {
    fn default() -> Self {
        Self::DBERROR
    }
}

impl std::fmt::Display for GmTicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DBERROR => f.write_str("DBERROR"),
            Self::HASTEXT => f.write_str("HASTEXT"),
            Self::DEFAULT => f.write_str("DEFAULT"),
        }
    }
}

impl TryFrom<u32> for GmTicketStatus {
    type Error = TryFromGmTicketStatusError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DBERROR),
            6 => Ok(Self::HASTEXT),
            10 => Ok(Self::DEFAULT),
            _ => Err(TryFromGmTicketStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGmTicketStatusError {
    value: u32,
}

impl TryFromGmTicketStatusError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GmTicketStatusError {
    Read(std::io::Error),
    TryFrom(TryFromGmTicketStatusError),
}

impl std::error::Error for GmTicketStatusError {}
impl std::fmt::Display for TryFromGmTicketStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for GmTicketStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GmTicketStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGmTicketStatusError> for GmTicketStatusError {
    fn from(value: TryFromGmTicketStatusError) -> Self {
        Self::TryFrom(value)
    }
}

