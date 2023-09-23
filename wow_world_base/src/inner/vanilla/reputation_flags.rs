/// Used in `Faction.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/reputation_flags.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/reputation_flags.wowm#L2):
/// ```text
/// flag ReputationFlags : u8 {
///     VISIBLE_TO_CLIENT = 0x01;
///     ENABLE_AT_WAR = 0x02;
///     HIDE_IN_CLIENT = 0x04;
///     FORCE_HIDE_IN_CLIENT = 0x08;
///     FORCE_AT_PEACE = 0x10;
///     FACTION_INACTIVE = 0x20;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ReputationFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl ReputationFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_visible_to_client() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "VISIBLE_TO_CLIENT").unwrap();
            first = false;
        }
        if self.is_enable_at_war() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ENABLE_AT_WAR").unwrap();
            first = false;
        }
        if self.is_hide_in_client() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HIDE_IN_CLIENT").unwrap();
            first = false;
        }
        if self.is_force_hide_in_client() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FORCE_HIDE_IN_CLIENT").unwrap();
            first = false;
        }
        if self.is_force_at_peace() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FORCE_AT_PEACE").unwrap();
            first = false;
        }
        if self.is_faction_inactive() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FACTION_INACTIVE").unwrap();
            first = false;
        }
        s
    }

}

impl ReputationFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const VISIBLE_TO_CLIENT: u8 = 0x01;
    pub const ENABLE_AT_WAR: u8 = 0x02;
    pub const HIDE_IN_CLIENT: u8 = 0x04;
    pub const FORCE_HIDE_IN_CLIENT: u8 = 0x08;
    pub const FORCE_AT_PEACE: u8 = 0x10;
    pub const FACTION_INACTIVE: u8 = 0x20;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::VISIBLE_TO_CLIENT
                | Self::ENABLE_AT_WAR
                | Self::HIDE_IN_CLIENT
                | Self::FORCE_HIDE_IN_CLIENT
                | Self::FORCE_AT_PEACE
                | Self::FACTION_INACTIVE
        }
    }

    pub const fn is_visible_to_client(&self) -> bool {
        (self.inner & Self::VISIBLE_TO_CLIENT) != 0
    }

    pub const fn new_visible_to_client() -> Self {
        Self { inner: Self::VISIBLE_TO_CLIENT }
    }

    pub fn set_visible_to_client(&mut self) -> Self {
        self.inner |= Self::VISIBLE_TO_CLIENT;
        *self
    }

    pub fn clear_visible_to_client(&mut self) -> Self {
        self.inner &= Self::VISIBLE_TO_CLIENT.reverse_bits();
        *self
    }

    pub const fn is_enable_at_war(&self) -> bool {
        (self.inner & Self::ENABLE_AT_WAR) != 0
    }

    pub const fn new_enable_at_war() -> Self {
        Self { inner: Self::ENABLE_AT_WAR }
    }

    pub fn set_enable_at_war(&mut self) -> Self {
        self.inner |= Self::ENABLE_AT_WAR;
        *self
    }

    pub fn clear_enable_at_war(&mut self) -> Self {
        self.inner &= Self::ENABLE_AT_WAR.reverse_bits();
        *self
    }

    pub const fn is_hide_in_client(&self) -> bool {
        (self.inner & Self::HIDE_IN_CLIENT) != 0
    }

    pub const fn new_hide_in_client() -> Self {
        Self { inner: Self::HIDE_IN_CLIENT }
    }

    pub fn set_hide_in_client(&mut self) -> Self {
        self.inner |= Self::HIDE_IN_CLIENT;
        *self
    }

    pub fn clear_hide_in_client(&mut self) -> Self {
        self.inner &= Self::HIDE_IN_CLIENT.reverse_bits();
        *self
    }

    pub const fn is_force_hide_in_client(&self) -> bool {
        (self.inner & Self::FORCE_HIDE_IN_CLIENT) != 0
    }

    pub const fn new_force_hide_in_client() -> Self {
        Self { inner: Self::FORCE_HIDE_IN_CLIENT }
    }

    pub fn set_force_hide_in_client(&mut self) -> Self {
        self.inner |= Self::FORCE_HIDE_IN_CLIENT;
        *self
    }

    pub fn clear_force_hide_in_client(&mut self) -> Self {
        self.inner &= Self::FORCE_HIDE_IN_CLIENT.reverse_bits();
        *self
    }

    pub const fn is_force_at_peace(&self) -> bool {
        (self.inner & Self::FORCE_AT_PEACE) != 0
    }

    pub const fn new_force_at_peace() -> Self {
        Self { inner: Self::FORCE_AT_PEACE }
    }

    pub fn set_force_at_peace(&mut self) -> Self {
        self.inner |= Self::FORCE_AT_PEACE;
        *self
    }

    pub fn clear_force_at_peace(&mut self) -> Self {
        self.inner &= Self::FORCE_AT_PEACE.reverse_bits();
        *self
    }

    pub const fn is_faction_inactive(&self) -> bool {
        (self.inner & Self::FACTION_INACTIVE) != 0
    }

    pub const fn new_faction_inactive() -> Self {
        Self { inner: Self::FACTION_INACTIVE }
    }

    pub fn set_faction_inactive(&mut self) -> Self {
        self.inner |= Self::FACTION_INACTIVE;
        *self
    }

    pub fn clear_faction_inactive(&mut self) -> Self {
        self.inner &= Self::FACTION_INACTIVE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for ReputationFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for ReputationFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for ReputationFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for ReputationFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for ReputationFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for ReputationFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for ReputationFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for ReputationFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for ReputationFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for ReputationFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for ReputationFlags {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for ReputationFlags {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for ReputationFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for ReputationFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for ReputationFlags {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for ReputationFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for ReputationFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for ReputationFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for ReputationFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

