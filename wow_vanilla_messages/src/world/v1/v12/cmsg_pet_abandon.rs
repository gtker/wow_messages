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
pub struct CMSG_PET_ABANDON {
    pub pet_guid: Guid,
}

impl CMSG_PET_ABANDON {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_PET_ABANDON {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8);
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0176;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        Ok(Self {
            pet_guid,
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
            // pet_guid: Guid
            let pet_guid = Guid::tokio_read(r).await?;

            Ok(Self {
                pet_guid,
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
            // pet_guid: Guid
            let pet_guid = Guid::astd_read(r).await?;

            Ok(Self {
                pet_guid,
            })
        })
    }

}

