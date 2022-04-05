use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:729`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L729):
/// ```text
/// enum PartyMemberStatus : u32 {
///     OFFLINE = 0x0000;
///     ONLINE = 0x0001;
///     PVP = 0x0002;
///     DEAD = 0x0004;
///     GHOST = 0x0008;
///     PVP_FFA = 0x0010;
///     ZONE_OUT = 0x0020;
///     AFK = 0x0040;
///     DND = 0x0080;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PartyMemberStatus {
    OFFLINE,
    ONLINE,
    PVP,
    DEAD,
    GHOST,
    PVP_FFA,
    ZONE_OUT,
    AFK,
    DND,
}

impl ReadableAndWritable for PartyMemberStatus {
    type Error = PartyMemberStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl PartyMemberStatus {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyMemberStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyMemberStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyMemberStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::OFFLINE => 0x0,
            Self::ONLINE => 0x1,
            Self::PVP => 0x2,
            Self::DEAD => 0x4,
            Self::GHOST => 0x8,
            Self::PVP_FFA => 0x10,
            Self::ZONE_OUT => 0x20,
            Self::AFK => 0x40,
            Self::DND => 0x80,
        }
    }

    pub const fn new() -> Self {
        Self::OFFLINE
    }

}

impl ConstantSized for PartyMemberStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PartyMemberStatus {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for PartyMemberStatus {
    fn default() -> Self {
        Self::OFFLINE
    }
}

impl std::fmt::Display for PartyMemberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ONLINE => f.write_str("ONLINE"),
            Self::PVP => f.write_str("PVP"),
            Self::DEAD => f.write_str("DEAD"),
            Self::GHOST => f.write_str("GHOST"),
            Self::PVP_FFA => f.write_str("PVP_FFA"),
            Self::ZONE_OUT => f.write_str("ZONE_OUT"),
            Self::AFK => f.write_str("AFK"),
            Self::DND => f.write_str("DND"),
        }
    }
}

impl TryFrom<u32> for PartyMemberStatus {
    type Error = TryFromPartyMemberStatusError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OFFLINE),
            1 => Ok(Self::ONLINE),
            2 => Ok(Self::PVP),
            4 => Ok(Self::DEAD),
            8 => Ok(Self::GHOST),
            16 => Ok(Self::PVP_FFA),
            32 => Ok(Self::ZONE_OUT),
            64 => Ok(Self::AFK),
            128 => Ok(Self::DND),
            _ => Err(TryFromPartyMemberStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPartyMemberStatusError {
    value: u32,
}

impl TryFromPartyMemberStatusError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PartyMemberStatusError {
    Read(std::io::Error),
    TryFrom(TryFromPartyMemberStatusError),
}

impl std::error::Error for PartyMemberStatusError {}
impl std::fmt::Display for TryFromPartyMemberStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PartyMemberStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for PartyMemberStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PartyMemberStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPartyMemberStatusError> for PartyMemberStatusError {
    fn from(value: TryFromPartyMemberStatusError) -> Self {
        Self::TryFrom(value)
    }
}

