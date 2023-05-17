use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::LfgUpdateFlag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L33):
/// ```text
/// struct LfgListGroup {
///     Guid group;
///     LfgUpdateFlag flags;
///     if (flags & COMMENT) {
///         CString comment;
///     }
///     if (flags & ROLES) {
///         u8[3] roles;
///     }
///     Guid instance;
///     u32 encounter_mask;
/// }
/// ```
pub struct LfgListGroup {
    pub group: Guid,
    pub flags: LfgListGroup_LfgUpdateFlag,
    pub instance: Guid,
    pub encounter_mask: u32,
}

impl LfgListGroup {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // group: Guid
        w.write_all(&self.group.guid().to_le_bytes())?;

        // flags: LfgUpdateFlag
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

        if let Some(if_statement) = &self.flags.comment {
            // comment: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(if_statement.comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
            w.write_all(if_statement.comment.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.flags.roles {
            // roles: u8[3]
            for i in if_statement.roles.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        // instance: Guid
        w.write_all(&self.instance.guid().to_le_bytes())?;

        // encounter_mask: u32
        w.write_all(&self.encounter_mask.to_le_bytes())?;

        Ok(())
    }
}

impl LfgListGroup {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // group: Guid
        let group = Guid::read(&mut r)?;

        // flags: LfgUpdateFlag
        let flags = LfgUpdateFlag::new(crate::util::read_u32_le(&mut r)?);

        let flags_COMMENT = if flags.is_comment() {
            // comment: CString
            let comment = {
                let comment = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(comment)?
            };

            Some(LfgListGroup_LfgUpdateFlag_Comment {
                comment,
            })
        }
        else {
            None
        };

        let flags_ROLES = if flags.is_roles() {
            // roles: u8[3]
            let roles = {
                let mut roles = [0_u8; 3];
                r.read_exact(&mut roles)?;
                roles
            };

            Some(LfgListGroup_LfgUpdateFlag_Roles {
                roles,
            })
        }
        else {
            None
        };

        // instance: Guid
        let instance = Guid::read(&mut r)?;

        // encounter_mask: u32
        let encounter_mask = crate::util::read_u32_le(&mut r)?;

        let flags = LfgListGroup_LfgUpdateFlag {
            inner: flags.as_int(),
            comment: flags_COMMENT,
            roles: flags_ROLES,
        };

        Ok(Self {
            group,
            flags,
            instance,
            encounter_mask,
        })
    }

}

impl LfgListGroup {
    pub(crate) fn size(&self) -> usize {
        8 // group: Guid
        + self.flags.size() // flags: LfgListGroup_LfgUpdateFlag
        + 8 // instance: Guid
        + 4 // encounter_mask: u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListGroup_LfgUpdateFlag {
    inner: u32,
    comment: Option<LfgListGroup_LfgUpdateFlag_Comment>,
    roles: Option<LfgListGroup_LfgUpdateFlag_Roles>,
}

impl LfgListGroup_LfgUpdateFlag {
    pub const fn new(inner: u32, comment: Option<LfgListGroup_LfgUpdateFlag_Comment>,roles: Option<LfgListGroup_LfgUpdateFlag_Roles>,) -> Self {
        Self {
            inner,
            comment, 
            roles, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            comment: None,
            roles: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.comment.is_none()
        && self.roles.is_none()
    }

    pub const fn new_character_info() -> Self {
        Self {
            inner: LfgUpdateFlag::CHARACTER_INFO,
            comment: None,
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_character_info(mut self) -> Self {
        self.inner |= LfgUpdateFlag::CHARACTER_INFO;
        self
    }

    pub const fn get_character_info(&self) -> bool {
        (self.inner & LfgUpdateFlag::CHARACTER_INFO) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_character_info(mut self) -> Self {
        self.inner &= LfgUpdateFlag::CHARACTER_INFO.reverse_bits();
        self
    }

    pub const fn new_comment(comment: LfgListGroup_LfgUpdateFlag_Comment) -> Self {
        Self {
            inner: LfgUpdateFlag::COMMENT,
            comment: Some(comment),
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_comment(mut self, comment: LfgListGroup_LfgUpdateFlag_Comment) -> Self {
        self.inner |= LfgUpdateFlag::COMMENT;
        self.comment = Some(comment);
        self
    }

    pub const fn get_comment(&self) -> Option<&LfgListGroup_LfgUpdateFlag_Comment> {
        self.comment.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_comment(mut self) -> Self {
        self.inner &= LfgUpdateFlag::COMMENT.reverse_bits();
        self.comment = None;
        self
    }

    pub const fn new_group_leader() -> Self {
        Self {
            inner: LfgUpdateFlag::GROUP_LEADER,
            comment: None,
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_group_leader(mut self) -> Self {
        self.inner |= LfgUpdateFlag::GROUP_LEADER;
        self
    }

    pub const fn get_group_leader(&self) -> bool {
        (self.inner & LfgUpdateFlag::GROUP_LEADER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_group_leader(mut self) -> Self {
        self.inner &= LfgUpdateFlag::GROUP_LEADER.reverse_bits();
        self
    }

    pub const fn new_group_guid() -> Self {
        Self {
            inner: LfgUpdateFlag::GROUP_GUID,
            comment: None,
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_group_guid(mut self) -> Self {
        self.inner |= LfgUpdateFlag::GROUP_GUID;
        self
    }

    pub const fn get_group_guid(&self) -> bool {
        (self.inner & LfgUpdateFlag::GROUP_GUID) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_group_guid(mut self) -> Self {
        self.inner &= LfgUpdateFlag::GROUP_GUID.reverse_bits();
        self
    }

    pub const fn new_roles(roles: LfgListGroup_LfgUpdateFlag_Roles) -> Self {
        Self {
            inner: LfgUpdateFlag::ROLES,
            comment: None,
            roles: Some(roles),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_roles(mut self, roles: LfgListGroup_LfgUpdateFlag_Roles) -> Self {
        self.inner |= LfgUpdateFlag::ROLES;
        self.roles = Some(roles);
        self
    }

    pub const fn get_roles(&self) -> Option<&LfgListGroup_LfgUpdateFlag_Roles> {
        self.roles.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_roles(mut self) -> Self {
        self.inner &= LfgUpdateFlag::ROLES.reverse_bits();
        self.roles = None;
        self
    }

    pub const fn new_area() -> Self {
        Self {
            inner: LfgUpdateFlag::AREA,
            comment: None,
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_area(mut self) -> Self {
        self.inner |= LfgUpdateFlag::AREA;
        self
    }

    pub const fn get_area(&self) -> bool {
        (self.inner & LfgUpdateFlag::AREA) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_area(mut self) -> Self {
        self.inner &= LfgUpdateFlag::AREA.reverse_bits();
        self
    }

    pub const fn new_status() -> Self {
        Self {
            inner: LfgUpdateFlag::STATUS,
            comment: None,
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_status(mut self) -> Self {
        self.inner |= LfgUpdateFlag::STATUS;
        self
    }

    pub const fn get_status(&self) -> bool {
        (self.inner & LfgUpdateFlag::STATUS) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_status(mut self) -> Self {
        self.inner &= LfgUpdateFlag::STATUS.reverse_bits();
        self
    }

    pub const fn new_bound() -> Self {
        Self {
            inner: LfgUpdateFlag::BOUND,
            comment: None,
            roles: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_bound(mut self) -> Self {
        self.inner |= LfgUpdateFlag::BOUND;
        self
    }

    pub const fn get_bound(&self) -> bool {
        (self.inner & LfgUpdateFlag::BOUND) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_bound(mut self) -> Self {
        self.inner &= LfgUpdateFlag::BOUND.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl LfgListGroup_LfgUpdateFlag {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.comment {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.roles {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListGroup_LfgUpdateFlag_Comment {
    pub comment: String,
}

impl LfgListGroup_LfgUpdateFlag_Comment {
    pub(crate) fn size(&self) -> usize {
        self.comment.len() + 1 // comment: CString
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListGroup_LfgUpdateFlag_Roles {
    pub roles: [u8; 3],
}

impl LfgListGroup_LfgUpdateFlag_Roles {
    pub(crate) const fn size(&self) -> usize {
        3 // roles: u8[3]
    }
}

