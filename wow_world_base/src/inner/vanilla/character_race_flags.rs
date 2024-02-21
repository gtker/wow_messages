/// Used in `ChrRaces.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/character_race_flags.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/character_race_flags.wowm#L2):
/// ```text
/// flag CharacterRaceFlags : u8 {
///     NONE = 0x00;
///     NOT_PLAYABLE = 0x01;
///     BARE_FEET = 0x02;
///     CAN_CURRENT_FORM_MOUNT = 0x04;
///     UNKNOWN2 = 0x08;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CharacterRaceFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl CharacterRaceFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_not_playable() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NOT_PLAYABLE").unwrap();
            first = false;
        }
        if self.is_bare_feet() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BARE_FEET").unwrap();
            first = false;
        }
        if self.is_can_current_form_mount() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CAN_CURRENT_FORM_MOUNT").unwrap();
            first = false;
        }
        if self.is_unknown2() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNKNOWN2").unwrap();
            first = false;
        }
        s
    }

}

impl CharacterRaceFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const NOT_PLAYABLE: u8 = 0x01;
    pub const BARE_FEET: u8 = 0x02;
    pub const CAN_CURRENT_FORM_MOUNT: u8 = 0x04;
    pub const UNKNOWN2: u8 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::NOT_PLAYABLE
                | Self::BARE_FEET
                | Self::CAN_CURRENT_FORM_MOUNT
                | Self::UNKNOWN2
        }
    }

    pub const fn is_not_playable(&self) -> bool {
        (self.inner & Self::NOT_PLAYABLE) != 0
    }

    pub const fn new_not_playable() -> Self {
        Self { inner: Self::NOT_PLAYABLE }
    }

    pub fn set_not_playable(&mut self) -> Self {
        self.inner |= Self::NOT_PLAYABLE;
        *self
    }

    pub fn clear_not_playable(&mut self) -> Self {
        self.inner &= Self::NOT_PLAYABLE.reverse_bits();
        *self
    }

    pub const fn is_bare_feet(&self) -> bool {
        (self.inner & Self::BARE_FEET) != 0
    }

    pub const fn new_bare_feet() -> Self {
        Self { inner: Self::BARE_FEET }
    }

    pub fn set_bare_feet(&mut self) -> Self {
        self.inner |= Self::BARE_FEET;
        *self
    }

    pub fn clear_bare_feet(&mut self) -> Self {
        self.inner &= Self::BARE_FEET.reverse_bits();
        *self
    }

    pub const fn is_can_current_form_mount(&self) -> bool {
        (self.inner & Self::CAN_CURRENT_FORM_MOUNT) != 0
    }

    pub const fn new_can_current_form_mount() -> Self {
        Self { inner: Self::CAN_CURRENT_FORM_MOUNT }
    }

    pub fn set_can_current_form_mount(&mut self) -> Self {
        self.inner |= Self::CAN_CURRENT_FORM_MOUNT;
        *self
    }

    pub fn clear_can_current_form_mount(&mut self) -> Self {
        self.inner &= Self::CAN_CURRENT_FORM_MOUNT.reverse_bits();
        *self
    }

    pub const fn is_unknown2(&self) -> bool {
        (self.inner & Self::UNKNOWN2) != 0
    }

    pub const fn new_unknown2() -> Self {
        Self { inner: Self::UNKNOWN2 }
    }

    pub fn set_unknown2(&mut self) -> Self {
        self.inner |= Self::UNKNOWN2;
        *self
    }

    pub fn clear_unknown2(&mut self) -> Self {
        self.inner &= Self::UNKNOWN2.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for CharacterRaceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for CharacterRaceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for CharacterRaceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for CharacterRaceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for CharacterRaceFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for CharacterRaceFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for CharacterRaceFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for CharacterRaceFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for CharacterRaceFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for CharacterRaceFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u8> for CharacterRaceFlags {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for CharacterRaceFlags {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for CharacterRaceFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for CharacterRaceFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for CharacterRaceFlags {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for CharacterRaceFlags {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for CharacterRaceFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for CharacterRaceFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for CharacterRaceFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

