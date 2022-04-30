use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_PET_UNLEARN_CONFIRM {
    pub pet_guid: Guid,
    pub talent_reset_cost: u32,
}

impl ServerMessageWrite for SMSG_PET_UNLEARN_CONFIRM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PET_UNLEARN_CONFIRM {
    const OPCODE: u16 = 0x02f1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet_guid,
            talent_reset_cost,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.write(w)?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::tokio_read(r).await?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            pet_guid,
            talent_reset_cost,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.tokio_write(w).await?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::astd_read(r).await?;

        // talent_reset_cost: u32
        let talent_reset_cost = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            pet_guid,
            talent_reset_cost,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_guid: Guid
        self.pet_guid.astd_write(w).await?;

        // talent_reset_cost: u32
        w.write_all(&self.talent_reset_cost.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_PET_UNLEARN_CONFIRM {}

impl MaximumPossibleSized for SMSG_PET_UNLEARN_CONFIRM {
    fn maximum_possible_size() -> usize {
        8 // pet_guid: Guid
        + 4 // talent_reset_cost: u32
    }
}

