use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ItemDamageType {
    pub damage_minimum: u32,
    pub damage_maximum: u32,
    pub damage_type: u32,
}

impl ItemDamageType {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // damage_minimum: u32
        w.write_all(&self.damage_minimum.to_le_bytes())?;

        // damage_maximum: u32
        w.write_all(&self.damage_maximum.to_le_bytes())?;

        // damage_type: u32
        w.write_all(&self.damage_type.to_le_bytes())?;

        Ok(())
    }
}

impl ItemDamageType {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // damage_minimum: u32
        let damage_minimum = crate::util::read_u32_le(r)?;

        // damage_maximum: u32
        let damage_maximum = crate::util::read_u32_le(r)?;

        // damage_type: u32
        let damage_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            damage_type,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // damage_minimum: u32
        let damage_minimum = crate::util::tokio_read_u32_le(r).await?;

        // damage_maximum: u32
        let damage_maximum = crate::util::tokio_read_u32_le(r).await?;

        // damage_type: u32
        let damage_type = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            damage_type,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // damage_minimum: u32
        let damage_minimum = crate::util::astd_read_u32_le(r).await?;

        // damage_maximum: u32
        let damage_maximum = crate::util::astd_read_u32_le(r).await?;

        // damage_type: u32
        let damage_type = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            damage_type,
        })
    }

}

