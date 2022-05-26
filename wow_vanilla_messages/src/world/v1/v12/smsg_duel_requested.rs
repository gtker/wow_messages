use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_DUEL_REQUESTED {
    pub initiator_guid: Guid,
    pub target_guid: Guid,
}

impl SMSG_DUEL_REQUESTED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // initiator_guid: Guid
        w.write_all(&self.initiator_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_DUEL_REQUESTED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(16);
        // initiator_guid: Guid
        w.write_all(&self.initiator_guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0167;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // initiator_guid: Guid
        let initiator_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            initiator_guid,
            target_guid,
        })
    }

    #[cfg(feature = "tokio")]
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
            // initiator_guid: Guid
            let initiator_guid = Guid::tokio_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            Ok(Self {
                initiator_guid,
                target_guid,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // initiator_guid: Guid
            let initiator_guid = Guid::astd_read(r).await?;

            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            Ok(Self {
                initiator_guid,
                target_guid,
            })
        })
    }

}

