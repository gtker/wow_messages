use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct GossipItem {
    pub id: u32,
    pub item_icon: u8,
    pub coded: u8,
}

impl GossipItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: u8
        w.write_all(&self.coded.to_le_bytes())?;

        Ok(())
    }
}

impl GossipItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_icon: u8
        let item_icon = crate::util::read_u8_le(r)?;

        // coded: u8
        let coded = crate::util::read_u8_le(r)?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

}

