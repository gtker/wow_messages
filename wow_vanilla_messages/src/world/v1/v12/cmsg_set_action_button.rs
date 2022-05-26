use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SET_ACTION_BUTTON {
    pub button: u8,
    pub action_type: u32,
}

impl CMSG_SET_ACTION_BUTTON {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action_type: u32
        w.write_all(&self.action_type.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_SET_ACTION_BUTTON {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // button: u8
        w.write_all(&self.button.to_le_bytes())?;

        // action_type: u32
        w.write_all(&self.action_type.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0128;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // button: u8
        let button = crate::util::read_u8_le(r)?;

        // action_type: u32
        let action_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            button,
            action_type,
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
            // button: u8
            let button = crate::util::tokio_read_u8_le(r).await?;

            // action_type: u32
            let action_type = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                button,
                action_type,
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
            // button: u8
            let button = crate::util::astd_read_u8_le(r).await?;

            // action_type: u32
            let action_type = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                button,
                action_type,
            })
        })
    }

}

