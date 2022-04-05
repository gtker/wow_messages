use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:1022`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L1022):
/// ```text
/// enum BattlegroundWinner : u8 {
///     HORDE = 0;
///     ALLIANCE = 1;
///     NONE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlegroundWinner {
    HORDE,
    ALLIANCE,
    NONE,
}

impl ReadableAndWritable for BattlegroundWinner {
    type Error = BattlegroundWinnerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl BattlegroundWinner {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundWinnerError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundWinnerError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundWinnerError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundWinnerError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundWinnerError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlegroundWinnerError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::HORDE => 0x0,
            Self::ALLIANCE => 0x1,
            Self::NONE => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::HORDE
    }

}

impl ConstantSized for BattlegroundWinner {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BattlegroundWinner {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for BattlegroundWinner {
    fn default() -> Self {
        Self::HORDE
    }
}

impl std::fmt::Display for BattlegroundWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HORDE => f.write_str("HORDE"),
            Self::ALLIANCE => f.write_str("ALLIANCE"),
            Self::NONE => f.write_str("NONE"),
        }
    }
}

impl TryFrom<u8> for BattlegroundWinner {
    type Error = TryFromBattlegroundWinnerError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::HORDE),
            1 => Ok(Self::ALLIANCE),
            2 => Ok(Self::NONE),
            _ => Err(TryFromBattlegroundWinnerError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBattlegroundWinnerError {
    value: u8,
}

impl TryFromBattlegroundWinnerError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BattlegroundWinnerError {
    Read(std::io::Error),
    TryFrom(TryFromBattlegroundWinnerError),
}

impl std::error::Error for BattlegroundWinnerError {}
impl std::fmt::Display for TryFromBattlegroundWinnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BattlegroundWinner': '{}'", self.value))
    }
}

impl std::fmt::Display for BattlegroundWinnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BattlegroundWinnerError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBattlegroundWinnerError> for BattlegroundWinnerError {
    fn from(value: TryFromBattlegroundWinnerError) -> Self {
        Self::TryFrom(value)
    }
}

