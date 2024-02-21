/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:112`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L112):
/// ```text
/// flag ItemFlag : u32 {
///     NONE = 0x00000000;
///     NO_PICKUP = 0x00000001;
///     CONJURED = 0x00000002;
///     LOOTABLE = 0x00000004;
///     DEPRECATED = 0x00000010;
///     INDESTRUCTIBLE = 0x00000020;
///     PLAYER_CAST = 0x00000040;
///     NO_EQUIP_COOLDOWN = 0x00000080;
///     INT_BONUS_INSTEAD = 0x00000100;
///     WRAPPER = 0x00000200;
///     IGNORE_BAG_SPACE = 0x00000400;
///     PARTY_LOOT = 0x00000800;
///     CHARTER = 0x00002000;
///     HAS_TEXT = 0x00004000;
///     NO_DISENCHANT = 0x00008000;
///     REAL_DURATION = 0x00010000;
///     NO_CREATOR = 0x00020000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ItemFlag {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl ItemFlag {
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
        if self.is_no_pickup() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_PICKUP").unwrap();
            first = false;
        }
        if self.is_conjured() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CONJURED").unwrap();
            first = false;
        }
        if self.is_lootable() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LOOTABLE").unwrap();
            first = false;
        }
        if self.is_deprecated() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DEPRECATED").unwrap();
            first = false;
        }
        if self.is_indestructible() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INDESTRUCTIBLE").unwrap();
            first = false;
        }
        if self.is_player_cast() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PLAYER_CAST").unwrap();
            first = false;
        }
        if self.is_no_equip_cooldown() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_EQUIP_COOLDOWN").unwrap();
            first = false;
        }
        if self.is_int_bonus_instead() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INT_BONUS_INSTEAD").unwrap();
            first = false;
        }
        if self.is_wrapper() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "WRAPPER").unwrap();
            first = false;
        }
        if self.is_ignore_bag_space() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IGNORE_BAG_SPACE").unwrap();
            first = false;
        }
        if self.is_party_loot() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PARTY_LOOT").unwrap();
            first = false;
        }
        if self.is_charter() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CHARTER").unwrap();
            first = false;
        }
        if self.is_has_text() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HAS_TEXT").unwrap();
            first = false;
        }
        if self.is_no_disenchant() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_DISENCHANT").unwrap();
            first = false;
        }
        if self.is_real_duration() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "REAL_DURATION").unwrap();
            first = false;
        }
        if self.is_no_creator() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_CREATOR").unwrap();
            first = false;
        }
        s
    }

}

