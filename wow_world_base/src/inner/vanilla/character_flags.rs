/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm#L3):
/// ```text
/// flag CharacterFlags : u32 {
///     NONE = 0x00;
///     LOCKED_FOR_TRANSFER = 0x04;
///     HIDE_HELM = 0x400;
///     HIDE_CLOAK = 0x800;
///     GHOST = 0x2000;
///     RENAME = 0x4000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct CharacterFlags {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl CharacterFlags {
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
        if self.is_locked_for_transfer() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "LOCKED_FOR_TRANSFER").unwrap();
            first = false;
        }
        if self.is_hide_helm() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HIDE_HELM").unwrap();
            first = false;
        }
        if self.is_hide_cloak() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HIDE_CLOAK").unwrap();
            first = false;
        }
        if self.is_ghost() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "GHOST").unwrap();
            first = false;
        }
        if self.is_rename() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RENAME").unwrap();
            first = false;
        }
        s
    }

}

impl CharacterFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const LOCKED_FOR_TRANSFER: u32 = 0x04;
    pub const HIDE_HELM: u32 = 0x400;
    pub const HIDE_CLOAK: u32 = 0x800;
    pub const GHOST: u32 = 0x2000;
    pub const RENAME: u32 = 0x4000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::LOCKED_FOR_TRANSFER
                | Self::HIDE_HELM
                | Self::HIDE_CLOAK
                | Self::GHOST
                | Self::RENAME
        }
    }

    pub const fn is_locked_for_transfer(&self) -> bool {
        (self.inner & Self::LOCKED_FOR_TRANSFER) != 0
    }

    pub const fn new_locked_for_transfer() -> Self {
        Self { inner: Self::LOCKED_FOR_TRANSFER }
    }

    pub fn set_locked_for_transfer(&mut self) -> Self {
        self.inner |= Self::LOCKED_FOR_TRANSFER;
        *self
    }

    pub fn clear_locked_for_transfer(&mut self) -> Self {
        self.inner &= Self::LOCKED_FOR_TRANSFER.reverse_bits();
        *self
    }

    pub const fn is_hide_helm(&self) -> bool {
        (self.inner & Self::HIDE_HELM) != 0
    }

    pub const fn new_hide_helm() -> Self {
        Self { inner: Self::HIDE_HELM }
    }

    pub fn set_hide_helm(&mut self) -> Self {
        self.inner |= Self::HIDE_HELM;
        *self
    }

    pub fn clear_hide_helm(&mut self) -> Self {
        self.inner &= Self::HIDE_HELM.reverse_bits();
        *self
    }

    pub const fn is_hide_cloak(&self) -> bool {
        (self.inner & Self::HIDE_CLOAK) != 0
    }

    pub const fn new_hide_cloak() -> Self {
        Self { inner: Self::HIDE_CLOAK }
    }

    pub fn set_hide_cloak(&mut self) -> Self {
        self.inner |= Self::HIDE_CLOAK;
        *self
    }

    pub fn clear_hide_cloak(&mut self) -> Self {
        self.inner &= Self::HIDE_CLOAK.reverse_bits();
        *self
    }

    pub const fn is_ghost(&self) -> bool {
        (self.inner & Self::GHOST) != 0
    }

    pub const fn new_ghost() -> Self {
        Self { inner: Self::GHOST }
    }

    pub fn set_ghost(&mut self) -> Self {
        self.inner |= Self::GHOST;
        *self
    }

    pub fn clear_ghost(&mut self) -> Self {
        self.inner &= Self::GHOST.reverse_bits();
        *self
    }

    pub const fn is_rename(&self) -> bool {
        (self.inner & Self::RENAME) != 0
    }

    pub const fn new_rename() -> Self {
        Self { inner: Self::RENAME }
    }

    pub fn set_rename(&mut self) -> Self {
        self.inner |= Self::RENAME;
        *self
    }

    pub fn clear_rename(&mut self) -> Self {
        self.inner &= Self::RENAME.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for CharacterFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for CharacterFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for CharacterFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for CharacterFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for CharacterFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for CharacterFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for CharacterFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for CharacterFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for CharacterFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for CharacterFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

