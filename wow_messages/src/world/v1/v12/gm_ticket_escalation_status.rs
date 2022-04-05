use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:790`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L790):
/// ```text
/// enum GmTicketEscalationStatus : u8 {
///     GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED = 0;
///     GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED = 1;
///     GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketEscalationStatus {
    GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED,
    GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED,
    GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED,
}

impl ReadableAndWritable for GmTicketEscalationStatus {
    type Error = GmTicketEscalationStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl GmTicketEscalationStatus {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketEscalationStatusError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketEscalationStatusError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketEscalationStatusError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketEscalationStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketEscalationStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketEscalationStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED => 0x0,
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED => 0x1,
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED
    }

}

impl ConstantSized for GmTicketEscalationStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GmTicketEscalationStatus {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GmTicketEscalationStatus {
    fn default() -> Self {
        Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED
    }
}

impl std::fmt::Display for GmTicketEscalationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED => f.write_str("GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED"),
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED => f.write_str("GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED"),
            Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED => f.write_str("GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED"),
        }
    }
}

impl TryFrom<u8> for GmTicketEscalationStatus {
    type Error = TryFromGmTicketEscalationStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_NOT_ASSIGNED),
            1 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_ASSIGNED),
            2 => Ok(Self::GMTICKET_ASSIGNEDTOGM_STATUS_ESCALATED),
            _ => Err(TryFromGmTicketEscalationStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGmTicketEscalationStatusError {
    value: u8,
}

impl TryFromGmTicketEscalationStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GmTicketEscalationStatusError {
    Read(std::io::Error),
    TryFrom(TryFromGmTicketEscalationStatusError),
}

impl std::error::Error for GmTicketEscalationStatusError {}
impl std::fmt::Display for TryFromGmTicketEscalationStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketEscalationStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for GmTicketEscalationStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GmTicketEscalationStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGmTicketEscalationStatusError> for GmTicketEscalationStatusError {
    fn from(value: TryFromGmTicketEscalationStatusError) -> Self {
        Self::TryFrom(value)
    }
}

