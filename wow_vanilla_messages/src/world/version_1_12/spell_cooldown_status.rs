use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SpellCooldownStatus {
    pub id: u32,
    pub cooldown_time_in_msecs: u32,
}

impl SpellCooldownStatus {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // cooldown_time_in_msecs: u32
        w.write_all(&self.cooldown_time_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl SpellCooldownStatus {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            cooldown_time_in_msecs,
        })
    }

}

