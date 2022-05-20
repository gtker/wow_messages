use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{FriendStatus, FriendStatusError};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Friend {
    pub guid: Guid,
    pub status: FriendFriendStatus,
}

impl Friend {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // status: FriendStatus
        let status: FriendStatus = crate::util::read_u8_le(r)?.try_into()?;

        let status_if = match status {
            FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
            FriendStatus::ONLINE => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                FriendFriendStatus::ONLINE {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::AFK => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                FriendFriendStatus::AFK {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::UNKNOWN3 => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                FriendFriendStatus::UNKNOWN3 {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::DND => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

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
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: FriendStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        match &self.status {
            FriendFriendStatus::OFFLINE => {}
            FriendFriendStatus::ONLINE {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes())?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes())?;

            }
            FriendFriendStatus::AFK {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes())?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes())?;

            }
            FriendFriendStatus::UNKNOWN3 {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes())?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes())?;

            }
            FriendFriendStatus::DND {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes())?;

                // level: u32
                w.write_all(&level.to_le_bytes())?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes())?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, FriendError> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // status: FriendStatus
        let status: FriendStatus = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        let status_if = match status {
            FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
            FriendStatus::ONLINE => {
                // area: Area
                let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::tokio_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

                FriendFriendStatus::ONLINE {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::AFK => {
                // area: Area
                let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::tokio_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

                FriendFriendStatus::AFK {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::UNKNOWN3 => {
                // area: Area
                let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::tokio_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

                FriendFriendStatus::UNKNOWN3 {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::DND => {
                // area: Area
                let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::tokio_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes()).await?;

        // status: FriendStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes()).await?;

        match &self.status {
            FriendFriendStatus::OFFLINE => {}
            FriendFriendStatus::ONLINE {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
            FriendFriendStatus::AFK {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
            FriendFriendStatus::UNKNOWN3 {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
            FriendFriendStatus::DND {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, FriendError> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // status: FriendStatus
        let status: FriendStatus = crate::util::astd_read_u8_le(r).await?.try_into()?;

        let status_if = match status {
            FriendStatus::OFFLINE => FriendFriendStatus::OFFLINE,
            FriendStatus::ONLINE => {
                // area: Area
                let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::astd_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

                FriendFriendStatus::ONLINE {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::AFK => {
                // area: Area
                let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::astd_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

                FriendFriendStatus::AFK {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::UNKNOWN3 => {
                // area: Area
                let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::astd_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

                FriendFriendStatus::UNKNOWN3 {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::DND => {
                // area: Area
                let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

                // level: u32
                let level = crate::util::astd_read_u32_le(r).await?;

                // class: Class
                let class: Class = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes()).await?;

        // status: FriendStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes()).await?;

        match &self.status {
            FriendFriendStatus::OFFLINE => {}
            FriendFriendStatus::ONLINE {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
            FriendFriendStatus::AFK {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
            FriendFriendStatus::UNKNOWN3 {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
            FriendFriendStatus::DND {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int() as u32).to_le_bytes()).await?;

                // level: u32
                w.write_all(&level.to_le_bytes()).await?;

                // class: Class
                w.write_all(&(class.as_int() as u32).to_le_bytes()).await?;

            }
        }

        Ok(())
    }

}

impl Friend {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + self.status.size() // status: FriendFriendStatus
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
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::OFFLINE => 0,
            Self::ONLINE { .. } => 1,
            Self::AFK { .. } => 2,
            Self::UNKNOWN3 { .. } => 3,
            Self::DND { .. } => 4,
        }
    }

}

impl FriendFriendStatus {
    pub fn size(&self) -> usize {
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

