use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/sever.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/sever.wowm#L34):
/// ```text
/// enum Population : u32 {
///     GREEN_RECOMMENDED = 0x43480000;
///     RED_FULL = 0x43c80000;
///     BLUE_RECOMMENDED = 0x44160000;
///     OTHER = self.value
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Population {
    GREEN_RECOMMENDED,
    RED_FULL,
    BLUE_RECOMMENDED,
    OTHER(u32),
}

impl ReadableAndWritable for Population {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.into())
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl Population {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).into())
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).into())
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).into())
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::GREEN_RECOMMENDED => 0x43480000,
            Self::RED_FULL => 0x43c80000,
            Self::BLUE_RECOMMENDED => 0x44160000,
            Self::OTHER(v) => *v,
        }
    }

    pub const fn new() -> Self {
        Self::GREEN_RECOMMENDED
    }

}

impl ConstantSized for Population {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Population {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Population {
    fn default() -> Self {
        Self::GREEN_RECOMMENDED
    }
}

impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GREEN_RECOMMENDED => f.write_str("GREEN_RECOMMENDED"),
            Self::RED_FULL => f.write_str("RED_FULL"),
            Self::BLUE_RECOMMENDED => f.write_str("BLUE_RECOMMENDED"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Population {
    fn from(value: u32) -> Self {
        match value {
            1128792064 => Self::GREEN_RECOMMENDED,
            1137180672 => Self::RED_FULL,
            1142292480 => Self::BLUE_RECOMMENDED,
            v => Self::OTHER(v)
        }
    }
}

