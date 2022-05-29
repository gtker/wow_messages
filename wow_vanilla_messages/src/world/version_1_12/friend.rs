use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::Area;
use crate::world::version_1_12::Class;
use crate::world::version_1_12::FriendStatus;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Friend {
    pub guid: Guid,
    pub status: Friend_FriendStatus,
}

impl Friend {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: FriendStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        match &self.status {
            Friend_FriendStatus::OFFLINE => {}
            Friend_FriendStatus::ONLINE {
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
            Friend_FriendStatus::AFK {
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
            Friend_FriendStatus::UNKNOWN3 {
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
            Friend_FriendStatus::DND {
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
}

impl Friend {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // status: FriendStatus
        let status: FriendStatus = crate::util::read_u8_le(r)?.try_into()?;

        let status_if = match status {
            FriendStatus::OFFLINE => Friend_FriendStatus::OFFLINE,
            FriendStatus::ONLINE => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                Friend_FriendStatus::ONLINE {
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

                Friend_FriendStatus::AFK {
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

                Friend_FriendStatus::UNKNOWN3 {
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

                Friend_FriendStatus::DND {
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

}

impl Friend {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.status.size() // status: Friend_FriendStatus
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Friend_FriendStatus {
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

impl Default for Friend_FriendStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::OFFLINE
    }
}

impl Friend_FriendStatus {
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

impl Friend_FriendStatus {
    pub(crate) fn size(&self) -> usize {
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

