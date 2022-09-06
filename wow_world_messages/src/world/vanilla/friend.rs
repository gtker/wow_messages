use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::Area;
use crate::world::vanilla::Class;
use crate::world::vanilla::FriendStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_list.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_list.wowm#L11):
/// ```text
/// struct Friend {
///     Guid guid;
///     FriendStatus status;
///     if (status != OFFLINE) {
///         Area area;
///         u32 level;
///         Class class;
///     }
/// }
/// ```
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
            Friend_FriendStatus::Offline => {}
            Friend_FriendStatus::Online {
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
            Friend_FriendStatus::Afk {
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
            Friend_FriendStatus::Unknown3 {
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
            Friend_FriendStatus::Dnd {
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
            FriendStatus::Offline => Friend_FriendStatus::Offline,
            FriendStatus::Online => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                Friend_FriendStatus::Online {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::Afk => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                Friend_FriendStatus::Afk {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::Unknown3 => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                Friend_FriendStatus::Unknown3 {
                    area,
                    class,
                    level,
                }
            }
            FriendStatus::Dnd => {
                // area: Area
                let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                // level: u32
                let level = crate::util::read_u32_le(r)?;

                // class: Class
                let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

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
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.status.size() // status: Friend_FriendStatus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Friend_FriendStatus {
    Offline,
    Online {
        area: Area,
        class: Class,
        level: u32,
    },
    Afk {
        area: Area,
        class: Class,
        level: u32,
    },
    Unknown3 {
        area: Area,
        class: Class,
        level: u32,
    },
    Dnd {
        area: Area,
        class: Class,
        level: u32,
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

impl Friend_FriendStatus {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Offline => {
                1
            }
            Self::Online {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
            Self::Afk {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
            Self::Unknown3 {
                area,
                class,
                level,
            } => {
                1
                + 4 // area: Area
                + 4 // class: Class
                + 4 // level: u32
            }
            Self::Dnd {
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

