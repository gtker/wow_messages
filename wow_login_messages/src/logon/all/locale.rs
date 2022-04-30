use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Locale {
    EN_GB,
    EN_US,
    ES_MX,
    PT_BR,
    FR_FR,
    DE_DE,
    ES_ES,
    PT_PT,
    IT_IT,
    RU_RU,
    KO_KR,
    ZH_TW,
    EN_TW,
    EN_CN,
    OTHER(u32),
}

impl ReadableAndWritable for Locale {
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

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for Locale {
    type Error = std::io::Error;

    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u32_le(r).await?;

        Ok(a.into())
    }

}

impl Locale {
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
            Self::EN_GB => 0x656e4742,
            Self::EN_US => 0x656e5553,
            Self::ES_MX => 0x65734d58,
            Self::PT_BR => 0x70744252,
            Self::FR_FR => 0x66724652,
            Self::DE_DE => 0x64654445,
            Self::ES_ES => 0x65734553,
            Self::PT_PT => 0x70745054,
            Self::IT_IT => 0x69744954,
            Self::RU_RU => 0x72755255,
            Self::KO_KR => 0x6b6f4b52,
            Self::ZH_TW => 0x7a685457,
            Self::EN_TW => 0x656e5457,
            Self::EN_CN => 0x656e434e,
            Self::OTHER(v) => *v,
        }
    }

    pub const fn new() -> Self {
        Self::EN_GB
    }

}

impl ConstantSized for Locale {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Locale {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::EN_GB
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EN_GB => f.write_str("EN_GB"),
            Self::EN_US => f.write_str("EN_US"),
            Self::ES_MX => f.write_str("ES_MX"),
            Self::PT_BR => f.write_str("PT_BR"),
            Self::FR_FR => f.write_str("FR_FR"),
            Self::DE_DE => f.write_str("DE_DE"),
            Self::ES_ES => f.write_str("ES_ES"),
            Self::PT_PT => f.write_str("PT_PT"),
            Self::IT_IT => f.write_str("IT_IT"),
            Self::RU_RU => f.write_str("RU_RU"),
            Self::KO_KR => f.write_str("KO_KR"),
            Self::ZH_TW => f.write_str("ZH_TW"),
            Self::EN_TW => f.write_str("EN_TW"),
            Self::EN_CN => f.write_str("EN_CN"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Locale {
    fn from(value: u32) -> Self {
        match value {
            1701726018 => Self::EN_GB,
            1701729619 => Self::EN_US,
            1702055256 => Self::ES_MX,
            1886667346 => Self::PT_BR,
            1718765138 => Self::FR_FR,
            1684358213 => Self::DE_DE,
            1702053203 => Self::ES_ES,
            1886670932 => Self::PT_PT,
            1769228628 => Self::IT_IT,
            1920291413 => Self::RU_RU,
            1802455890 => Self::KO_KR,
            2053657687 => Self::ZH_TW,
            1701729367 => Self::EN_TW,
            1701725006 => Self::EN_CN,
            v => Self::OTHER(v)
        }
    }
}

