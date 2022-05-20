use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: u8,
}

impl ChannelMember {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ChannelMember {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // member_flags: u8
        let member_flags = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            member_flags,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // member_flags: u8
        let member_flags = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            guid,
            member_flags,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // member_flags: u8
        let member_flags = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            guid,
            member_flags,
        })
    }

}

