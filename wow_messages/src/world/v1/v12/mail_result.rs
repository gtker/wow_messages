use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new2.wowm:768`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new2.wowm#L768):
/// ```text
/// enum MailResult : u32 {
///     OK = 0;
///     ERR_EQUIP_ERROR = 1;
///     ERR_CANNOT_SEND_TO_SELF = 2;
///     ERR_NOT_ENOUGH_MONEY = 3;
///     ERR_RECIPIENT_NOT_FOUND = 4;
///     ERR_NOT_YOUR_TEAM = 5;
///     ERR_INTERNAL_ERROR = 6;
///     ERR_DISABLED_FOR_TRIAL_ACC = 14;
///     ERR_RECIPIENT_CAP_REACHED = 15;
///     ERR_CANT_SEND_WRAPPED_COD = 16;
///     ERR_MAIL_AND_CHAT_SUSPENDED = 17;
///     ERR_TOO_MANY_ATTACHMENTS = 18;
///     ERR_MAIL_ATTACHMENT_INVALID = 19;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MailResult {
    OK,
    ERR_EQUIP_ERROR,
    ERR_CANNOT_SEND_TO_SELF,
    ERR_NOT_ENOUGH_MONEY,
    ERR_RECIPIENT_NOT_FOUND,
    ERR_NOT_YOUR_TEAM,
    ERR_INTERNAL_ERROR,
    ERR_DISABLED_FOR_TRIAL_ACC,
    ERR_RECIPIENT_CAP_REACHED,
    ERR_CANT_SEND_WRAPPED_COD,
    ERR_MAIL_AND_CHAT_SUSPENDED,
    ERR_TOO_MANY_ATTACHMENTS,
    ERR_MAIL_ATTACHMENT_INVALID,
}

impl ReadableAndWritable for MailResult {
    type Error = MailResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl MailResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailResultError> {
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
            Self::ERR_EQUIP_ERROR => 0x1,
            Self::ERR_CANNOT_SEND_TO_SELF => 0x2,
            Self::ERR_NOT_ENOUGH_MONEY => 0x3,
            Self::ERR_RECIPIENT_NOT_FOUND => 0x4,
            Self::ERR_NOT_YOUR_TEAM => 0x5,
            Self::ERR_INTERNAL_ERROR => 0x6,
            Self::ERR_DISABLED_FOR_TRIAL_ACC => 0xe,
            Self::ERR_RECIPIENT_CAP_REACHED => 0xf,
            Self::ERR_CANT_SEND_WRAPPED_COD => 0x10,
            Self::ERR_MAIL_AND_CHAT_SUSPENDED => 0x11,
            Self::ERR_TOO_MANY_ATTACHMENTS => 0x12,
            Self::ERR_MAIL_ATTACHMENT_INVALID => 0x13,
        }
    }

    pub const fn new() -> Self {
        Self::OK
    }

}

impl ConstantSized for MailResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MailResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for MailResult {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for MailResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::ERR_EQUIP_ERROR => f.write_str("ERR_EQUIP_ERROR"),
            Self::ERR_CANNOT_SEND_TO_SELF => f.write_str("ERR_CANNOT_SEND_TO_SELF"),
            Self::ERR_NOT_ENOUGH_MONEY => f.write_str("ERR_NOT_ENOUGH_MONEY"),
            Self::ERR_RECIPIENT_NOT_FOUND => f.write_str("ERR_RECIPIENT_NOT_FOUND"),
            Self::ERR_NOT_YOUR_TEAM => f.write_str("ERR_NOT_YOUR_TEAM"),
            Self::ERR_INTERNAL_ERROR => f.write_str("ERR_INTERNAL_ERROR"),
            Self::ERR_DISABLED_FOR_TRIAL_ACC => f.write_str("ERR_DISABLED_FOR_TRIAL_ACC"),
            Self::ERR_RECIPIENT_CAP_REACHED => f.write_str("ERR_RECIPIENT_CAP_REACHED"),
            Self::ERR_CANT_SEND_WRAPPED_COD => f.write_str("ERR_CANT_SEND_WRAPPED_COD"),
            Self::ERR_MAIL_AND_CHAT_SUSPENDED => f.write_str("ERR_MAIL_AND_CHAT_SUSPENDED"),
            Self::ERR_TOO_MANY_ATTACHMENTS => f.write_str("ERR_TOO_MANY_ATTACHMENTS"),
            Self::ERR_MAIL_ATTACHMENT_INVALID => f.write_str("ERR_MAIL_ATTACHMENT_INVALID"),
        }
    }
}

impl TryFrom<u32> for MailResult {
    type Error = TryFromMailResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::ERR_EQUIP_ERROR),
            2 => Ok(Self::ERR_CANNOT_SEND_TO_SELF),
            3 => Ok(Self::ERR_NOT_ENOUGH_MONEY),
            4 => Ok(Self::ERR_RECIPIENT_NOT_FOUND),
            5 => Ok(Self::ERR_NOT_YOUR_TEAM),
            6 => Ok(Self::ERR_INTERNAL_ERROR),
            14 => Ok(Self::ERR_DISABLED_FOR_TRIAL_ACC),
            15 => Ok(Self::ERR_RECIPIENT_CAP_REACHED),
            16 => Ok(Self::ERR_CANT_SEND_WRAPPED_COD),
            17 => Ok(Self::ERR_MAIL_AND_CHAT_SUSPENDED),
            18 => Ok(Self::ERR_TOO_MANY_ATTACHMENTS),
            19 => Ok(Self::ERR_MAIL_ATTACHMENT_INVALID),
            _ => Err(TryFromMailResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMailResultError {
    value: u32,
}

impl TryFromMailResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MailResultError {
    Read(std::io::Error),
    TryFrom(TryFromMailResultError),
}

impl std::error::Error for MailResultError {}
impl std::fmt::Display for TryFromMailResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MailResult': '{}'", self.value))
    }
}

impl std::fmt::Display for MailResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MailResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMailResultError> for MailResultError {
    fn from(value: TryFromMailResultError) -> Self {
        Self::TryFrom(value)
    }
}

