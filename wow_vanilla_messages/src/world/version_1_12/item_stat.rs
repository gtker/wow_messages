use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ItemStat {
    pub item_stat_type: u32,
    pub item_stat_value: u32,
}

impl ItemStat {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes())?;

        // item_stat_value: u32
        w.write_all(&self.item_stat_value.to_le_bytes())?;

        Ok(())
    }
}

impl ItemStat {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_stat_type: u32
        let item_stat_type = crate::util::read_u32_le(r)?;

        // item_stat_value: u32
        let item_stat_value = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

}

