use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
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

impl MessageBody for SMSG_ATTACKSTART {
    const OPCODE: u16 = 0x0143;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // attacker_guid: Guid
        self.attacker_guid.write(w)?;

        // victim_guid: Guid
        self.victim_guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // attacker_guid: Guid
            let attacker_guid = Guid::tokio_read(r).await?;

            // victim_guid: Guid
            let victim_guid = Guid::tokio_read(r).await?;

            Ok(Self {
                attacker_guid,
                victim_guid,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // attacker_guid: Guid
            self.attacker_guid.tokio_write(w).await?;

            // victim_guid: Guid
            self.victim_guid.tokio_write(w).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // attacker_guid: Guid
            let attacker_guid = Guid::astd_read(r).await?;

            // victim_guid: Guid
            let victim_guid = Guid::astd_read(r).await?;

            Ok(Self {
                attacker_guid,
                victim_guid,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // attacker_guid: Guid
            self.attacker_guid.astd_write(w).await?;

            // victim_guid: Guid
            self.victim_guid.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_ATTACKSTART {}

impl MaximumPossibleSized for SMSG_ATTACKSTART {
    fn maximum_possible_size() -> usize {
        0
        + 8 // attacker_guid: Guid
        + 8 // victim_guid: Guid
    }
}

