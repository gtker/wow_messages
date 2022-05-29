use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct WorldState {
    pub state: u32,
    pub value: u32,
}

impl WorldState {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
}

impl WorldState {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::read_u32_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            state,
            value,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::tokio_read_u32_le(r).await?;

        // value: u32
        let value = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            state,
            value,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::astd_read_u32_le(r).await?;

        // value: u32
        let value = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            state,
            value,
        })
    }

}

