use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_gm_ticket_status_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_gm_ticket_status_update.wowm#L3):
/// ```text
/// enum GmTicketStatusResponse : u32 {
///     UPDATED = 1;
///     CLOSED = 2;
///     SURVEY = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketStatusResponse {
    UPDATED,
    CLOSED,
    SURVEY,
}

impl ReadableAndWritable for GmTicketStatusResponse {
    type Error = GmTicketStatusResponseError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl GmTicketStatusResponse {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketStatusResponseError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketStatusResponseError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketStatusResponseError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::UPDATED => 0x1,
            Self::CLOSED => 0x2,
            Self::SURVEY => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::UPDATED
    }

}

impl ConstantSized for GmTicketStatusResponse {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GmTicketStatusResponse {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for GmTicketStatusResponse {
    fn default() -> Self {
        Self::UPDATED
    }
}

impl std::fmt::Display for GmTicketStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UPDATED => f.write_str("UPDATED"),
            Self::CLOSED => f.write_str("CLOSED"),
            Self::SURVEY => f.write_str("SURVEY"),
        }
    }
}

impl TryFrom<u32> for GmTicketStatusResponse {
    type Error = TryFromGmTicketStatusResponseError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::UPDATED),
            2 => Ok(Self::CLOSED),
            3 => Ok(Self::SURVEY),
            _ => Err(TryFromGmTicketStatusResponseError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGmTicketStatusResponseError {
    value: u32,
}

impl TryFromGmTicketStatusResponseError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GmTicketStatusResponseError {
    Read(std::io::Error),
    TryFrom(TryFromGmTicketStatusResponseError),
}

impl std::error::Error for GmTicketStatusResponseError {}
impl std::fmt::Display for TryFromGmTicketStatusResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketStatusResponse': '{}'", self.value))
    }
}

impl std::fmt::Display for GmTicketStatusResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GmTicketStatusResponseError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGmTicketStatusResponseError> for GmTicketStatusResponseError {
    fn from(value: TryFromGmTicketStatusResponseError) -> Self {
        Self::TryFrom(value)
    }
}

