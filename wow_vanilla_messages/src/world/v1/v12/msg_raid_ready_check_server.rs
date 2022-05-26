use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_RAID_READY_CHECK_Server {
    pub state_check: Option<MSG_RAID_READY_CHECK_Serverstate_check>,
}

impl ServerMessage for MSG_RAID_READY_CHECK_Server {}

impl MSG_RAID_READY_CHECK_Server {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // optional state_check
        if let Some(v) = &self.state_check {
            // guid: Guid
            w.write_all(&v.guid.guid().to_le_bytes())?;

            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(w)
    }
}

impl MessageBody for MSG_RAID_READY_CHECK_Server {
    const OPCODE: u16 = 0x0322;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // optional state_check
        let current_size = {
            0
        };
        let state_check = if current_size < body_size as usize {
            // guid: Guid
            let guid = Guid::read(r)?;

            // state: u8
            let state = crate::util::read_u8_le(r)?;

            Some(MSG_RAID_READY_CHECK_Serverstate_check {
                guid,
                state,
            })
        } else {
            None
        };

        Ok(Self {
            state_check,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // optional state_check
            let current_size = {
                0
            };
            let state_check = if current_size < body_size as usize {
                // guid: Guid
                let guid = Guid::tokio_read(r).await?;

                // state: u8
                let state = crate::util::tokio_read_u8_le(r).await?;

                Some(MSG_RAID_READY_CHECK_Serverstate_check {
                    guid,
                    state,
                })
            } else {
                None
            };

            Ok(Self {
                state_check,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // optional state_check
            let current_size = {
                0
            };
            let state_check = if current_size < body_size as usize {
                // guid: Guid
                let guid = Guid::astd_read(r).await?;

                // state: u8
                let state = crate::util::astd_read_u8_le(r).await?;

                Some(MSG_RAID_READY_CHECK_Serverstate_check {
                    guid,
                    state,
                })
            } else {
                None
            };

            Ok(Self {
                state_check,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl MSG_RAID_READY_CHECK_Server {
    pub fn size(&self) -> usize {
        0
        + if let Some(state_check) = &self.state_check {
            0
            + 8 // guid: Guid
            + 1 // state: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSG_RAID_READY_CHECK_Serverstate_check {
    pub guid: Guid,
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_Serverstate_check {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // state: u8
    }

}

