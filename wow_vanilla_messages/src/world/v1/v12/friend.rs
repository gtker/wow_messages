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
                    class,
                    level,
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
                    class,
                    level,
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
                    class,
                    level,
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
                    class,
                    level,
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
                class,
                level,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                crate::util::write_u32_le(w, class.as_int() as u32)?;

            }
            FriendFriendStatus::AFK {
                area,
                class,
                level,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                crate::util::write_u32_le(w, class.as_int() as u32)?;

            }
            FriendFriendStatus::UNKNOWN3 {
                area,
                class,
                level,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                crate::util::write_u32_le(w, class.as_int() as u32)?;

            }
            FriendFriendStatus::DND {
                area,
                class,
                level,
            } => {
                // area: Area
                area.write(w)?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                crate::util::write_u32_le(w, class.as_int() as u32)?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
                        class,
                        level,
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
                        class,
                        level,
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
                        class,
                        level,
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
                        class,
                        level,
                    }
                }
            };

            Ok(Self {
                guid,
                status: status_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
                    class,
                    level,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::tokio_write_u32_le(w, class.as_int() as u32).await?;

                }
                FriendFriendStatus::AFK {
                    area,
                    class,
                    level,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::tokio_write_u32_le(w, class.as_int() as u32).await?;

                }
                FriendFriendStatus::UNKNOWN3 {
                    area,
                    class,
                    level,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::tokio_write_u32_le(w, class.as_int() as u32).await?;

                }
                FriendFriendStatus::DND {
                    area,
                    class,
                    level,
                } => {
                    // area: Area
                    area.tokio_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::tokio_write_u32_le(w, class.as_int() as u32).await?;

                }
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
                        class,
                        level,
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
                        class,
                        level,
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
                        class,
                        level,
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
                        class,
                        level,
                    }
                }
            };

            Ok(Self {
                guid,
                status: status_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
                    class,
                    level,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::astd_write_u32_le(w, class.as_int() as u32).await?;

                }
                FriendFriendStatus::AFK {
                    area,
                    class,
                    level,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::astd_write_u32_le(w, class.as_int() as u32).await?;

                }
                FriendFriendStatus::UNKNOWN3 {
                    area,
                    class,
                    level,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::astd_write_u32_le(w, class.as_int() as u32).await?;

                }
                FriendFriendStatus::DND {
                    area,
                    class,
                    level,
                } => {
                    // area: Area
                    area.astd_write(w).await?;

                    // level: u32
                    w.write_all(&level.to_le_bytes()).await?;

                    // class: Class
                    crate::util::astd_write_u32_le(w, class.as_int() as u32).await?;

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

impl Default for FriendFriendStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::OFFLINE
    }
}

impl FriendFriendStatus {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    pub(crate) fn as_int(&self) -> u8 {
        match self {
            Self::OFFLINE => 0,
            Self::ONLINE{ .. } => 1,
            Self::AFK{ .. } => 2,
            Self::UNKNOWN3{ .. } => 3,
            Self::DND{ .. } => 4,
        }
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
        10
    }
}

