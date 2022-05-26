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
pub struct SMSG_PARTYKILLLOG {
    pub player_with_killing_blow: Guid,
    pub victim: Guid,
}

impl SMSG_PARTYKILLLOG {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // player_with_killing_blow: Guid
        w.write_all(&self.player_with_killing_blow.guid().to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PARTYKILLLOG {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(16);
        // player_with_killing_blow: Guid
        w.write_all(&self.player_with_killing_blow.guid().to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x01f5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_with_killing_blow: Guid
        let player_with_killing_blow = Guid::read(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        Ok(Self {
            player_with_killing_blow,
            victim,
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
            // player_with_killing_blow: Guid
            let player_with_killing_blow = Guid::tokio_read(r).await?;

            // victim: Guid
            let victim = Guid::tokio_read(r).await?;

            Ok(Self {
                player_with_killing_blow,
                victim,
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
            // player_with_killing_blow: Guid
            let player_with_killing_blow = Guid::astd_read(r).await?;

            // victim: Guid
            let victim = Guid::astd_read(r).await?;

            Ok(Self {
                player_with_killing_blow,
                victim,
            })
        })
    }

}

