use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::FactionFlag;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct FactionInitializer {
    pub flag: FactionFlag,
    pub standing: u32,
}

impl FactionInitializer {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }
}

impl FactionInitializer {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // flag: FactionFlag
        let flag = FactionFlag::new(crate::util::read_u8_le(r)?);

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            flag,
            standing,
        })
    }

}

