/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:115`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L115):
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
pub struct ItemFlag {
    inner: u32,
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

    pub const fn is_NO_PICKUP(&self) -> bool {
        (self.inner & Self::NO_PICKUP) != 0
    }

    /// not used
    ///
    pub const fn new_NO_PICKUP() -> Self {
        Self { inner: Self::NO_PICKUP }
    }

    pub fn set_NO_PICKUP(&mut self) -> Self {
        self.inner |= Self::NO_PICKUP;
        *self
    }

    pub fn clear_NO_PICKUP(&mut self) -> Self {
        self.inner &= Self::NO_PICKUP.reverse_bits();
        *self
    }

    pub const fn is_CONJURED(&self) -> bool {
        (self.inner & Self::CONJURED) != 0
    }

    pub const fn new_CONJURED() -> Self {
        Self { inner: Self::CONJURED }
    }

    pub fn set_CONJURED(&mut self) -> Self {
        self.inner |= Self::CONJURED;
        *self
    }

    pub fn clear_CONJURED(&mut self) -> Self {
        self.inner &= Self::CONJURED.reverse_bits();
        *self
    }

    pub const fn is_LOOTABLE(&self) -> bool {
        (self.inner & Self::LOOTABLE) != 0
    }

    /// affect only non container items that can be 'open' for loot. It or lockid set enable for client show 'Right click to open'. See also `ITEM_DYNFLAG_UNLOCKED`
    ///
    pub const fn new_LOOTABLE() -> Self {
        Self { inner: Self::LOOTABLE }
    }

    pub fn set_LOOTABLE(&mut self) -> Self {
        self.inner |= Self::LOOTABLE;
        *self
    }

    pub fn clear_LOOTABLE(&mut self) -> Self {
        self.inner &= Self::LOOTABLE.reverse_bits();
        *self
    }

    pub const fn is_DEPRECATED(&self) -> bool {
        (self.inner & Self::DEPRECATED) != 0
    }

    /// can't repeat old note: appears red icon (like when item durability==0)
    ///
    pub const fn new_DEPRECATED() -> Self {
        Self { inner: Self::DEPRECATED }
    }

    pub fn set_DEPRECATED(&mut self) -> Self {
        self.inner |= Self::DEPRECATED;
        *self
    }

    pub fn clear_DEPRECATED(&mut self) -> Self {
        self.inner &= Self::DEPRECATED.reverse_bits();
        *self
    }

    pub const fn is_INDESTRUCTIBLE(&self) -> bool {
        (self.inner & Self::INDESTRUCTIBLE) != 0
    }

    /// used for totem. Item can not be destroyed, except by using spell (item can be reagent for spell and then allowed)
    ///
    pub const fn new_INDESTRUCTIBLE() -> Self {
        Self { inner: Self::INDESTRUCTIBLE }
    }

    pub fn set_INDESTRUCTIBLE(&mut self) -> Self {
        self.inner |= Self::INDESTRUCTIBLE;
        *self
    }

    pub fn clear_INDESTRUCTIBLE(&mut self) -> Self {
        self.inner &= Self::INDESTRUCTIBLE.reverse_bits();
        *self
    }

    pub const fn is_PLAYER_CAST(&self) -> bool {
        (self.inner & Self::PLAYER_CAST) != 0
    }

    /// ? old note: usable
    ///
    pub const fn new_PLAYER_CAST() -> Self {
        Self { inner: Self::PLAYER_CAST }
    }

    pub fn set_PLAYER_CAST(&mut self) -> Self {
        self.inner |= Self::PLAYER_CAST;
        *self
    }

    pub fn clear_PLAYER_CAST(&mut self) -> Self {
        self.inner &= Self::PLAYER_CAST.reverse_bits();
        *self
    }

    pub const fn is_NO_EQUIP_COOLDOWN(&self) -> bool {
        (self.inner & Self::NO_EQUIP_COOLDOWN) != 0
    }

    pub const fn new_NO_EQUIP_COOLDOWN() -> Self {
        Self { inner: Self::NO_EQUIP_COOLDOWN }
    }

    pub fn set_NO_EQUIP_COOLDOWN(&mut self) -> Self {
        self.inner |= Self::NO_EQUIP_COOLDOWN;
        *self
    }

    pub fn clear_NO_EQUIP_COOLDOWN(&mut self) -> Self {
        self.inner &= Self::NO_EQUIP_COOLDOWN.reverse_bits();
        *self
    }

    pub const fn is_INT_BONUS_INSTEAD(&self) -> bool {
        (self.inner & Self::INT_BONUS_INSTEAD) != 0
    }

    pub const fn new_INT_BONUS_INSTEAD() -> Self {
        Self { inner: Self::INT_BONUS_INSTEAD }
    }

    pub fn set_INT_BONUS_INSTEAD(&mut self) -> Self {
        self.inner |= Self::INT_BONUS_INSTEAD;
        *self
    }

    pub fn clear_INT_BONUS_INSTEAD(&mut self) -> Self {
        self.inner &= Self::INT_BONUS_INSTEAD.reverse_bits();
        *self
    }

    pub const fn is_WRAPPER(&self) -> bool {
        (self.inner & Self::WRAPPER) != 0
    }

    /// used or not used wrapper
    ///
    pub const fn new_WRAPPER() -> Self {
        Self { inner: Self::WRAPPER }
    }

    pub fn set_WRAPPER(&mut self) -> Self {
        self.inner |= Self::WRAPPER;
        *self
    }

    pub fn clear_WRAPPER(&mut self) -> Self {
        self.inner &= Self::WRAPPER.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_BAG_SPACE(&self) -> bool {
        (self.inner & Self::IGNORE_BAG_SPACE) != 0
    }

    /// ignore bag space at new item creation?
    ///
    pub const fn new_IGNORE_BAG_SPACE() -> Self {
        Self { inner: Self::IGNORE_BAG_SPACE }
    }

    pub fn set_IGNORE_BAG_SPACE(&mut self) -> Self {
        self.inner |= Self::IGNORE_BAG_SPACE;
        *self
    }

    pub fn clear_IGNORE_BAG_SPACE(&mut self) -> Self {
        self.inner &= Self::IGNORE_BAG_SPACE.reverse_bits();
        *self
    }

    pub const fn is_PARTY_LOOT(&self) -> bool {
        (self.inner & Self::PARTY_LOOT) != 0
    }

    /// determines if item is party loot or not
    ///
    pub const fn new_PARTY_LOOT() -> Self {
        Self { inner: Self::PARTY_LOOT }
    }

    pub fn set_PARTY_LOOT(&mut self) -> Self {
        self.inner |= Self::PARTY_LOOT;
        *self
    }

    pub fn clear_PARTY_LOOT(&mut self) -> Self {
        self.inner &= Self::PARTY_LOOT.reverse_bits();
        *self
    }

    pub const fn is_CHARTER(&self) -> bool {
        (self.inner & Self::CHARTER) != 0
    }

    /// guild charter
    ///
    pub const fn new_CHARTER() -> Self {
        Self { inner: Self::CHARTER }
    }

    pub fn set_CHARTER(&mut self) -> Self {
        self.inner |= Self::CHARTER;
        *self
    }

    pub fn clear_CHARTER(&mut self) -> Self {
        self.inner &= Self::CHARTER.reverse_bits();
        *self
    }

    pub const fn is_HAS_TEXT(&self) -> bool {
        (self.inner & Self::HAS_TEXT) != 0
    }

    /// Only readable items have this (but not all)
    ///
    pub const fn new_HAS_TEXT() -> Self {
        Self { inner: Self::HAS_TEXT }
    }

    pub fn set_HAS_TEXT(&mut self) -> Self {
        self.inner |= Self::HAS_TEXT;
        *self
    }

    pub fn clear_HAS_TEXT(&mut self) -> Self {
        self.inner &= Self::HAS_TEXT.reverse_bits();
        *self
    }

    pub const fn is_NO_DISENCHANT(&self) -> bool {
        (self.inner & Self::NO_DISENCHANT) != 0
    }

    pub const fn new_NO_DISENCHANT() -> Self {
        Self { inner: Self::NO_DISENCHANT }
    }

    pub fn set_NO_DISENCHANT(&mut self) -> Self {
        self.inner |= Self::NO_DISENCHANT;
        *self
    }

    pub fn clear_NO_DISENCHANT(&mut self) -> Self {
        self.inner &= Self::NO_DISENCHANT.reverse_bits();
        *self
    }

    pub const fn is_REAL_DURATION(&self) -> bool {
        (self.inner & Self::REAL_DURATION) != 0
    }

    pub const fn new_REAL_DURATION() -> Self {
        Self { inner: Self::REAL_DURATION }
    }

    pub fn set_REAL_DURATION(&mut self) -> Self {
        self.inner |= Self::REAL_DURATION;
        *self
    }

    pub fn clear_REAL_DURATION(&mut self) -> Self {
        self.inner &= Self::REAL_DURATION.reverse_bits();
        *self
    }

    pub const fn is_NO_CREATOR(&self) -> bool {
        (self.inner & Self::NO_CREATOR) != 0
    }

    pub const fn new_NO_CREATOR() -> Self {
        Self { inner: Self::NO_CREATOR }
    }

    pub fn set_NO_CREATOR(&mut self) -> Self {
        self.inner |= Self::NO_CREATOR;
        *self
    }

    pub fn clear_NO_CREATOR(&mut self) -> Self {
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

