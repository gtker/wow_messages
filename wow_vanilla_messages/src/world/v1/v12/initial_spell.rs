use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct InitialSpell {
    pub spell_id: u16,
    pub unknown1: u16,
}

impl InitialSpell {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(array_w)
    }
}

impl InitialSpell {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // spell_id: u16
        let spell_id = crate::util::tokio_read_u16_le(r).await?;

        // unknown1: u16
        let unknown1 = crate::util::tokio_read_u16_le(r).await?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // spell_id: u16
        let spell_id = crate::util::astd_read_u16_le(r).await?;

        // unknown1: u16
        let unknown1 = crate::util::astd_read_u16_le(r).await?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

}

