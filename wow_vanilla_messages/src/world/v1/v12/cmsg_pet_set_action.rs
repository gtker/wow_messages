use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_PET_SET_ACTION {
    pub guid: Guid,
    pub position1: u32,
    pub data1: u32,
    pub extra: Option<CMSG_PET_SET_ACTIONextra>,
}

impl ClientMessageWrite for CMSG_PET_SET_ACTION {}

impl CMSG_PET_SET_ACTION {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // position1: u32
        w.write_all(&self.position1.to_le_bytes())?;

        // data1: u32
        w.write_all(&self.data1.to_le_bytes())?;

        // optional extra
        if let Some(v) = &self.extra {
            // position2: u32
            w.write_all(&v.position2.to_le_bytes())?;

            // data2: u32
            w.write_all(&v.data2.to_le_bytes())?;

        }

        Ok(w)
    }
}

impl MessageBody for CMSG_PET_SET_ACTION {
    const OPCODE: u16 = 0x0174;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // position1: u32
        let position1 = crate::util::read_u32_le(r)?;

        // data1: u32
        let data1 = crate::util::read_u32_le(r)?;

        // optional extra
        let current_size = {
            0
            + 8 // guid: Guid
            + 4 // position1: u32
            + 4 // data1: u32
        };
        let extra = if current_size < body_size as usize {
            // position2: u32
            let position2 = crate::util::read_u32_le(r)?;

            // data2: u32
            let data2 = crate::util::read_u32_le(r)?;

            Some(CMSG_PET_SET_ACTIONextra {
                position2,
                data2,
            })
        } else {
            None
        };

        Ok(Self {
            guid,
            position1,
            data1,
            extra,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // position1: u32
            let position1 = crate::util::tokio_read_u32_le(r).await?;

            // data1: u32
            let data1 = crate::util::tokio_read_u32_le(r).await?;

            // optional extra
            let current_size = {
                0
                + 8 // guid: Guid
                + 4 // position1: u32
                + 4 // data1: u32
            };
            let extra = if current_size < body_size as usize {
                // position2: u32
                let position2 = crate::util::tokio_read_u32_le(r).await?;

                // data2: u32
                let data2 = crate::util::tokio_read_u32_le(r).await?;

                Some(CMSG_PET_SET_ACTIONextra {
                    position2,
                    data2,
                })
            } else {
                None
            };

            Ok(Self {
                guid,
                position1,
                data1,
                extra,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // position1: u32
            let position1 = crate::util::astd_read_u32_le(r).await?;

            // data1: u32
            let data1 = crate::util::astd_read_u32_le(r).await?;

            // optional extra
            let current_size = {
                0
                + 8 // guid: Guid
                + 4 // position1: u32
                + 4 // data1: u32
            };
            let extra = if current_size < body_size as usize {
                // position2: u32
                let position2 = crate::util::astd_read_u32_le(r).await?;

                // data2: u32
                let data2 = crate::util::astd_read_u32_le(r).await?;

                Some(CMSG_PET_SET_ACTIONextra {
                    position2,
                    data2,
                })
            } else {
                None
            };

            Ok(Self {
                guid,
                position1,
                data1,
                extra,
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

impl CMSG_PET_SET_ACTION {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // position1: u32
        + 4 // data1: u32
        + if let Some(extra) = &self.extra {
            0
            + 4 // position2: u32
            + 4 // data2: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_PET_SET_ACTIONextra {
    pub position2: u32,
    pub data2: u32,
}

impl CMSG_PET_SET_ACTIONextra {
    pub(crate) fn size(&self) -> usize {
        4 // position2: u32
        + 4 // data2: u32
    }

}

