use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PET_ACTION {
    pub pet_guid: Guid,
    pub data: u32,
    pub target_guid: Guid,
}

impl ClientMessageWrite for CMSG_PET_ACTION {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_PET_ACTION {
    const OPCODE: u16 = 0x0175;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            pet_guid,
            data,
            target_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.write(w)?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::tokio_read(r).await?;

        // data: u32
        let data = crate::util::tokio_read_u32_le(r).await?;

        // target_guid: Guid
        let target_guid = Guid::tokio_read(r).await?;

        Ok(Self {
            pet_guid,
            data,
            target_guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.tokio_write(w).await?;

        // data: u32
        w.write_all(&self.data.to_le_bytes()).await?;

        // target_guid: Guid
        self.target_guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::astd_read(r).await?;

        // data: u32
        let data = crate::util::astd_read_u32_le(r).await?;

        // target_guid: Guid
        let target_guid = Guid::astd_read(r).await?;

        Ok(Self {
            pet_guid,
            data,
            target_guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.astd_write(w).await?;

        // data: u32
        w.write_all(&self.data.to_le_bytes()).await?;

        // target_guid: Guid
        self.target_guid.astd_write(w).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PET_ACTION {}

impl MaximumPossibleSized for CMSG_PET_ACTION {
    fn maximum_possible_size() -> usize {
        8 // pet_guid: Guid
        + 4 // data: u32
        + 8 // target_guid: Guid
    }
}

