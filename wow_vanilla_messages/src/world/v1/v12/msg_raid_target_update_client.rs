use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RaidTargetIndex, RaidTargetIndexError};
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
pub struct MSG_RAID_TARGET_UPDATE_Client {
    pub index: MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex,
}

impl ClientMessageWrite for MSG_RAID_TARGET_UPDATE_Client {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_RAID_TARGET_UPDATE_Client {
    const OPCODE: u16 = 0x0321;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = MSG_RAID_TARGET_UPDATE_ClientError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::read(r)?;

        let index_if = match index {
            RaidTargetIndex::UNKNOWN0 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN1 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN2 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN3 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN4 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN5 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN6 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN7 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN8 => {
                // target: Guid
                let target = Guid::read(r)?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 {
                    target,
                }
            }
            RaidTargetIndex::REQUEST_ICONS => MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS,
        };

        Ok(Self {
            index: index_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        self.index.write(w)?;

        match &self.index {
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 {
                target,
            } => {
                // target: Guid
                target.write(w)?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::tokio_read(r).await?;

        let index_if = match index {
            RaidTargetIndex::UNKNOWN0 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN1 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN2 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN3 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN4 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN5 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN6 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN7 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN8 => {
                // target: Guid
                let target = Guid::tokio_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 {
                    target,
                }
            }
            RaidTargetIndex::REQUEST_ICONS => MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS,
        };

        Ok(Self {
            index: index_if,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        self.index.tokio_write(w).await?;

        match &self.index {
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 {
                target,
            } => {
                // target: Guid
                target.tokio_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::astd_read(r).await?;

        let index_if = match index {
            RaidTargetIndex::UNKNOWN0 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN1 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN2 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN3 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN4 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN5 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN6 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN7 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 {
                    target,
                }
            }
            RaidTargetIndex::UNKNOWN8 => {
                // target: Guid
                let target = Guid::astd_read(r).await?;

                MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 {
                    target,
                }
            }
            RaidTargetIndex::REQUEST_ICONS => MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS,
        };

        Ok(Self {
            index: index_if,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        self.index.astd_write(w).await?;

        match &self.index {
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 {
                target,
            } => {
                // target: Guid
                target.astd_write(w).await?;

            }
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS => {}
        }

        Ok(())
    }

}

impl VariableSized for MSG_RAID_TARGET_UPDATE_Client {
    fn size(&self) -> usize {
        self.index.size() // index: RaidTargetIndex and subfields
    }
}

impl MaximumPossibleSized for MSG_RAID_TARGET_UPDATE_Client {
    fn maximum_possible_size() -> usize {
        RaidTargetIndex::maximum_possible_size() // index: RaidTargetIndex
    }
}

#[derive(Debug)]
pub enum MSG_RAID_TARGET_UPDATE_ClientError {
    Io(std::io::Error),
    RaidTargetIndex(RaidTargetIndexError),
}

impl std::error::Error for MSG_RAID_TARGET_UPDATE_ClientError {}
impl std::fmt::Display for MSG_RAID_TARGET_UPDATE_ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidTargetIndex(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_RAID_TARGET_UPDATE_ClientError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidTargetIndexError> for MSG_RAID_TARGET_UPDATE_ClientError {
    fn from(e: RaidTargetIndexError) -> Self {
        Self::RaidTargetIndex(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex {
    UNKNOWN0 {
        target: Guid,
    },
    UNKNOWN1 {
        target: Guid,
    },
    UNKNOWN2 {
        target: Guid,
    },
    UNKNOWN3 {
        target: Guid,
    },
    UNKNOWN4 {
        target: Guid,
    },
    UNKNOWN5 {
        target: Guid,
    },
    UNKNOWN6 {
        target: Guid,
    },
    UNKNOWN7 {
        target: Guid,
    },
    UNKNOWN8 {
        target: Guid,
    },
    REQUEST_ICONS,
}

impl From<&RaidTargetIndex> for MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex {
    fn from(e: &RaidTargetIndex) -> Self {
        match &e {
            RaidTargetIndex::UNKNOWN0 => Self::UNKNOWN0 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN1 => Self::UNKNOWN1 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN2 => Self::UNKNOWN2 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN3 => Self::UNKNOWN3 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN4 => Self::UNKNOWN4 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN5 => Self::UNKNOWN5 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN6 => Self::UNKNOWN6 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN7 => Self::UNKNOWN7 {
                target: Default::default(),
            },
            RaidTargetIndex::UNKNOWN8 => Self::UNKNOWN8 {
                target: Default::default(),
            },
            RaidTargetIndex::REQUEST_ICONS => Self::REQUEST_ICONS,
        }
    }
}

impl From<&MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex> for RaidTargetIndex {
    fn from(v: &MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex) -> Self {
        match &v {
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN0 { .. } => Self::UNKNOWN0,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN1 { .. } => Self::UNKNOWN1,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN2 { .. } => Self::UNKNOWN2,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN3 { .. } => Self::UNKNOWN3,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN4 { .. } => Self::UNKNOWN4,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN5 { .. } => Self::UNKNOWN5,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN6 { .. } => Self::UNKNOWN6,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN7 { .. } => Self::UNKNOWN7,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::UNKNOWN8 { .. } => Self::UNKNOWN8,
            MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex::REQUEST_ICONS => Self::REQUEST_ICONS,
        }
    }
}

impl Default for MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex {
    fn default() -> Self {
        // First enumerator without any fields
        Self::UNKNOWN0 {
            target: Default::default(),
        }
    }
}

impl MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RaidTargetIndex = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex {
    fn size(&self) -> usize {
        match self {
            Self::UNKNOWN0  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN1  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN2  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN3  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN4  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN5  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN6  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN7  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::UNKNOWN8  {
                target,
            } => {
                1
                + 8 // target: Guid
            }
            Self::REQUEST_ICONS =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for MSG_RAID_TARGET_UPDATE_ClientRaidTargetIndex {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

