use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_PET_SET_ACTION {
    pub guid: Guid,
    pub position1: u32,
    pub data1: u32,
    pub extra: Option<CMSG_PET_SET_ACTION_extra>,
}

impl ClientMessageWrite for CMSG_PET_SET_ACTION {}

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

            Some(CMSG_PET_SET_ACTION_extra {
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
        // guid: Guid
        self.guid.write(w)?;

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

        Ok(())
    }

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

                Some(CMSG_PET_SET_ACTION_extra {
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

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // position1: u32
            w.write_all(&self.position1.to_le_bytes()).await?;

            // data1: u32
            w.write_all(&self.data1.to_le_bytes()).await?;

            // optional extra
            if let Some(v) = &self.extra {
                // position2: u32
                w.write_all(&v.position2.to_le_bytes()).await?;

                // data2: u32
                w.write_all(&v.data2.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

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

                Some(CMSG_PET_SET_ACTION_extra {
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

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guid: Guid
            self.guid.astd_write(w).await?;

            // position1: u32
            w.write_all(&self.position1.to_le_bytes()).await?;

            // data1: u32
            w.write_all(&self.data1.to_le_bytes()).await?;

            // optional extra
            if let Some(v) = &self.extra {
                // position2: u32
                w.write_all(&v.position2.to_le_bytes()).await?;

                // data2: u32
                w.write_all(&v.data2.to_le_bytes()).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for CMSG_PET_SET_ACTION {
    fn size(&self) -> usize {
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

impl MaximumPossibleSized for CMSG_PET_SET_ACTION {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 4 // position1: u32
        + 4 // data1: u32
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_PET_SET_ACTION_extra {
    pub position2: u32,
    pub data2: u32,
}

impl CMSG_PET_SET_ACTION_extra {
    pub fn size(&self) -> usize {
        4 // position2: u32
        + 4 // data2: u32
    }
}

