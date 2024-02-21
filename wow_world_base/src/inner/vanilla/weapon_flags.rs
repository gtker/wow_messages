/// Used in `AnimationData.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/weapon_flags.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/weapon_flags.wowm#L2):
/// ```text
/// flag WeaponFlags : u8 {
///     WEAPON_NOT_AFFECTED_BY_ANIMATION = 0x00;
///     SHEATHE_WEAPONS_AUTOMATICALLY = 0x04;
///     SHEATHE_WEAPONS_AUTOMATICALLY_2 = 0x10;
///     UNSHEATHE_WEAPONS = 0x20;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct WeaponFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl WeaponFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "WEAPON_NOT_AFFECTED_BY_ANIMATION").unwrap();
            first = false;
        }
        if self.is_sheathe_weapons_automatically() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SHEATHE_WEAPONS_AUTOMATICALLY").unwrap();
            first = false;
        }
        if self.is_sheathe_weapons_automatically_2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SHEATHE_WEAPONS_AUTOMATICALLY_2").unwrap();
            first = false;
        }
        if self.is_unsheathe_weapons() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNSHEATHE_WEAPONS").unwrap();
            first = false;
        }
        s
    }

}

impl WeaponFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const WEAPON_NOT_AFFECTED_BY_ANIMATION: u8 = 0x00;
    pub const SHEATHE_WEAPONS_AUTOMATICALLY: u8 = 0x04;
    pub const SHEATHE_WEAPONS_AUTOMATICALLY_2: u8 = 0x10;
    pub const UNSHEATHE_WEAPONS: u8 = 0x20;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::WEAPON_NOT_AFFECTED_BY_ANIMATION
                | Self::SHEATHE_WEAPONS_AUTOMATICALLY
                | Self::SHEATHE_WEAPONS_AUTOMATICALLY_2
                | Self::UNSHEATHE_WEAPONS
        }
    }

    pub const fn is_sheathe_weapons_automatically(&self) -> bool {
        (self.inner & Self::SHEATHE_WEAPONS_AUTOMATICALLY) != 0
    }

    pub const fn new_sheathe_weapons_automatically() -> Self {
        Self { inner: Self::SHEATHE_WEAPONS_AUTOMATICALLY }
    }

    pub fn set_sheathe_weapons_automatically(&mut self) -> Self {
        self.inner |= Self::SHEATHE_WEAPONS_AUTOMATICALLY;
        *self
    }

    pub fn clear_sheathe_weapons_automatically(&mut self) -> Self {
        self.inner &= Self::SHEATHE_WEAPONS_AUTOMATICALLY.reverse_bits();
        *self
    }

    pub const fn is_sheathe_weapons_automatically_2(&self) -> bool {
        (self.inner & Self::SHEATHE_WEAPONS_AUTOMATICALLY_2) != 0
    }

    pub const fn new_sheathe_weapons_automatically_2() -> Self {
        Self { inner: Self::SHEATHE_WEAPONS_AUTOMATICALLY_2 }
    }

    pub fn set_sheathe_weapons_automatically_2(&mut self) -> Self {
        self.inner |= Self::SHEATHE_WEAPONS_AUTOMATICALLY_2;
        *self
    }

    pub fn clear_sheathe_weapons_automatically_2(&mut self) -> Self {
        self.inner &= Self::SHEATHE_WEAPONS_AUTOMATICALLY_2.reverse_bits();
        *self
    }

    pub const fn is_unsheathe_weapons(&self) -> bool {
        (self.inner & Self::UNSHEATHE_WEAPONS) != 0
    }

    pub const fn new_unsheathe_weapons() -> Self {
        Self { inner: Self::UNSHEATHE_WEAPONS }
    }

    pub fn set_unsheathe_weapons(&mut self) -> Self {
        self.inner |= Self::UNSHEATHE_WEAPONS;
        *self
    }

    pub fn clear_unsheathe_weapons(&mut self) -> Self {
        self.inner &= Self::UNSHEATHE_WEAPONS.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for WeaponFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for WeaponFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for WeaponFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for WeaponFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for WeaponFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for WeaponFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for WeaponFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for WeaponFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for WeaponFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for WeaponFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for WeaponFlags {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for WeaponFlags {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for WeaponFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for WeaponFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for WeaponFlags {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for WeaponFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for WeaponFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for WeaponFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for WeaponFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

