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
pub struct SMSG_ATTACKSTART {
    pub attacker_guid: Guid,
    pub victim_guid: Guid,
}

impl ServerMessageWrite for SMSG_ATTACKSTART {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_ATTACKSTART {
    const OPCODE: u16 = 0x0143;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // attacker_guid: Guid
        let attacker_guid = Guid::read(r)?;

        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // attacker_guid: Guid
        self.attacker_guid.write(w)?;

        // victim_guid: Guid
        self.victim_guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // attacker_guid: Guid
        let attacker_guid = Guid::tokio_read(r).await?;

        // victim_guid: Guid
        let victim_guid = Guid::tokio_read(r).await?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // attacker_guid: Guid
        self.attacker_guid.tokio_write(w).await?;

        // victim_guid: Guid
        self.victim_guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // attacker_guid: Guid
        let attacker_guid = Guid::astd_read(r).await?;

        // victim_guid: Guid
        let victim_guid = Guid::astd_read(r).await?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // attacker_guid: Guid
        self.attacker_guid.astd_write(w).await?;

        // victim_guid: Guid
        self.victim_guid.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_ATTACKSTART {}

impl MaximumPossibleSized for SMSG_ATTACKSTART {
    fn maximum_possible_size() -> usize {
        8 // attacker_guid: Guid
        + 8 // victim_guid: Guid
    }
}

