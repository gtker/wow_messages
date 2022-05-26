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
pub struct SMSG_TAXINODE_STATUS {
    pub guid: Guid,
    pub taxi_mask_node_known: u8,
}

impl SMSG_TAXINODE_STATUS {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // taxi_mask_node_known: u8
        w.write_all(&self.taxi_mask_node_known.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TAXINODE_STATUS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // taxi_mask_node_known: u8
        w.write_all(&self.taxi_mask_node_known.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ab;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // taxi_mask_node_known: u8
        let taxi_mask_node_known = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            taxi_mask_node_known,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // taxi_mask_node_known: u8
            let taxi_mask_node_known = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                taxi_mask_node_known,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // taxi_mask_node_known: u8
            let taxi_mask_node_known = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                taxi_mask_node_known,
            })
        })
    }

}

