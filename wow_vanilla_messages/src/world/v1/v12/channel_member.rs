use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: u8,
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

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes())?;

        Ok(())
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes()).await?;

        Ok(())
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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes()).await?;

        Ok(())
    }

}

impl ChannelMember {
    pub(crate) fn size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // member_flags: u8
    }
}

