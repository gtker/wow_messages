use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidTargetUpdate, RaidTargetUpdateError};
use crate::world::v1::v12::{RaidTargetUpdateType, RaidTargetUpdateTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_RAID_TARGET_UPDATE_Server {
    pub update_type: MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType,
}

impl ServerMessageWrite for MSG_RAID_TARGET_UPDATE_Server {}

impl MessageBody for MSG_RAID_TARGET_UPDATE_Server {
    const OPCODE: u16 = 0x0321;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = MSG_RAID_TARGET_UPDATE_ServerError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // update_type: RaidTargetUpdateType
        let update_type: RaidTargetUpdateType = crate::util::read_u8_le(r)?.try_into()?;

        let update_type_if = match update_type {
            RaidTargetUpdateType::PARTIAL => {
                // raid_target: RaidTargetUpdate
                let raid_target = RaidTargetUpdate::read(r)?;

                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                    raid_target,
                }
            }
            RaidTargetUpdateType::FULL => {
                // raid_targets: RaidTargetUpdate[8]
                let mut raid_targets = Vec::with_capacity(8 as usize);
                for i in 0..8 {
                    raid_targets.push(RaidTargetUpdate::read(r)?);
                }
                let raid_targets = raid_targets.try_into().unwrap();

                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                    raid_targets,
                }
            }
        };

        Ok(Self {
            update_type: update_type_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // update_type: RaidTargetUpdateType
        w.write_all(&(self.update_type.as_int() as u8).to_le_bytes())?;

        match &self.update_type {
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                raid_target,
            } => {
                // raid_target: RaidTargetUpdate
                raid_target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                raid_targets,
            } => {
                // raid_targets: RaidTargetUpdate[8]
                for i in raid_targets.iter() {
                    i.write(w)?;
                }

            }
        }

        Ok(())
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
            // update_type: RaidTargetUpdateType
            let update_type: RaidTargetUpdateType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let update_type_if = match update_type {
                RaidTargetUpdateType::PARTIAL => {
                    // raid_target: RaidTargetUpdate
                    let raid_target = RaidTargetUpdate::tokio_read(r).await?;

                    MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                        raid_target,
                    }
                }
                RaidTargetUpdateType::FULL => {
                    // raid_targets: RaidTargetUpdate[8]
                    let mut raid_targets = Vec::with_capacity(8 as usize);
                    for i in 0..8 {
                        raid_targets.push(RaidTargetUpdate::tokio_read(r).await?);
                    }
                    let raid_targets = raid_targets.try_into().unwrap();

                    MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                        raid_targets,
                    }
                }
            };

            Ok(Self {
                update_type: update_type_if,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // update_type: RaidTargetUpdateType
            w.write_all(&(self.update_type.as_int() as u8).to_le_bytes()).await?;

            match &self.update_type {
                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                    raid_target,
                } => {
                    // raid_target: RaidTargetUpdate
                    raid_target.tokio_write(w).await?;

                }
                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                    raid_targets,
                } => {
                    // raid_targets: RaidTargetUpdate[8]
                    for i in raid_targets.iter() {
                        i.tokio_write(w).await?;
                    }

                }
            }

            Ok(())
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
            // update_type: RaidTargetUpdateType
            let update_type: RaidTargetUpdateType = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let update_type_if = match update_type {
                RaidTargetUpdateType::PARTIAL => {
                    // raid_target: RaidTargetUpdate
                    let raid_target = RaidTargetUpdate::astd_read(r).await?;

                    MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                        raid_target,
                    }
                }
                RaidTargetUpdateType::FULL => {
                    // raid_targets: RaidTargetUpdate[8]
                    let mut raid_targets = Vec::with_capacity(8 as usize);
                    for i in 0..8 {
                        raid_targets.push(RaidTargetUpdate::astd_read(r).await?);
                    }
                    let raid_targets = raid_targets.try_into().unwrap();

                    MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                        raid_targets,
                    }
                }
            };

            Ok(Self {
                update_type: update_type_if,
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
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // update_type: RaidTargetUpdateType
            w.write_all(&(self.update_type.as_int() as u8).to_le_bytes()).await?;

            match &self.update_type {
                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::PARTIAL {
                    raid_target,
                } => {
                    // raid_target: RaidTargetUpdate
                    raid_target.astd_write(w).await?;

                }
                MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType::FULL {
                    raid_targets,
                } => {
                    // raid_targets: RaidTargetUpdate[8]
                    for i in raid_targets.iter() {
                        i.astd_write(w).await?;
                    }

                }
            }

            Ok(())
        })
    }

}

impl MSG_RAID_TARGET_UPDATE_Server {
    pub fn size(&self) -> usize {
        0
        + self.update_type.size() // update_type: MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType
    }
}

#[derive(Debug)]
pub enum MSG_RAID_TARGET_UPDATE_ServerError {
    Io(std::io::Error),
    RaidTargetUpdate(RaidTargetUpdateError),
    RaidTargetUpdateType(RaidTargetUpdateTypeError),
}

impl std::error::Error for MSG_RAID_TARGET_UPDATE_ServerError {}
impl std::fmt::Display for MSG_RAID_TARGET_UPDATE_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidTargetUpdate(i) => i.fmt(f),
            Self::RaidTargetUpdateType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_RAID_TARGET_UPDATE_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidTargetUpdateError> for MSG_RAID_TARGET_UPDATE_ServerError {
    fn from(e: RaidTargetUpdateError) -> Self {
        Self::RaidTargetUpdate(e)
    }
}

impl From<RaidTargetUpdateTypeError> for MSG_RAID_TARGET_UPDATE_ServerError {
    fn from(e: RaidTargetUpdateTypeError) -> Self {
        Self::RaidTargetUpdateType(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    PARTIAL {
        raid_target: RaidTargetUpdate,
    },
    FULL {
        raid_targets: [RaidTargetUpdate; 8],
    },
}

impl Default for MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::PARTIAL {
            raid_target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PARTIAL { .. } => 0,
            Self::FULL { .. } => 1,
        }
    }

}

impl MSG_RAID_TARGET_UPDATE_ServerRaidTargetUpdateType {
    pub fn size(&self) -> usize {
        match self {
            Self::PARTIAL {
                raid_target,
            } => {
                1
                + RaidTargetUpdate::size() // raid_target: RaidTargetUpdate
            }
            Self::FULL {
                raid_targets,
            } => {
                1
                + 8 * RaidTargetUpdate::size() // raid_targets: RaidTargetUpdate[8]
            }
        }
    }
}

