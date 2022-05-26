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
pub struct CMSG_GROUP_ASSISTANT_LEADER {
    pub guid: Guid,
    pub set_assistant: u8,
}

impl CMSG_GROUP_ASSISTANT_LEADER {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // set_assistant: u8
        w.write_all(&self.set_assistant.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_GROUP_ASSISTANT_LEADER {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(9);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // set_assistant: u8
        w.write_all(&self.set_assistant.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x028f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // set_assistant: u8
        let set_assistant = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            set_assistant,
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

            // set_assistant: u8
            let set_assistant = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                set_assistant,
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

            // set_assistant: u8
            let set_assistant = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                set_assistant,
            })
        })
    }

}

