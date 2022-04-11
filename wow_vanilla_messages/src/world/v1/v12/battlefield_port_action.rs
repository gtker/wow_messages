use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlefieldPortAction {
    LEAVE_QUEUE,
    ENTER_BATTLE,
}

impl ReadableAndWritable for BattlefieldPortAction {
    type Error = BattlefieldPortActionError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl BattlefieldPortAction {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlefieldPortActionError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlefieldPortActionError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlefieldPortActionError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlefieldPortActionError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlefieldPortActionError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BattlefieldPortActionError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::LEAVE_QUEUE => 0x0,
            Self::ENTER_BATTLE => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::LEAVE_QUEUE
    }

}

impl ConstantSized for BattlefieldPortAction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BattlefieldPortAction {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for BattlefieldPortAction {
    fn default() -> Self {
        Self::LEAVE_QUEUE
    }
}

impl std::fmt::Display for BattlefieldPortAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LEAVE_QUEUE => f.write_str("LEAVE_QUEUE"),
            Self::ENTER_BATTLE => f.write_str("ENTER_BATTLE"),
        }
    }
}

impl TryFrom<u8> for BattlefieldPortAction {
    type Error = TryFromBattlefieldPortActionError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LEAVE_QUEUE),
            1 => Ok(Self::ENTER_BATTLE),
            _ => Err(TryFromBattlefieldPortActionError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBattlefieldPortActionError {
    value: u8,
}

impl TryFromBattlefieldPortActionError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BattlefieldPortActionError {
    Read(std::io::Error),
    TryFrom(TryFromBattlefieldPortActionError),
}

impl std::error::Error for BattlefieldPortActionError {}
impl std::fmt::Display for TryFromBattlefieldPortActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BattlefieldPortAction': '{}'", self.value))
    }
}

impl std::fmt::Display for BattlefieldPortActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BattlefieldPortActionError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBattlefieldPortActionError> for BattlefieldPortActionError {
    fn from(value: TryFromBattlefieldPortActionError) -> Self {
        Self::TryFrom(value)
    }
}

