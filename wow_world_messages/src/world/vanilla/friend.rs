use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::vanilla::{
    Area, Class, FriendStatus,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_list.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_list.wowm#L11):
/// ```text
/// struct Friend {
///     Guid guid;
///     FriendStatus status;
///     if (status != OFFLINE) {
///         Area area;
///         Level32 level;
///         (u32)Class class;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Friend {
    pub guid: Guid,
    pub status: Friend_FriendStatus,
}

impl Friend {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: FriendStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        match &self.status {
            Friend_FriendStatus::Online {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int().to_le_bytes()))?;

                // level: Level32
                w.write_all(&u32::from(level.as_int()).to_le_bytes())?;

                // class: Class
                w.write_all(&u32::from(class.as_int()).to_le_bytes())?;

            }
            Friend_FriendStatus::Afk {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int().to_le_bytes()))?;

                // level: Level32
                w.write_all(&u32::from(level.as_int()).to_le_bytes())?;

                // class: Class
                w.write_all(&u32::from(class.as_int()).to_le_bytes())?;

            }
            Friend_FriendStatus::Unknown3 {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int().to_le_bytes()))?;

                // level: Level32
                w.write_all(&u32::from(level.as_int()).to_le_bytes())?;

                // class: Class
                w.write_all(&u32::from(class.as_int()).to_le_bytes())?;

            }
            Friend_FriendStatus::Dnd {
                area,
                class,
                level,
            } => {
                // area: Area
                w.write_all(&(area.as_int().to_le_bytes()))?;

                // level: Level32
                w.write_all(&u32::from(level.as_int()).to_le_bytes())?;

                // class: Class
                w.write_all(&u32::from(class.as_int()).to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl Friend {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // status: FriendStatus
        let status = crate::util::read_u8_le(&mut r)?.try_into()?;

        let status_if = match status {
            FriendStatus::Offline => Friend_FriendStatus::Offline,
            FriendStatus::Online => {
                // area: Area
                let area = crate::util::read_u32_le(&mut r)?.try_into()?;

                // level: Level32
                let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

                // class: Class
                let class = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                Friend_FriendStatus::Online {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::Afk => {
                // area: Area
                let area = crate::util::read_u32_le(&mut r)?.try_into()?;

                // level: Level32
                let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

                // class: Class
                let class = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                Friend_FriendStatus::Afk {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::Unknown3 => {
                // area: Area
                let area = crate::util::read_u32_le(&mut r)?.try_into()?;

                // level: Level32
                let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

                // class: Class
                let class = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                Friend_FriendStatus::Unknown3 {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::Dnd => {
                // area: Area
                let area = crate::util::read_u32_le(&mut r)?.try_into()?;

                // level: Level32
                let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

                // class: Class
                let class = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                Friend_FriendStatus::Dnd {
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
    pub(crate) const fn size(&self) -> usize {
        8 // guid: Guid
        + self.status.size() // status: Friend_FriendStatus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Friend_FriendStatus {
    Offline,
    Online {
        area: Area,
        class: Class,
        level: Level,
    },
    Afk {
        area: Area,
        class: Class,
        level: Level,
    },
    Unknown3 {
        area: Area,
        class: Class,
        level: Level,
    },
    Dnd {
        area: Area,
        class: Class,
        level: Level,
    },
}

impl Default for Friend_FriendStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Offline
    }
}

impl Friend_FriendStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Offline => 0,
            Self::Online { .. } => 1,
            Self::Afk { .. } => 2,
            Self::Unknown3 { .. } => 3,
            Self::Dnd { .. } => 4,
        }
    }

}

impl std::fmt::Display for Friend_FriendStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Offline => f.write_str("Offline"),
            Self::Online{ .. } => f.write_str("Online"),
            Self::Afk{ .. } => f.write_str("Afk"),
            Self::Unknown3{ .. } => f.write_str("Unknown3"),
            Self::Dnd{ .. } => f.write_str("Dnd"),
        }
    }
}

impl Friend_FriendStatus {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Online {
                ..
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: Level32
            }
            Self::Afk {
                ..
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: Level32
            }
            Self::Unknown3 {
                ..
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: Level32
            }
            Self::Dnd {
                ..
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: Level32
            }
            _ => 1,
        }
    }
}

