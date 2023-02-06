/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:297`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L297):
/// ```text
/// flag ItemFlag : u32 {
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
///     LETTER = 0x00004000;
///     NO_DISENCHANT = 0x00008000;
///     REAL_DURATION = 0x00010000;
///     NO_CREATOR = 0x00020000;
///     PROSPECTABLE = 0x00040000;
///     UNIQUE_EQUIPPED = 0x00080000;
///     IGNORE_FOR_AURAS = 0x00100000;
///     IGNORE_DEFAULT_ARENA_RESTRICTIONS = 0x00200000;
///     NO_DURABILITY_LOSS = 0x00400000;
///     SPECIAL_USE = 0x00800000;
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

    pub(crate) const NO_PICKUP: u32 = 0x01;
    pub(crate) const CONJURED: u32 = 0x02;
    pub(crate) const LOOTABLE: u32 = 0x04;
    pub(crate) const DEPRECATED: u32 = 0x10;
    pub(crate) const INDESTRUCTIBLE: u32 = 0x20;
    pub(crate) const PLAYER_CAST: u32 = 0x40;
    pub(crate) const NO_EQUIP_COOLDOWN: u32 = 0x80;
    pub(crate) const INT_BONUS_INSTEAD: u32 = 0x100;
    pub(crate) const WRAPPER: u32 = 0x200;
    pub(crate) const IGNORE_BAG_SPACE: u32 = 0x400;
    pub(crate) const PARTY_LOOT: u32 = 0x800;
    pub(crate) const CHARTER: u32 = 0x2000;
    pub(crate) const LETTER: u32 = 0x4000;
    pub(crate) const NO_DISENCHANT: u32 = 0x8000;
    pub(crate) const REAL_DURATION: u32 = 0x10000;
    pub(crate) const NO_CREATOR: u32 = 0x20000;
    pub(crate) const PROSPECTABLE: u32 = 0x40000;
    pub(crate) const UNIQUE_EQUIPPED: u32 = 0x80000;
    pub(crate) const IGNORE_FOR_AURAS: u32 = 0x100000;
    pub(crate) const IGNORE_DEFAULT_ARENA_RESTRICTIONS: u32 = 0x200000;
    pub(crate) const NO_DURABILITY_LOSS: u32 = 0x400000;
    pub(crate) const SPECIAL_USE: u32 = 0x800000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NO_PICKUP
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
                | Self::LETTER
                | Self::NO_DISENCHANT
                | Self::REAL_DURATION
                | Self::NO_CREATOR
                | Self::PROSPECTABLE
                | Self::UNIQUE_EQUIPPED
                | Self::IGNORE_FOR_AURAS
                | Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS
                | Self::NO_DURABILITY_LOSS
                | Self::SPECIAL_USE
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

    /// items created by spells with SPELL_EFFECT_CREATE_ITEM
    ///
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

    /// affect only non container items that can be 'open' for loot. It or lockid set enable for client show 'Right click to open'. See also ITEM_DYNFLAG_UNLOCKED
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

    /// items without an equip cooldown (and usually a _USABLE flag)
    ///
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

    /// saw this on item 47115, 49295...
    ///
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

    /// items which can be looted by all party members
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

    /// arena/guild charter
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

    pub const fn is_LETTER(&self) -> bool {
        (self.inner & Self::LETTER) != 0
    }

    /// readable letter items
    ///
    pub const fn new_LETTER() -> Self {
        Self { inner: Self::LETTER }
    }

    pub fn set_LETTER(&mut self) -> Self {
        self.inner |= Self::LETTER;
        *self
    }

    pub fn clear_LETTER(&mut self) -> Self {
        self.inner &= Self::LETTER.reverse_bits();
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

    pub const fn is_PROSPECTABLE(&self) -> bool {
        (self.inner & Self::PROSPECTABLE) != 0
    }

    /// item can have prospecting loot (in fact some items expected have empty loot)
    ///
    pub const fn new_PROSPECTABLE() -> Self {
        Self { inner: Self::PROSPECTABLE }
    }

    pub fn set_PROSPECTABLE(&mut self) -> Self {
        self.inner |= Self::PROSPECTABLE;
        *self
    }

    pub fn clear_PROSPECTABLE(&mut self) -> Self {
        self.inner &= Self::PROSPECTABLE.reverse_bits();
        *self
    }

    pub const fn is_UNIQUE_EQUIPPED(&self) -> bool {
        (self.inner & Self::UNIQUE_EQUIPPED) != 0
    }

    /// custom server side check, in client added in 2.x
    ///
    pub const fn new_UNIQUE_EQUIPPED() -> Self {
        Self { inner: Self::UNIQUE_EQUIPPED }
    }

    pub fn set_UNIQUE_EQUIPPED(&mut self) -> Self {
        self.inner |= Self::UNIQUE_EQUIPPED;
        *self
    }

    pub fn clear_UNIQUE_EQUIPPED(&mut self) -> Self {
        self.inner &= Self::UNIQUE_EQUIPPED.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_FOR_AURAS(&self) -> bool {
        (self.inner & Self::IGNORE_FOR_AURAS) != 0
    }

    pub const fn new_IGNORE_FOR_AURAS() -> Self {
        Self { inner: Self::IGNORE_FOR_AURAS }
    }

    pub fn set_IGNORE_FOR_AURAS(&mut self) -> Self {
        self.inner |= Self::IGNORE_FOR_AURAS;
        *self
    }

    pub fn clear_IGNORE_FOR_AURAS(&mut self) -> Self {
        self.inner &= Self::IGNORE_FOR_AURAS.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_DEFAULT_ARENA_RESTRICTIONS(&self) -> bool {
        (self.inner & Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS) != 0
    }

    /// Item can be used during arena match
    ///
    pub const fn new_IGNORE_DEFAULT_ARENA_RESTRICTIONS() -> Self {
        Self { inner: Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS }
    }

    pub fn set_IGNORE_DEFAULT_ARENA_RESTRICTIONS(&mut self) -> Self {
        self.inner |= Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS;
        *self
    }

    pub fn clear_IGNORE_DEFAULT_ARENA_RESTRICTIONS(&mut self) -> Self {
        self.inner &= Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS.reverse_bits();
        *self
    }

    pub const fn is_NO_DURABILITY_LOSS(&self) -> bool {
        (self.inner & Self::NO_DURABILITY_LOSS) != 0
    }

    /// Some Thrown weapons have it (and only Thrown) but not all
    ///
    pub const fn new_NO_DURABILITY_LOSS() -> Self {
        Self { inner: Self::NO_DURABILITY_LOSS }
    }

    pub fn set_NO_DURABILITY_LOSS(&mut self) -> Self {
        self.inner |= Self::NO_DURABILITY_LOSS;
        *self
    }

    pub fn clear_NO_DURABILITY_LOSS(&mut self) -> Self {
        self.inner &= Self::NO_DURABILITY_LOSS.reverse_bits();
        *self
    }

    pub const fn is_SPECIAL_USE(&self) -> bool {
        (self.inner & Self::SPECIAL_USE) != 0
    }

    pub const fn new_SPECIAL_USE() -> Self {
        Self { inner: Self::SPECIAL_USE }
    }

    pub fn set_SPECIAL_USE(&mut self) -> Self {
        self.inner |= Self::SPECIAL_USE;
        *self
    }

    pub fn clear_SPECIAL_USE(&mut self) -> Self {
        self.inner &= Self::SPECIAL_USE.reverse_bits();
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

