use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{FriendStatus, FriendStatusError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Friend {
    pub guid: Guid,
    pub status: FriendFriendStatus,
}

impl ReadableAndWritable for Friend {
    type Error = FriendError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // status: FriendStatus
        let status = FriendStatus::read(r)?;

        let status_if = match status {
            FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
            FriendStatus::ONLINE => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::ONLINE {
                    area,
                    level,
                    class,
                }
            }
            FriendStatus::AFK => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::AFK {
                    area,
                    level,
                    class,
                }
            }
            FriendStatus::UNKNOWN3 => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::UNKNOWN3 {
                    area,
                    level,
                    class,
                }
            }
            FriendStatus::DND => {
                // area: Area
                let area = Area::read(r)?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class = Class::read_u32_le(r)?;

                FriendFriendStatus::DND {
                    area,
                    level,
                    class,
                }
            }
        };

        Ok(Self {
            guid,
            status: status_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // status: FriendStatus
        self.status.write(w)?;

        match &self.status {
            FriendFriendStatus::OFFLINE => {}
            FriendFriendStatus::ONLINE {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
            FriendFriendStatus::AFK {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
            FriendFriendStatus::UNKNOWN3 {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
            FriendFriendStatus::DND {
                area,
                level,
                class,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                class.write_u32_le(w)?;

            }
        }

        Ok(())
    }

    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
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

            // status: FriendStatus
            let status = FriendStatus::tokio_read(r).await?;

            let status_if = match status {
                FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
                FriendStatus::ONLINE => {
                    // area: Area
                    let area = Area::tokio_read(r).await?;

                    // level: u32
                    let level = crate::util::tokio_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::tokio_read_u32_le(r).await?;

                    FriendFriendStatus::ONLINE {
                        area,
                        level,
                        class,
                    }
                }
                FriendStatus::AFK => {
                    // area: Area
                    let area = Area::tokio_read(r).await?;

                    // level: u32
                    let level = crate::util::tokio_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::tokio_read_u32_le(r).await?;

                    FriendFriendStatus::AFK {
                        area,
                        level,
                        class,
                    }
                }
                FriendStatus::UNKNOWN3 => {
                    // area: Area
                    let area = Area::tokio_read(r).await?;

                    // level: u32
                    let level = crate::util::tokio_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::tokio_read_u32_le(r).await?;

                    FriendFriendStatus::UNKNOWN3 {
                        area,
                        level,
                        class,
                    }
                }
                FriendStatus::DND => {
                    // area: Area
                    let area = Area::tokio_read(r).await?;

                    // level: u32
                    let level = crate::util::tokio_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::tokio_read_u32_le(r).await?;

                    FriendFriendStatus::DND {
                        area,
                        level,
                        class,
                    }
                }
            };

            Ok(Self {
                guid,
                status: status_if,
            })
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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

            // status: FriendStatus
            self.status.tokio_write(w).await?;

            match &self.status {
                FriendFriendStatus::OFFLINE => {}
                FriendFriendStatus::ONLINE {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.tokio_write_u32_le(w).await?;

                }
                FriendFriendStatus::AFK {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.tokio_write_u32_le(w).await?;

                }
                FriendFriendStatus::UNKNOWN3 {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.tokio_write_u32_le(w).await?;

                }
                FriendFriendStatus::DND {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.tokio_write_u32_le(w).await?;

                }
            }

            Ok(())
        })
    }

    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
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

            // status: FriendStatus
            let status = FriendStatus::astd_read(r).await?;

            let status_if = match status {
                FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
                FriendStatus::ONLINE => {
                    // area: Area
                    let area = Area::astd_read(r).await?;

                    // level: u32
                    let level = crate::util::astd_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::astd_read_u32_le(r).await?;

                    FriendFriendStatus::ONLINE {
                        area,
                        level,
                        class,
                    }
                }
                FriendStatus::AFK => {
                    // area: Area
                    let area = Area::astd_read(r).await?;

                    // level: u32
                    let level = crate::util::astd_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::astd_read_u32_le(r).await?;

                    FriendFriendStatus::AFK {
                        area,
                        level,
                        class,
                    }
                }
                FriendStatus::UNKNOWN3 => {
                    // area: Area
                    let area = Area::astd_read(r).await?;

                    // level: u32
                    let level = crate::util::astd_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::astd_read_u32_le(r).await?;

                    FriendFriendStatus::UNKNOWN3 {
                        area,
                        level,
                        class,
                    }
                }
                FriendStatus::DND => {
                    // area: Area
                    let area = Area::astd_read(r).await?;

                    // level: u32
                    let level = crate::util::astd_read_u32_le(r).await?;

                    // class: Class
                    let class = Class::astd_read_u32_le(r).await?;

                    FriendFriendStatus::DND {
                        area,
                        level,
                        class,
                    }
                }
            };

            Ok(Self {
                guid,
                status: status_if,
            })
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
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

            // status: FriendStatus
            self.status.astd_write(w).await?;

            match &self.status {
                FriendFriendStatus::OFFLINE => {}
                FriendFriendStatus::ONLINE {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.astd_write_u32_le(w).await?;

                }
                FriendFriendStatus::AFK {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.astd_write_u32_le(w).await?;

                }
                FriendFriendStatus::UNKNOWN3 {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.astd_write_u32_le(w).await?;

                }
                FriendFriendStatus::DND {
                    area,
                    level,
                    class,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    class.astd_write_u32_le(w).await?;

                }
            }

            Ok(())
        })
    }

}

