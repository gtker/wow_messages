use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: u8,
}

impl ChannelMember {
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

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes())?;

        Ok(())
    }

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

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes()).await?;

        Ok(())
    }

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

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for ChannelMember {}

impl MaximumPossibleSized for ChannelMember {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // member_flags: u8
    }
}

