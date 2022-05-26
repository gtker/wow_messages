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
pub struct SMSG_SPELL_FAILED_OTHER {
    pub caster_guid: Guid,
    pub id: u32,
}

impl SMSG_SPELL_FAILED_OTHER {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_SPELL_FAILED_OTHER {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(12);
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x02a6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            caster_guid,
            id,
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
            // caster_guid: Guid
            let caster_guid = Guid::tokio_read(r).await?;

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                caster_guid,
                id,
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
            // caster_guid: Guid
            let caster_guid = Guid::astd_read(r).await?;

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                caster_guid,
                id,
            })
        })
    }

}