impl VariableSized for Friend {
    fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + self.status.size() // status: FriendFriendStatus
    }
}

impl MaximumPossibleSized for Friend {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 10 // status: FriendFriendStatus
    }
}

#[derive(Debug)]
pub enum FriendError {
    Io(std::io::Error),
    Area(AreaError),
    Class(ClassError),
    FriendStatus(FriendStatusError),
}

impl std::error::Error for FriendError {}
impl std::fmt::Display for FriendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::FriendStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for FriendError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for FriendError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<ClassError> for FriendError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<FriendStatusError> for FriendError {
    fn from(e: FriendStatusError) -> Self {
        Self::FriendStatus(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FriendFriendStatus {
    OFFLINE,
    ONLINE {
        area: Area,
        class: Class,
        level: u32,
    },
    AFK {
        area: Area,
        class: Class,
        level: u32,
    },
    UNKNOWN3 {
        area: Area,
        class: Class,
        level: u32,
    },
    DND {
        area: Area,
        class: Class,
        level: u32,
    },
}

impl From<&FriendStatus> for FriendFriendStatus {
    fn from(e: &FriendStatus) -> Self {
        match &e {
            FriendStatus::OFFLINE => Self::OFFLINE,
            FriendStatus::ONLINE => Self::ONLINE {
                area: Default::default(),
                class: Default::default(),
                level: Default::default(),
            },
            FriendStatus::AFK => Self::AFK {
                area: Default::default(),
                class: Default::default(),
                level: Default::default(),
            },
            FriendStatus::UNKNOWN3 => Self::UNKNOWN3 {
                area: Default::default(),
                class: Default::default(),
                level: Default::default(),
            },
            FriendStatus::DND => Self::DND {
                area: Default::default(),
                class: Default::default(),
                level: Default::default(),
            },
        }
    }
}

impl From<&FriendFriendStatus> for FriendStatus {
    fn from(v: &FriendFriendStatus) -> Self {
        match &v {
            FriendFriendStatus::OFFLINE => Self::OFFLINE,
            FriendFriendStatus::ONLINE { .. } => Self::ONLINE,
            FriendFriendStatus::AFK { .. } => Self::AFK,
            FriendFriendStatus::UNKNOWN3 { .. } => Self::UNKNOWN3,
            FriendFriendStatus::DND { .. } => Self::DND,
        }
    }
}

impl Default for FriendFriendStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::OFFLINE
    }
}

impl FriendFriendStatus {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write_u16_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write_u16_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write_u32_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write_u32_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write_u64_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: FriendStatus = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for FriendFriendStatus {
    fn size(&self) -> usize {
        match self {
            Self::OFFLINE => {
                1
            }
            Self::ONLINE {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
            Self::AFK {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
            Self::UNKNOWN3 {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
            Self::DND {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
        }
    }
}

impl MaximumPossibleSized for FriendFriendStatus {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

