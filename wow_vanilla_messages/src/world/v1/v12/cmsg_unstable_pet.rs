use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_UNSTABLE_PET {
    pub npc_guid: Guid,
    pub pet_number: u32,
}

impl CMSG_UNSTABLE_PET {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // npc_guid: Guid
        w.write_all(&self.npc_guid.guid().to_le_bytes())?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_UNSTABLE_PET {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc_guid: Guid
        w.write_all(&self.npc_guid.guid().to_le_bytes())?;

        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0271;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
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
            // npc_guid: Guid
            let npc_guid = Guid::tokio_read(r).await?;

            // pet_number: u32
            let pet_number = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                npc_guid,
                pet_number,
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
            // npc_guid: Guid
            let npc_guid = Guid::astd_read(r).await?;

            // pet_number: u32
            let pet_number = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                npc_guid,
                pet_number,
            })
        })
    }

}

