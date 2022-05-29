use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct Faction {
    pub reputation_list_id: u32,
    pub standing: u32,
}

impl Faction {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }
}

impl Faction {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

}

