/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L19):
/// ```text
/// flag LfgUpdateFlag : u32 {
///     NONE = 0x00;
///     CHARACTER_INFO = 0x01;
///     COMMENT = 0x02;
///     GROUP_LEADER = 0x04;
///     GROUP_GUID = 0x08;
///     ROLES = 0x10;
///     AREA = 0x20;
///     STATUS = 0x40;
///     BOUND = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LfgUpdateFlag {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl LfgUpdateFlag {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_character_info() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CHARACTER_INFO").unwrap();
            first = false;
        }
        if self.is_comment() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "COMMENT").unwrap();
            first = false;
        }
        if self.is_group_leader() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "GROUP_LEADER").unwrap();
            first = false;
        }
        if self.is_group_guid() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "GROUP_GUID").unwrap();
            first = false;
        }
        if self.is_roles() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ROLES").unwrap();
            first = false;
        }
        if self.is_area() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AREA").unwrap();
            first = false;
        }
        if self.is_status() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "STATUS").unwrap();
            first = false;
        }
        if self.is_bound() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "BOUND").unwrap();
            first = false;
        }
        s
    }

}

impl LfgUpdateFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const CHARACTER_INFO: u32 = 0x01;
    pub const COMMENT: u32 = 0x02;
    pub const GROUP_LEADER: u32 = 0x04;
    pub const GROUP_GUID: u32 = 0x08;
    pub const ROLES: u32 = 0x10;
    pub const AREA: u32 = 0x20;
    pub const STATUS: u32 = 0x40;
    pub const BOUND: u32 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::CHARACTER_INFO
                | Self::COMMENT
                | Self::GROUP_LEADER
                | Self::GROUP_GUID
                | Self::ROLES
                | Self::AREA
                | Self::STATUS
                | Self::BOUND
        }
    }

    pub const fn is_character_info(&self) -> bool {
        (self.inner & Self::CHARACTER_INFO) != 0
    }

    pub const fn new_character_info() -> Self {
        Self { inner: Self::CHARACTER_INFO }
    }

    pub fn set_character_info(&mut self) -> Self {
        self.inner |= Self::CHARACTER_INFO;
        *self
    }

    pub fn clear_character_info(&mut self) -> Self {
        self.inner &= Self::CHARACTER_INFO.reverse_bits();
        *self
    }

    pub const fn is_comment(&self) -> bool {
        (self.inner & Self::COMMENT) != 0
    }

    pub const fn new_comment() -> Self {
        Self { inner: Self::COMMENT }
    }

    pub fn set_comment(&mut self) -> Self {
        self.inner |= Self::COMMENT;
        *self
    }

    pub fn clear_comment(&mut self) -> Self {
        self.inner &= Self::COMMENT.reverse_bits();
        *self
    }

    pub const fn is_group_leader(&self) -> bool {
        (self.inner & Self::GROUP_LEADER) != 0
    }

    pub const fn new_group_leader() -> Self {
        Self { inner: Self::GROUP_LEADER }
    }

    pub fn set_group_leader(&mut self) -> Self {
        self.inner |= Self::GROUP_LEADER;
        *self
    }

    pub fn clear_group_leader(&mut self) -> Self {
        self.inner &= Self::GROUP_LEADER.reverse_bits();
        *self
    }

    pub const fn is_group_guid(&self) -> bool {
        (self.inner & Self::GROUP_GUID) != 0
    }

    pub const fn new_group_guid() -> Self {
        Self { inner: Self::GROUP_GUID }
    }

    pub fn set_group_guid(&mut self) -> Self {
        self.inner |= Self::GROUP_GUID;
        *self
    }

    pub fn clear_group_guid(&mut self) -> Self {
        self.inner &= Self::GROUP_GUID.reverse_bits();
        *self
    }

    pub const fn is_roles(&self) -> bool {
        (self.inner & Self::ROLES) != 0
    }

    pub const fn new_roles() -> Self {
        Self { inner: Self::ROLES }
    }

    pub fn set_roles(&mut self) -> Self {
        self.inner |= Self::ROLES;
        *self
    }

    pub fn clear_roles(&mut self) -> Self {
        self.inner &= Self::ROLES.reverse_bits();
        *self
    }

    pub const fn is_area(&self) -> bool {
        (self.inner & Self::AREA) != 0
    }

    pub const fn new_area() -> Self {
        Self { inner: Self::AREA }
    }

    pub fn set_area(&mut self) -> Self {
        self.inner |= Self::AREA;
        *self
    }

    pub fn clear_area(&mut self) -> Self {
        self.inner &= Self::AREA.reverse_bits();
        *self
    }

    pub const fn is_status(&self) -> bool {
        (self.inner & Self::STATUS) != 0
    }

    pub const fn new_status() -> Self {
        Self { inner: Self::STATUS }
    }

    pub fn set_status(&mut self) -> Self {
        self.inner |= Self::STATUS;
        *self
    }

    pub fn clear_status(&mut self) -> Self {
        self.inner &= Self::STATUS.reverse_bits();
        *self
    }

    pub const fn is_bound(&self) -> bool {
        (self.inner & Self::BOUND) != 0
    }

    pub const fn new_bound() -> Self {
        Self { inner: Self::BOUND }
    }

    pub fn set_bound(&mut self) -> Self {
        self.inner |= Self::BOUND;
        *self
    }

    pub fn clear_bound(&mut self) -> Self {
        self.inner &= Self::BOUND.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for LfgUpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for LfgUpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for LfgUpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for LfgUpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for LfgUpdateFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for LfgUpdateFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for LfgUpdateFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for LfgUpdateFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for LfgUpdateFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for LfgUpdateFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for LfgUpdateFlag {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for LfgUpdateFlag {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for LfgUpdateFlag {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for LfgUpdateFlag {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for LfgUpdateFlag {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for LfgUpdateFlag {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for LfgUpdateFlag {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for LfgUpdateFlag {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for LfgUpdateFlag {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

