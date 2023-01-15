use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::Area;
use crate::world::tbc::Class;
use crate::world::tbc::FriendStatus;
use crate::world::tbc::RelationType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L21):
/// ```text
/// struct Relation {
///     Guid guid;
///     RelationType relation_mask;
///     CString note;
///     if (relation_mask & FRIEND) {
///         FriendStatus status;
///         if (status == ONLINE) {
///             Area area;
///             u32 level;
///             (u32)Class class;
///         }
///     }
/// }
/// ```
pub struct Relation {
    pub guid: Guid,
    pub relation_mask: Relation_RelationType,
    pub note: String,
}

impl Relation {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // relation_mask: RelationType
        w.write_all(&(self.relation_mask.as_int() as u32).to_le_bytes())?;

        // note: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.note.as_bytes().iter().rev().next(), Some(&0_u8), "String `note` must not be null-terminated.");
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        if let Some(if_statement) = &self.relation_mask.friend {
            // status: FriendStatus
            w.write_all(&(if_statement.status.as_int() as u8).to_le_bytes())?;

            match &if_statement.status {
                Relation_FriendStatus::Offline => {}
                Relation_FriendStatus::Online {
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
                Relation_FriendStatus::Afk => {}
                Relation_FriendStatus::Unknown3 => {}
                Relation_FriendStatus::Dnd => {}
            }

        }

        Ok(())
    }
}

impl Relation {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // relation_mask: RelationType
        let relation_mask = RelationType::new(crate::util::read_u32_le(r)?);

        // note: CString
        let note = crate::util::read_c_string_to_vec(r)?;
        let note = String::from_utf8(note)?;

        let relation_mask_FRIEND = if relation_mask.is_FRIEND() {
            // status: FriendStatus
            let status: FriendStatus = crate::util::read_u8_le(r)?.try_into()?;

            let status_if = match status {
                FriendStatus::Offline => Relation_FriendStatus::Offline,
                FriendStatus::Online => {
                    // area: Area
                    let area: Area = crate::util::read_u32_le(r)?.try_into()?;

                    // level: u32
                    let level = crate::util::read_u32_le(r)?;

                    // class: Class
                    let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

                    Relation_FriendStatus::Online {
                        area,
                        class,
                        level,
                    }
                }
                FriendStatus::Afk => Relation_FriendStatus::Afk,
                FriendStatus::Unknown3 => Relation_FriendStatus::Unknown3,
                FriendStatus::Dnd => Relation_FriendStatus::Dnd,
            };

            Some(Relation_RelationType_Friend {
                status: status_if,
            })
        }
        else {
            None
        };

        let relation_mask = Relation_RelationType {
            inner: relation_mask.as_int(),
            friend: relation_mask_FRIEND,
        };

        Ok(Self {
            guid,
            relation_mask,
            note,
        })
    }

}

impl Relation {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.relation_mask.size() // relation_mask: Relation_RelationType
        + self.note.len() + 1 // note: CString
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Relation_FriendStatus {
    Offline,
    Online {
        area: Area,
        class: Class,
        level: u32,
    },
    Afk,
    Unknown3,
    Dnd,
}

impl Default for Relation_FriendStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Offline
    }
}

impl Relation_FriendStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Offline => 0,
            Self::Online { .. } => 1,
            Self::Afk => 2,
            Self::Unknown3 => 3,
            Self::Dnd => 4,
        }
    }

}

impl Relation_FriendStatus {
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
            Self::Afk => {
                1
            }
            Self::Unknown3 => {
                1
            }
            Self::Dnd => {
                1
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Relation_RelationType {
    inner: u32,
    friend: Option<Relation_RelationType_Friend>,
}

impl Relation_RelationType {
    pub const fn new(inner: u32, friend: Option<Relation_RelationType_Friend>,) -> Self {
        Self {
            inner,
            friend, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            friend: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.friend.is_none()
    }

    pub const fn new_FRIEND(friend: Relation_RelationType_Friend) -> Self {
        Self {
            inner: RelationType::FRIEND,
            friend: Some(friend),
        }
    }

    pub fn set_FRIEND(mut self, friend: Relation_RelationType_Friend) -> Self {
        self.inner |= RelationType::FRIEND;
        self.friend = Some(friend);
        self
    }

    pub const fn get_FRIEND(&self) -> Option<&Relation_RelationType_Friend> {
        self.friend.as_ref()
    }

    pub fn clear_FRIEND(mut self) -> Self {
        self.inner &= RelationType::FRIEND.reverse_bits();
        self.friend = None;
        self
    }

    pub const fn new_IGNORED() -> Self {
        Self {
            inner: RelationType::IGNORED,
            friend: None,
        }
    }

    pub fn set_IGNORED(mut self) -> Self {
        self.inner |= RelationType::IGNORED;
        self
    }

    pub const fn get_IGNORED(&self) -> bool {
        (self.inner & RelationType::IGNORED) != 0
    }

    pub fn clear_IGNORED(mut self) -> Self {
        self.inner &= RelationType::IGNORED.reverse_bits();
        self
    }

    pub const fn new_MUTED() -> Self {
        Self {
            inner: RelationType::MUTED,
            friend: None,
        }
    }

    pub fn set_MUTED(mut self) -> Self {
        self.inner |= RelationType::MUTED;
        self
    }

    pub const fn get_MUTED(&self) -> bool {
        (self.inner & RelationType::MUTED) != 0
    }

    pub fn clear_MUTED(mut self) -> Self {
        self.inner &= RelationType::MUTED.reverse_bits();
        self
    }

    pub const fn new_RECRUITAFRIEND() -> Self {
        Self {
            inner: RelationType::RECRUITAFRIEND,
            friend: None,
        }
    }

    pub fn set_RECRUITAFRIEND(mut self) -> Self {
        self.inner |= RelationType::RECRUITAFRIEND;
        self
    }

    pub const fn get_RECRUITAFRIEND(&self) -> bool {
        (self.inner & RelationType::RECRUITAFRIEND) != 0
    }

    pub fn clear_RECRUITAFRIEND(mut self) -> Self {
        self.inner &= RelationType::RECRUITAFRIEND.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl Relation_RelationType {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.friend {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Relation_RelationType_Friend {
    pub status: Relation_FriendStatus,
}

impl Relation_RelationType_Friend {
    pub(crate) fn size(&self) -> usize {
        self.status.size() // status: Relation_FriendStatus
    }
}

