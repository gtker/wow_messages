/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L1):
/// ```text
/// flag RelationType : u32 {
///     NONE = 0x00;
///     FRIEND = 0x01;
///     IGNORED = 0x02;
///     MUTED = 0x04;
///     RECRUITAFRIEND = 0x08;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RelationType {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl RelationType {
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
        if self.is_friend() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FRIEND").unwrap();
            first = false;
        }
        if self.is_ignored() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORED").unwrap();
            first = false;
        }
        if self.is_muted() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MUTED").unwrap();
            first = false;
        }
        if self.is_recruitafriend() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RECRUITAFRIEND").unwrap();
            first = false;
        }
        s
    }

}

impl RelationType {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const FRIEND: u32 = 0x01;
    pub const IGNORED: u32 = 0x02;
    pub const MUTED: u32 = 0x04;
    pub const RECRUITAFRIEND: u32 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::FRIEND
                | Self::IGNORED
                | Self::MUTED
                | Self::RECRUITAFRIEND
        }
    }

    pub const fn is_friend(&self) -> bool {
        (self.inner & Self::FRIEND) != 0
    }

    pub const fn new_friend() -> Self {
        Self { inner: Self::FRIEND }
    }

    pub fn set_friend(&mut self) -> Self {
        self.inner |= Self::FRIEND;
        *self
    }

    pub fn clear_friend(&mut self) -> Self {
        self.inner &= Self::FRIEND.reverse_bits();
        *self
    }

    pub const fn is_ignored(&self) -> bool {
        (self.inner & Self::IGNORED) != 0
    }

    pub const fn new_ignored() -> Self {
        Self { inner: Self::IGNORED }
    }

    pub fn set_ignored(&mut self) -> Self {
        self.inner |= Self::IGNORED;
        *self
    }

    pub fn clear_ignored(&mut self) -> Self {
        self.inner &= Self::IGNORED.reverse_bits();
        *self
    }

    pub const fn is_muted(&self) -> bool {
        (self.inner & Self::MUTED) != 0
    }

    pub const fn new_muted() -> Self {
        Self { inner: Self::MUTED }
    }

    pub fn set_muted(&mut self) -> Self {
        self.inner |= Self::MUTED;
        *self
    }

    pub fn clear_muted(&mut self) -> Self {
        self.inner &= Self::MUTED.reverse_bits();
        *self
    }

    pub const fn is_recruitafriend(&self) -> bool {
        (self.inner & Self::RECRUITAFRIEND) != 0
    }

    pub const fn new_recruitafriend() -> Self {
        Self { inner: Self::RECRUITAFRIEND }
    }

    pub fn set_recruitafriend(&mut self) -> Self {
        self.inner |= Self::RECRUITAFRIEND;
        *self
    }

    pub fn clear_recruitafriend(&mut self) -> Self {
        self.inner &= Self::RECRUITAFRIEND.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for RelationType {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for RelationType {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for RelationType {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for RelationType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for RelationType {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for RelationType {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

