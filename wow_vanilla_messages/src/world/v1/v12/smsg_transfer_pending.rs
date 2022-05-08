use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_TRANSFER_PENDING {
    pub map: Map,
    pub has_transport: Option<SMSG_TRANSFER_PENDING_has_transport>,
}

impl ServerMessageWrite for SMSG_TRANSFER_PENDING {}

impl MessageBody for SMSG_TRANSFER_PENDING {
    const OPCODE: u16 = 0x003f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_TRANSFER_PENDINGError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // optional has_transport
        let current_size = {
            0
            + 4 // map: Map
        };
        let has_transport = if current_size < body_size as usize {
            // transport: u32
            let transport = crate::util::read_u32_le(r)?;

            // transport_map: Map
            let transport_map = Map::read(r)?;

            Some(SMSG_TRANSFER_PENDING_has_transport {
                transport,
                transport_map,
            })
        } else {
            None
        };

        Ok(Self {
            map,
            has_transport,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // optional has_transport
        if let Some(v) = &self.has_transport {
            // transport: u32
            w.write_all(&v.transport.to_le_bytes())?;

            // transport_map: Map
            v.transport_map.write(w)?;

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
            // map: Map
            let map = Map::tokio_read(r).await?;

            // optional has_transport
            let current_size = {
                0
                + 4 // map: Map
            };
            let has_transport = if current_size < body_size as usize {
                // transport: u32
                let transport = crate::util::tokio_read_u32_le(r).await?;

                // transport_map: Map
                let transport_map = Map::tokio_read(r).await?;

                Some(SMSG_TRANSFER_PENDING_has_transport {
                    transport,
                    transport_map,
                })
            } else {
                None
            };

            Ok(Self {
                map,
                has_transport,
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
            // map: Map
            self.map.tokio_write(w).await?;

            // optional has_transport
            if let Some(v) = &self.has_transport {
                // transport: u32
                w.write_all(&v.transport.to_le_bytes()).await?;

                // transport_map: Map
                v.transport_map.tokio_write(w).await?;

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
            // map: Map
            let map = Map::astd_read(r).await?;

            // optional has_transport
            let current_size = {
                0
                + 4 // map: Map
            };
            let has_transport = if current_size < body_size as usize {
                // transport: u32
                let transport = crate::util::astd_read_u32_le(r).await?;

                // transport_map: Map
                let transport_map = Map::astd_read(r).await?;

                Some(SMSG_TRANSFER_PENDING_has_transport {
                    transport,
                    transport_map,
                })
            } else {
                None
            };

            Ok(Self {
                map,
                has_transport,
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
            // map: Map
            self.map.astd_write(w).await?;

            // optional has_transport
            if let Some(v) = &self.has_transport {
                // transport: u32
                w.write_all(&v.transport.to_le_bytes()).await?;

                // transport_map: Map
                v.transport_map.astd_write(w).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_TRANSFER_PENDING {
    fn size(&self) -> usize {
        0
        + 4 // map: Map
        + if let Some(has_transport) = &self.has_transport {
            0
            + 4 // transport: u32
            + 4 // transport_map: Map
        } else {
            0
        }
    }
}

impl MaximumPossibleSized for SMSG_TRANSFER_PENDING {
    fn maximum_possible_size() -> usize {
        0
        + 4 // map: Map
    }
}

#[derive(Debug)]
pub enum SMSG_TRANSFER_PENDINGError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for SMSG_TRANSFER_PENDINGError {}
impl std::fmt::Display for SMSG_TRANSFER_PENDINGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRANSFER_PENDINGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_TRANSFER_PENDINGError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_TRANSFER_PENDING_has_transport {
    pub transport: u32,
    pub transport_map: Map,
}

impl SMSG_TRANSFER_PENDING_has_transport {
    pub fn size(&self) -> usize {
        4 // transport: u32
        + 4 // transport_map: Map
    }
}

