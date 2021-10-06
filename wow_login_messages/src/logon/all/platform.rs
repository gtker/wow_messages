use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm):
/// ```text
/// enum Platform : u32 {
///     X86 = "\0x86";
///     PPC = "\0PPC";
///     OTHER = self.value
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Platform {
    X86,
    PPC,
    OTHER(u32),
}

impl ReadableAndWritable for Platform {
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

impl Platform {
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
            Self::X86 => 0x783836,
            Self::PPC => 0x505043,
            Self::OTHER(v) => *v,
        }
    }

    pub const fn new() -> Self {
        Self::X86
    }

}

impl ConstantSized for Platform {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Platform {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::X86
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X86 => f.write_str("X86"),
            Self::PPC => f.write_str("PPC"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Platform {
    fn from(value: u32) -> Self {
        match value {
            7878710 => Self::X86,
            5263427 => Self::PPC,
            v => Self::OTHER(v)
        }
    }
}

