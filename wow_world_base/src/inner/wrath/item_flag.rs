/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:457`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L457):
/// ```text
/// flag ItemFlag : u32 {
///     NO_PICKUP = 0x00000001;
///     CONJURED = 0x00000002;
///     LOOTABLE = 0x00000004;
///     HEROIC_TOOLTIP = 0x00000008;
///     DEPRECATED = 0x00000010;
///     INDESTRUCTIBLE = 0x00000020;
///     PLAYER_CAST = 0x00000040;
///     NO_EQUIP_COOLDOWN = 0x00000080;
///     MULTI_LOOT_QUEST = 0x00000100;
///     WRAPPER = 0x00000200;
///     USES_RESOURCES = 0x00000400;
///     MULTI_DROP = 0x00000800;
///     ITEM_PURCHASE_RECORD = 0x00001000;
///     CHARTER = 0x00002000;
///     HAS_TEXT = 0x00004000;
///     NO_DISENCHANT = 0x00008000;
///     REAL_DURATION = 0x00010000;
///     NO_CREATOR = 0x00020000;
///     IS_PROSPECTABLE = 0x00040000;
///     UNIQUE_EQUIPPED = 0x00080000;
///     IGNORE_FOR_AURAS = 0x00100000;
///     IGNORE_DEFAULT_ARENA_RESTRICTIONS = 0x00200000;
///     NO_DURABILITY_LOSS = 0x00400000;
///     USE_WHEN_SHAPESHIFTED = 0x00800000;
///     HAS_QUEST_GLOW = 0x01000000;
///     HIDE_UNUSABLE_RECIPE = 0x02000000;
///     NOT_USEABLE_IN_ARENA = 0x04000000;
///     IS_BOUND_TO_ACCOUNT = 0x08000000;
///     NO_REAGENT_COST = 0x10000000;
///     IS_MILLABLE = 0x20000000;
///     REPORT_TO_GUILD_CHAT = 0x40000000;
///     NO_PROGRESSIVE_LOOT = 0x80000000;
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
    pub(crate) const HEROIC_TOOLTIP: u32 = 0x08;
    pub(crate) const DEPRECATED: u32 = 0x10;
    pub(crate) const INDESTRUCTIBLE: u32 = 0x20;
    pub(crate) const PLAYER_CAST: u32 = 0x40;
    pub(crate) const NO_EQUIP_COOLDOWN: u32 = 0x80;
    pub(crate) const MULTI_LOOT_QUEST: u32 = 0x100;
    pub(crate) const WRAPPER: u32 = 0x200;
    pub(crate) const USES_RESOURCES: u32 = 0x400;
    pub(crate) const MULTI_DROP: u32 = 0x800;
    pub(crate) const ITEM_PURCHASE_RECORD: u32 = 0x1000;
    pub(crate) const CHARTER: u32 = 0x2000;
    pub(crate) const HAS_TEXT: u32 = 0x4000;
    pub(crate) const NO_DISENCHANT: u32 = 0x8000;
    pub(crate) const REAL_DURATION: u32 = 0x10000;
    pub(crate) const NO_CREATOR: u32 = 0x20000;
    pub(crate) const IS_PROSPECTABLE: u32 = 0x40000;
    pub(crate) const UNIQUE_EQUIPPED: u32 = 0x80000;
    pub(crate) const IGNORE_FOR_AURAS: u32 = 0x100000;
    pub(crate) const IGNORE_DEFAULT_ARENA_RESTRICTIONS: u32 = 0x200000;
    pub(crate) const NO_DURABILITY_LOSS: u32 = 0x400000;
    pub(crate) const USE_WHEN_SHAPESHIFTED: u32 = 0x800000;
    pub(crate) const HAS_QUEST_GLOW: u32 = 0x1000000;
    pub(crate) const HIDE_UNUSABLE_RECIPE: u32 = 0x2000000;
    pub(crate) const NOT_USEABLE_IN_ARENA: u32 = 0x4000000;
    pub(crate) const IS_BOUND_TO_ACCOUNT: u32 = 0x8000000;
    pub(crate) const NO_REAGENT_COST: u32 = 0x10000000;
    pub(crate) const IS_MILLABLE: u32 = 0x20000000;
    pub(crate) const REPORT_TO_GUILD_CHAT: u32 = 0x40000000;
    pub(crate) const NO_PROGRESSIVE_LOOT: u32 = 0x80000000;

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
                | Self::HEROIC_TOOLTIP
                | Self::DEPRECATED
                | Self::INDESTRUCTIBLE
                | Self::PLAYER_CAST
                | Self::NO_EQUIP_COOLDOWN
                | Self::MULTI_LOOT_QUEST
                | Self::WRAPPER
                | Self::USES_RESOURCES
                | Self::MULTI_DROP
                | Self::ITEM_PURCHASE_RECORD
                | Self::CHARTER
                | Self::HAS_TEXT
                | Self::NO_DISENCHANT
                | Self::REAL_DURATION
                | Self::NO_CREATOR
                | Self::IS_PROSPECTABLE
                | Self::UNIQUE_EQUIPPED
                | Self::IGNORE_FOR_AURAS
                | Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS
                | Self::NO_DURABILITY_LOSS
                | Self::USE_WHEN_SHAPESHIFTED
                | Self::HAS_QUEST_GLOW
                | Self::HIDE_UNUSABLE_RECIPE
                | Self::NOT_USEABLE_IN_ARENA
                | Self::IS_BOUND_TO_ACCOUNT
                | Self::NO_REAGENT_COST
                | Self::IS_MILLABLE
                | Self::REPORT_TO_GUILD_CHAT
                | Self::NO_PROGRESSIVE_LOOT
        }
    }

    pub const fn is_NO_PICKUP(&self) -> bool {
        (self.inner & Self::NO_PICKUP) != 0
    }

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

    /// Conjured item
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

    /// Item can be right clicked to open for loot
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

    pub const fn is_HEROIC_TOOLTIP(&self) -> bool {
        (self.inner & Self::HEROIC_TOOLTIP) != 0
    }

    /// Makes green 'Heroic' text appear on item
    ///
    pub const fn new_HEROIC_TOOLTIP() -> Self {
        Self { inner: Self::HEROIC_TOOLTIP }
    }

    pub fn set_HEROIC_TOOLTIP(&mut self) -> Self {
        self.inner |= Self::HEROIC_TOOLTIP;
        *self
    }

    pub fn clear_HEROIC_TOOLTIP(&mut self) -> Self {
        self.inner &= Self::HEROIC_TOOLTIP.reverse_bits();
        *self
    }

    pub const fn is_DEPRECATED(&self) -> bool {
        (self.inner & Self::DEPRECATED) != 0
    }

    /// Cannot equip or use
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

    /// Item can not be destroyed, except by using spell (item can be reagent for spell)
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

    /// Item's spells are castable by players
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

    /// No default 30 seconds cooldown when equipped
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

    pub const fn is_MULTI_LOOT_QUEST(&self) -> bool {
        (self.inner & Self::MULTI_LOOT_QUEST) != 0
    }

    pub const fn new_MULTI_LOOT_QUEST() -> Self {
        Self { inner: Self::MULTI_LOOT_QUEST }
    }

    pub fn set_MULTI_LOOT_QUEST(&mut self) -> Self {
        self.inner |= Self::MULTI_LOOT_QUEST;
        *self
    }

    pub fn clear_MULTI_LOOT_QUEST(&mut self) -> Self {
        self.inner &= Self::MULTI_LOOT_QUEST.reverse_bits();
        *self
    }

    pub const fn is_WRAPPER(&self) -> bool {
        (self.inner & Self::WRAPPER) != 0
    }

    /// Item can wrap other items
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

    pub const fn is_USES_RESOURCES(&self) -> bool {
        (self.inner & Self::USES_RESOURCES) != 0
    }

    pub const fn new_USES_RESOURCES() -> Self {
        Self { inner: Self::USES_RESOURCES }
    }

    pub fn set_USES_RESOURCES(&mut self) -> Self {
        self.inner |= Self::USES_RESOURCES;
        *self
    }

    pub fn clear_USES_RESOURCES(&mut self) -> Self {
        self.inner &= Self::USES_RESOURCES.reverse_bits();
        *self
    }

    pub const fn is_MULTI_DROP(&self) -> bool {
        (self.inner & Self::MULTI_DROP) != 0
    }

    /// Looting this item does not remove it from available loot
    ///
    pub const fn new_MULTI_DROP() -> Self {
        Self { inner: Self::MULTI_DROP }
    }

    pub fn set_MULTI_DROP(&mut self) -> Self {
        self.inner |= Self::MULTI_DROP;
        *self
    }

    pub fn clear_MULTI_DROP(&mut self) -> Self {
        self.inner &= Self::MULTI_DROP.reverse_bits();
        *self
    }

    pub const fn is_ITEM_PURCHASE_RECORD(&self) -> bool {
        (self.inner & Self::ITEM_PURCHASE_RECORD) != 0
    }

    /// Item can be returned to vendor for its original cost (extended cost)
    ///
    pub const fn new_ITEM_PURCHASE_RECORD() -> Self {
        Self { inner: Self::ITEM_PURCHASE_RECORD }
    }

    pub fn set_ITEM_PURCHASE_RECORD(&mut self) -> Self {
        self.inner |= Self::ITEM_PURCHASE_RECORD;
        *self
    }

    pub fn clear_ITEM_PURCHASE_RECORD(&mut self) -> Self {
        self.inner &= Self::ITEM_PURCHASE_RECORD.reverse_bits();
        *self
    }

    pub const fn is_CHARTER(&self) -> bool {
        (self.inner & Self::CHARTER) != 0
    }

    /// Item is guild or arena charter
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

    pub const fn is_IS_PROSPECTABLE(&self) -> bool {
        (self.inner & Self::IS_PROSPECTABLE) != 0
    }

    /// Item can be prospected
    ///
    pub const fn new_IS_PROSPECTABLE() -> Self {
        Self { inner: Self::IS_PROSPECTABLE }
    }

    pub fn set_IS_PROSPECTABLE(&mut self) -> Self {
        self.inner |= Self::IS_PROSPECTABLE;
        *self
    }

    pub fn clear_IS_PROSPECTABLE(&mut self) -> Self {
        self.inner &= Self::IS_PROSPECTABLE.reverse_bits();
        *self
    }

    pub const fn is_UNIQUE_EQUIPPED(&self) -> bool {
        (self.inner & Self::UNIQUE_EQUIPPED) != 0
    }

    /// You can only equip one of these
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

    pub const fn is_USE_WHEN_SHAPESHIFTED(&self) -> bool {
        (self.inner & Self::USE_WHEN_SHAPESHIFTED) != 0
    }

    /// Item can be used in shapeshift forms
    ///
    pub const fn new_USE_WHEN_SHAPESHIFTED() -> Self {
        Self { inner: Self::USE_WHEN_SHAPESHIFTED }
    }

    pub fn set_USE_WHEN_SHAPESHIFTED(&mut self) -> Self {
        self.inner |= Self::USE_WHEN_SHAPESHIFTED;
        *self
    }

    pub fn clear_USE_WHEN_SHAPESHIFTED(&mut self) -> Self {
        self.inner &= Self::USE_WHEN_SHAPESHIFTED.reverse_bits();
        *self
    }

    pub const fn is_HAS_QUEST_GLOW(&self) -> bool {
        (self.inner & Self::HAS_QUEST_GLOW) != 0
    }

    pub const fn new_HAS_QUEST_GLOW() -> Self {
        Self { inner: Self::HAS_QUEST_GLOW }
    }

    pub fn set_HAS_QUEST_GLOW(&mut self) -> Self {
        self.inner |= Self::HAS_QUEST_GLOW;
        *self
    }

    pub fn clear_HAS_QUEST_GLOW(&mut self) -> Self {
        self.inner &= Self::HAS_QUEST_GLOW.reverse_bits();
        *self
    }

    pub const fn is_HIDE_UNUSABLE_RECIPE(&self) -> bool {
        (self.inner & Self::HIDE_UNUSABLE_RECIPE) != 0
    }

    /// Profession recipes: can only be looted if you meet requirements and don't already know it
    ///
    pub const fn new_HIDE_UNUSABLE_RECIPE() -> Self {
        Self { inner: Self::HIDE_UNUSABLE_RECIPE }
    }

    pub fn set_HIDE_UNUSABLE_RECIPE(&mut self) -> Self {
        self.inner |= Self::HIDE_UNUSABLE_RECIPE;
        *self
    }

    pub fn clear_HIDE_UNUSABLE_RECIPE(&mut self) -> Self {
        self.inner &= Self::HIDE_UNUSABLE_RECIPE.reverse_bits();
        *self
    }

    pub const fn is_NOT_USEABLE_IN_ARENA(&self) -> bool {
        (self.inner & Self::NOT_USEABLE_IN_ARENA) != 0
    }

    /// Item cannot be used in arena
    ///
    pub const fn new_NOT_USEABLE_IN_ARENA() -> Self {
        Self { inner: Self::NOT_USEABLE_IN_ARENA }
    }

    pub fn set_NOT_USEABLE_IN_ARENA(&mut self) -> Self {
        self.inner |= Self::NOT_USEABLE_IN_ARENA;
        *self
    }

    pub fn clear_NOT_USEABLE_IN_ARENA(&mut self) -> Self {
        self.inner &= Self::NOT_USEABLE_IN_ARENA.reverse_bits();
        *self
    }

    pub const fn is_IS_BOUND_TO_ACCOUNT(&self) -> bool {
        (self.inner & Self::IS_BOUND_TO_ACCOUNT) != 0
    }

    /// Item binds to account and can be sent only to your own characters
    ///
    pub const fn new_IS_BOUND_TO_ACCOUNT() -> Self {
        Self { inner: Self::IS_BOUND_TO_ACCOUNT }
    }

    pub fn set_IS_BOUND_TO_ACCOUNT(&mut self) -> Self {
        self.inner |= Self::IS_BOUND_TO_ACCOUNT;
        *self
    }

    pub fn clear_IS_BOUND_TO_ACCOUNT(&mut self) -> Self {
        self.inner &= Self::IS_BOUND_TO_ACCOUNT.reverse_bits();
        *self
    }

    pub const fn is_NO_REAGENT_COST(&self) -> bool {
        (self.inner & Self::NO_REAGENT_COST) != 0
    }

    /// Spell is cast ignoring reagents
    ///
    pub const fn new_NO_REAGENT_COST() -> Self {
        Self { inner: Self::NO_REAGENT_COST }
    }

    pub fn set_NO_REAGENT_COST(&mut self) -> Self {
        self.inner |= Self::NO_REAGENT_COST;
        *self
    }

    pub fn clear_NO_REAGENT_COST(&mut self) -> Self {
        self.inner &= Self::NO_REAGENT_COST.reverse_bits();
        *self
    }

    pub const fn is_IS_MILLABLE(&self) -> bool {
        (self.inner & Self::IS_MILLABLE) != 0
    }

    /// Item can be milled
    ///
    pub const fn new_IS_MILLABLE() -> Self {
        Self { inner: Self::IS_MILLABLE }
    }

    pub fn set_IS_MILLABLE(&mut self) -> Self {
        self.inner |= Self::IS_MILLABLE;
        *self
    }

    pub fn clear_IS_MILLABLE(&mut self) -> Self {
        self.inner &= Self::IS_MILLABLE.reverse_bits();
        *self
    }

    pub const fn is_REPORT_TO_GUILD_CHAT(&self) -> bool {
        (self.inner & Self::REPORT_TO_GUILD_CHAT) != 0
    }

    pub const fn new_REPORT_TO_GUILD_CHAT() -> Self {
        Self { inner: Self::REPORT_TO_GUILD_CHAT }
    }

    pub fn set_REPORT_TO_GUILD_CHAT(&mut self) -> Self {
        self.inner |= Self::REPORT_TO_GUILD_CHAT;
        *self
    }

    pub fn clear_REPORT_TO_GUILD_CHAT(&mut self) -> Self {
        self.inner &= Self::REPORT_TO_GUILD_CHAT.reverse_bits();
        *self
    }

    pub const fn is_NO_PROGRESSIVE_LOOT(&self) -> bool {
        (self.inner & Self::NO_PROGRESSIVE_LOOT) != 0
    }

    pub const fn new_NO_PROGRESSIVE_LOOT() -> Self {
        Self { inner: Self::NO_PROGRESSIVE_LOOT }
    }

    pub fn set_NO_PROGRESSIVE_LOOT(&mut self) -> Self {
        self.inner |= Self::NO_PROGRESSIVE_LOOT;
        *self
    }

    pub fn clear_NO_PROGRESSIVE_LOOT(&mut self) -> Self {
        self.inner &= Self::NO_PROGRESSIVE_LOOT.reverse_bits();
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

