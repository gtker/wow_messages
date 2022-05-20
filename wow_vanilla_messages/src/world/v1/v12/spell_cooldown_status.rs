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
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // cooldown_time_in_msecs: u32
        w.write_all(&self.cooldown_time_in_msecs.to_le_bytes())?;

        Ok(array_w)
    }
}

impl SpellCooldownStatus {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            cooldown_time_in_msecs,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::tokio_read_u32_le(r).await?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            id,
            cooldown_time_in_msecs,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::astd_read_u32_le(r).await?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            id,
            cooldown_time_in_msecs,
        })
    }

}

impl SpellCooldownStatus {
    pub(crate) fn size() -> usize {
        0
        + 4 // id: u32
        + 4 // cooldown_time_in_msecs: u32
    }
}