impl ItemFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const NO_PICKUP: u32 = 0x01;
    pub const CONJURED: u32 = 0x02;
    pub const LOOTABLE: u32 = 0x04;
    pub const DEPRECATED: u32 = 0x10;
    pub const INDESTRUCTIBLE: u32 = 0x20;
    pub const PLAYER_CAST: u32 = 0x40;
    pub const NO_EQUIP_COOLDOWN: u32 = 0x80;
    pub const INT_BONUS_INSTEAD: u32 = 0x100;
    pub const WRAPPER: u32 = 0x200;
    pub const IGNORE_BAG_SPACE: u32 = 0x400;
    pub const PARTY_LOOT: u32 = 0x800;
    pub const CHARTER: u32 = 0x2000;
    pub const HAS_TEXT: u32 = 0x4000;
    pub const NO_DISENCHANT: u32 = 0x8000;
    pub const REAL_DURATION: u32 = 0x10000;
    pub const NO_CREATOR: u32 = 0x20000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::NO_PICKUP
                | Self::CONJURED
                | Self::LOOTABLE
                | Self::DEPRECATED
                | Self::INDESTRUCTIBLE
                | Self::PLAYER_CAST
                | Self::NO_EQUIP_COOLDOWN
                | Self::INT_BONUS_INSTEAD
                | Self::WRAPPER
                | Self::IGNORE_BAG_SPACE
                | Self::PARTY_LOOT
                | Self::CHARTER
                | Self::HAS_TEXT
                | Self::NO_DISENCHANT
                | Self::REAL_DURATION
                | Self::NO_CREATOR
        }
    }

    pub const fn is_no_pickup(&self) -> bool {
        (self.inner & Self::NO_PICKUP) != 0
    }

    /// not used
    pub const fn new_no_pickup() -> Self {
        Self { inner: Self::NO_PICKUP }
    }

    pub fn set_no_pickup(&mut self) -> Self {
        self.inner |= Self::NO_PICKUP;
        *self
    }

    pub fn clear_no_pickup(&mut self) -> Self {
        self.inner &= Self::NO_PICKUP.reverse_bits();
        *self
    }

    pub const fn is_conjured(&self) -> bool {
        (self.inner & Self::CONJURED) != 0
    }

    pub const fn new_conjured() -> Self {
        Self { inner: Self::CONJURED }
    }

    pub fn set_conjured(&mut self) -> Self {
        self.inner |= Self::CONJURED;
        *self
    }

    pub fn clear_conjured(&mut self) -> Self {
        self.inner &= Self::CONJURED.reverse_bits();
        *self
    }

    pub const fn is_lootable(&self) -> bool {
        (self.inner & Self::LOOTABLE) != 0
    }

    /// affect only non container items that can be 'open' for loot. It or lockid set enable for client show 'Right click to open'. See also `ITEM_DYNFLAG_UNLOCKED`
    pub const fn new_lootable() -> Self {
        Self { inner: Self::LOOTABLE }
    }

    pub fn set_lootable(&mut self) -> Self {
        self.inner |= Self::LOOTABLE;
        *self
    }

    pub fn clear_lootable(&mut self) -> Self {
        self.inner &= Self::LOOTABLE.reverse_bits();
        *self
    }

    pub const fn is_deprecated(&self) -> bool {
        (self.inner & Self::DEPRECATED) != 0
    }

    /// can't repeat old note: appears red icon (like when item durability==0)
    pub const fn new_deprecated() -> Self {
        Self { inner: Self::DEPRECATED }
    }

    pub fn set_deprecated(&mut self) -> Self {
        self.inner |= Self::DEPRECATED;
        *self
    }

    pub fn clear_deprecated(&mut self) -> Self {
        self.inner &= Self::DEPRECATED.reverse_bits();
        *self
    }

    pub const fn is_indestructible(&self) -> bool {
        (self.inner & Self::INDESTRUCTIBLE) != 0
    }

    /// used for totem. Item can not be destroyed, except by using spell (item can be reagent for spell and then allowed)
    pub const fn new_indestructible() -> Self {
        Self { inner: Self::INDESTRUCTIBLE }
    }

    pub fn set_indestructible(&mut self) -> Self {
        self.inner |= Self::INDESTRUCTIBLE;
        *self
    }

    pub fn clear_indestructible(&mut self) -> Self {
        self.inner &= Self::INDESTRUCTIBLE.reverse_bits();
        *self
    }

    pub const fn is_player_cast(&self) -> bool {
        (self.inner & Self::PLAYER_CAST) != 0
    }

    /// ? old note: usable
    pub const fn new_player_cast() -> Self {
        Self { inner: Self::PLAYER_CAST }
    }

    pub fn set_player_cast(&mut self) -> Self {
        self.inner |= Self::PLAYER_CAST;
        *self
    }

    pub fn clear_player_cast(&mut self) -> Self {
        self.inner &= Self::PLAYER_CAST.reverse_bits();
        *self
    }

    pub const fn is_no_equip_cooldown(&self) -> bool {
        (self.inner & Self::NO_EQUIP_COOLDOWN) != 0
    }

    pub const fn new_no_equip_cooldown() -> Self {
        Self { inner: Self::NO_EQUIP_COOLDOWN }
    }

    pub fn set_no_equip_cooldown(&mut self) -> Self {
        self.inner |= Self::NO_EQUIP_COOLDOWN;
        *self
    }

    pub fn clear_no_equip_cooldown(&mut self) -> Self {
        self.inner &= Self::NO_EQUIP_COOLDOWN.reverse_bits();
        *self
    }

    pub const fn is_int_bonus_instead(&self) -> bool {
        (self.inner & Self::INT_BONUS_INSTEAD) != 0
    }

    pub const fn new_int_bonus_instead() -> Self {
        Self { inner: Self::INT_BONUS_INSTEAD }
    }

    pub fn set_int_bonus_instead(&mut self) -> Self {
        self.inner |= Self::INT_BONUS_INSTEAD;
        *self
    }

    pub fn clear_int_bonus_instead(&mut self) -> Self {
        self.inner &= Self::INT_BONUS_INSTEAD.reverse_bits();
        *self
    }

    pub const fn is_wrapper(&self) -> bool {
        (self.inner & Self::WRAPPER) != 0
    }

    /// used or not used wrapper
    pub const fn new_wrapper() -> Self {
        Self { inner: Self::WRAPPER }
    }

    pub fn set_wrapper(&mut self) -> Self {
        self.inner |= Self::WRAPPER;
        *self
    }

    pub fn clear_wrapper(&mut self) -> Self {
        self.inner &= Self::WRAPPER.reverse_bits();
        *self
    }

    pub const fn is_ignore_bag_space(&self) -> bool {
        (self.inner & Self::IGNORE_BAG_SPACE) != 0
    }

    /// ignore bag space at new item creation?
    pub const fn new_ignore_bag_space() -> Self {
        Self { inner: Self::IGNORE_BAG_SPACE }
    }

    pub fn set_ignore_bag_space(&mut self) -> Self {
        self.inner |= Self::IGNORE_BAG_SPACE;
        *self
    }

    pub fn clear_ignore_bag_space(&mut self) -> Self {
        self.inner &= Self::IGNORE_BAG_SPACE.reverse_bits();
        *self
    }

    pub const fn is_party_loot(&self) -> bool {
        (self.inner & Self::PARTY_LOOT) != 0
    }

    /// determines if item is party loot or not
    pub const fn new_party_loot() -> Self {
        Self { inner: Self::PARTY_LOOT }
    }

    pub fn set_party_loot(&mut self) -> Self {
        self.inner |= Self::PARTY_LOOT;
        *self
    }

    pub fn clear_party_loot(&mut self) -> Self {
        self.inner &= Self::PARTY_LOOT.reverse_bits();
        *self
    }

    pub const fn is_charter(&self) -> bool {
        (self.inner & Self::CHARTER) != 0
    }

    /// guild charter
    pub const fn new_charter() -> Self {
        Self { inner: Self::CHARTER }
    }

    pub fn set_charter(&mut self) -> Self {
        self.inner |= Self::CHARTER;
        *self
    }

    pub fn clear_charter(&mut self) -> Self {
        self.inner &= Self::CHARTER.reverse_bits();
        *self
    }

    pub const fn is_has_text(&self) -> bool {
        (self.inner & Self::HAS_TEXT) != 0
    }

    /// Only readable items have this (but not all)
    pub const fn new_has_text() -> Self {
        Self { inner: Self::HAS_TEXT }
    }

    pub fn set_has_text(&mut self) -> Self {
        self.inner |= Self::HAS_TEXT;
        *self
    }

    pub fn clear_has_text(&mut self) -> Self {
        self.inner &= Self::HAS_TEXT.reverse_bits();
        *self
    }

    pub const fn is_no_disenchant(&self) -> bool {
        (self.inner & Self::NO_DISENCHANT) != 0
    }

    pub const fn new_no_disenchant() -> Self {
        Self { inner: Self::NO_DISENCHANT }
    }

    pub fn set_no_disenchant(&mut self) -> Self {
        self.inner |= Self::NO_DISENCHANT;
        *self
    }

    pub fn clear_no_disenchant(&mut self) -> Self {
        self.inner &= Self::NO_DISENCHANT.reverse_bits();
        *self
    }

    pub const fn is_real_duration(&self) -> bool {
        (self.inner & Self::REAL_DURATION) != 0
    }

    pub const fn new_real_duration() -> Self {
        Self { inner: Self::REAL_DURATION }
    }

    pub fn set_real_duration(&mut self) -> Self {
        self.inner |= Self::REAL_DURATION;
        *self
    }

    pub fn clear_real_duration(&mut self) -> Self {
        self.inner &= Self::REAL_DURATION.reverse_bits();
        *self
    }

    pub const fn is_no_creator(&self) -> bool {
        (self.inner & Self::NO_CREATOR) != 0
    }

    pub const fn new_no_creator() -> Self {
        Self { inner: Self::NO_CREATOR }
    }

    pub fn set_no_creator(&mut self) -> Self {
        self.inner |= Self::NO_CREATOR;
        *self
    }

    pub fn clear_no_creator(&mut self) -> Self {
        self.inner &= Self::NO_CREATOR.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for ItemFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for ItemFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for ItemFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for ItemFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for ItemFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for ItemFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for ItemFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for ItemFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for ItemFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for ItemFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for ItemFlag {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for ItemFlag {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for ItemFlag {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for ItemFlag {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for ItemFlag {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl TryFrom<i16> for ItemFlag {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        Ok(Self::new(v.into()))
    }
}

impl From<i32> for ItemFlag {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for ItemFlag {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for ItemFlag {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

