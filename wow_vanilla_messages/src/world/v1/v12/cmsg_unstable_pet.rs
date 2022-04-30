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
pub struct CMSG_UNSTABLE_PET {
    pub npc_guid: Guid,
    pub pet_number: u32,
}

impl ClientMessageWrite for CMSG_UNSTABLE_PET {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_UNSTABLE_PET {
    const OPCODE: u16 = 0x0271;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc_guid,
            pet_number,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.write(w)?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::tokio_read(r).await?;

        // pet_number: u32
        let pet_number = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            npc_guid,
            pet_number,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.tokio_write(w).await?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::astd_read(r).await?;

        // pet_number: u32
        let pet_number = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            npc_guid,
            pet_number,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.astd_write(w).await?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes()).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_UNSTABLE_PET {}

impl MaximumPossibleSized for CMSG_UNSTABLE_PET {
    fn maximum_possible_size() -> usize {
        8 // npc_guid: Guid
        + 4 // pet_number: u32
    }
}

