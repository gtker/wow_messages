/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:449`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L449):
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

#[cfg(feature = "print-testcase")]
impl ItemFlag {
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_no_pickup() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_PICKUP").unwrap();
            first = false;
        }
        if self.is_conjured() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CONJURED").unwrap();
            first = false;
        }
        if self.is_lootable() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "LOOTABLE").unwrap();
            first = false;
        }
        if self.is_heroic_tooltip() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HEROIC_TOOLTIP").unwrap();
            first = false;
        }
        if self.is_deprecated() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DEPRECATED").unwrap();
            first = false;
        }
        if self.is_indestructible() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INDESTRUCTIBLE").unwrap();
            first = false;
        }
        if self.is_player_cast() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PLAYER_CAST").unwrap();
            first = false;
        }
        if self.is_no_equip_cooldown() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_EQUIP_COOLDOWN").unwrap();
            first = false;
        }
        if self.is_multi_loot_quest() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MULTI_LOOT_QUEST").unwrap();
            first = false;
        }
        if self.is_wrapper() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "WRAPPER").unwrap();
            first = false;
        }
        if self.is_uses_resources() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "USES_RESOURCES").unwrap();
            first = false;
        }
        if self.is_multi_drop() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MULTI_DROP").unwrap();
            first = false;
        }
        if self.is_item_purchase_record() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ITEM_PURCHASE_RECORD").unwrap();
            first = false;
        }
        if self.is_charter() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CHARTER").unwrap();
            first = false;
        }
        if self.is_has_text() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HAS_TEXT").unwrap();
            first = false;
        }
        if self.is_no_disenchant() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_DISENCHANT").unwrap();
            first = false;
        }
        if self.is_real_duration() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "REAL_DURATION").unwrap();
            first = false;
        }
        if self.is_no_creator() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_CREATOR").unwrap();
            first = false;
        }
        if self.is_is_prospectable() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IS_PROSPECTABLE").unwrap();
            first = false;
        }
        if self.is_unique_equipped() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNIQUE_EQUIPPED").unwrap();
            first = false;
        }
        if self.is_ignore_for_auras() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_FOR_AURAS").unwrap();
            first = false;
        }
        if self.is_ignore_default_arena_restrictions() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_DEFAULT_ARENA_RESTRICTIONS").unwrap();
            first = false;
        }
        if self.is_no_durability_loss() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_DURABILITY_LOSS").unwrap();
            first = false;
        }
        if self.is_use_when_shapeshifted() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "USE_WHEN_SHAPESHIFTED").unwrap();
            first = false;
        }
        if self.is_has_quest_glow() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HAS_QUEST_GLOW").unwrap();
            first = false;
        }
        if self.is_hide_unusable_recipe() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HIDE_UNUSABLE_RECIPE").unwrap();
            first = false;
        }
        if self.is_not_useable_in_arena() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NOT_USEABLE_IN_ARENA").unwrap();
            first = false;
        }
        if self.is_is_bound_to_account() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IS_BOUND_TO_ACCOUNT").unwrap();
            first = false;
        }
        if self.is_no_reagent_cost() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_REAGENT_COST").unwrap();
            first = false;
        }
        if self.is_is_millable() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IS_MILLABLE").unwrap();
            first = false;
        }
        if self.is_report_to_guild_chat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "REPORT_TO_GUILD_CHAT").unwrap();
            first = false;
        }
        if self.is_no_progressive_loot() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_PROGRESSIVE_LOOT").unwrap();
            first = false;
        }
        s
    }

}

impl ItemFlag {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NO_PICKUP: u32 = 0x01;
    pub const CONJURED: u32 = 0x02;
    pub const LOOTABLE: u32 = 0x04;
    pub const HEROIC_TOOLTIP: u32 = 0x08;
    pub const DEPRECATED: u32 = 0x10;
    pub const INDESTRUCTIBLE: u32 = 0x20;
    pub const PLAYER_CAST: u32 = 0x40;
    pub const NO_EQUIP_COOLDOWN: u32 = 0x80;
    pub const MULTI_LOOT_QUEST: u32 = 0x100;
    pub const WRAPPER: u32 = 0x200;
    pub const USES_RESOURCES: u32 = 0x400;
    pub const MULTI_DROP: u32 = 0x800;
    pub const ITEM_PURCHASE_RECORD: u32 = 0x1000;
    pub const CHARTER: u32 = 0x2000;
    pub const HAS_TEXT: u32 = 0x4000;
    pub const NO_DISENCHANT: u32 = 0x8000;
    pub const REAL_DURATION: u32 = 0x10000;
    pub const NO_CREATOR: u32 = 0x20000;
    pub const IS_PROSPECTABLE: u32 = 0x40000;
    pub const UNIQUE_EQUIPPED: u32 = 0x80000;
    pub const IGNORE_FOR_AURAS: u32 = 0x100000;
    pub const IGNORE_DEFAULT_ARENA_RESTRICTIONS: u32 = 0x200000;
    pub const NO_DURABILITY_LOSS: u32 = 0x400000;
    pub const USE_WHEN_SHAPESHIFTED: u32 = 0x800000;
    pub const HAS_QUEST_GLOW: u32 = 0x1000000;
    pub const HIDE_UNUSABLE_RECIPE: u32 = 0x2000000;
    pub const NOT_USEABLE_IN_ARENA: u32 = 0x4000000;
    pub const IS_BOUND_TO_ACCOUNT: u32 = 0x8000000;
    pub const NO_REAGENT_COST: u32 = 0x10000000;
    pub const IS_MILLABLE: u32 = 0x20000000;
    pub const REPORT_TO_GUILD_CHAT: u32 = 0x40000000;
    pub const NO_PROGRESSIVE_LOOT: u32 = 0x80000000;

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

    pub const fn is_no_pickup(&self) -> bool {
        (self.inner & Self::NO_PICKUP) != 0
    }

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

    /// Conjured item
    ///
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

    /// Item can be right clicked to open for loot
    ///
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

    pub const fn is_heroic_tooltip(&self) -> bool {
        (self.inner & Self::HEROIC_TOOLTIP) != 0
    }

    /// Makes green 'Heroic' text appear on item
    ///
    pub const fn new_heroic_tooltip() -> Self {
        Self { inner: Self::HEROIC_TOOLTIP }
    }

    pub fn set_heroic_tooltip(&mut self) -> Self {
        self.inner |= Self::HEROIC_TOOLTIP;
        *self
    }

    pub fn clear_heroic_tooltip(&mut self) -> Self {
        self.inner &= Self::HEROIC_TOOLTIP.reverse_bits();
        *self
    }

    pub const fn is_deprecated(&self) -> bool {
        (self.inner & Self::DEPRECATED) != 0
    }

    /// Cannot equip or use
    ///
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

    /// Item can not be destroyed, except by using spell (item can be reagent for spell)
    ///
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

    /// Item's spells are castable by players
    ///
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

    /// No default 30 seconds cooldown when equipped
    ///
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

    pub const fn is_multi_loot_quest(&self) -> bool {
        (self.inner & Self::MULTI_LOOT_QUEST) != 0
    }

    pub const fn new_multi_loot_quest() -> Self {
        Self { inner: Self::MULTI_LOOT_QUEST }
    }

    pub fn set_multi_loot_quest(&mut self) -> Self {
        self.inner |= Self::MULTI_LOOT_QUEST;
        *self
    }

    pub fn clear_multi_loot_quest(&mut self) -> Self {
        self.inner &= Self::MULTI_LOOT_QUEST.reverse_bits();
        *self
    }

    pub const fn is_wrapper(&self) -> bool {
        (self.inner & Self::WRAPPER) != 0
    }

    /// Item can wrap other items
    ///
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

    pub const fn is_uses_resources(&self) -> bool {
        (self.inner & Self::USES_RESOURCES) != 0
    }

    pub const fn new_uses_resources() -> Self {
        Self { inner: Self::USES_RESOURCES }
    }

    pub fn set_uses_resources(&mut self) -> Self {
        self.inner |= Self::USES_RESOURCES;
        *self
    }

    pub fn clear_uses_resources(&mut self) -> Self {
        self.inner &= Self::USES_RESOURCES.reverse_bits();
        *self
    }

    pub const fn is_multi_drop(&self) -> bool {
        (self.inner & Self::MULTI_DROP) != 0
    }

    /// Looting this item does not remove it from available loot
    ///
    pub const fn new_multi_drop() -> Self {
        Self { inner: Self::MULTI_DROP }
    }

    pub fn set_multi_drop(&mut self) -> Self {
        self.inner |= Self::MULTI_DROP;
        *self
    }

    pub fn clear_multi_drop(&mut self) -> Self {
        self.inner &= Self::MULTI_DROP.reverse_bits();
        *self
    }

    pub const fn is_item_purchase_record(&self) -> bool {
        (self.inner & Self::ITEM_PURCHASE_RECORD) != 0
    }

    /// Item can be returned to vendor for its original cost (extended cost)
    ///
    pub const fn new_item_purchase_record() -> Self {
        Self { inner: Self::ITEM_PURCHASE_RECORD }
    }

    pub fn set_item_purchase_record(&mut self) -> Self {
        self.inner |= Self::ITEM_PURCHASE_RECORD;
        *self
    }

    pub fn clear_item_purchase_record(&mut self) -> Self {
        self.inner &= Self::ITEM_PURCHASE_RECORD.reverse_bits();
        *self
    }

    pub const fn is_charter(&self) -> bool {
        (self.inner & Self::CHARTER) != 0
    }

    /// Item is guild or arena charter
    ///
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
    ///
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

    pub const fn is_is_prospectable(&self) -> bool {
        (self.inner & Self::IS_PROSPECTABLE) != 0
    }

    /// Item can be prospected
    ///
    pub const fn new_is_prospectable() -> Self {
        Self { inner: Self::IS_PROSPECTABLE }
    }

    pub fn set_is_prospectable(&mut self) -> Self {
        self.inner |= Self::IS_PROSPECTABLE;
        *self
    }

    pub fn clear_is_prospectable(&mut self) -> Self {
        self.inner &= Self::IS_PROSPECTABLE.reverse_bits();
        *self
    }

    pub const fn is_unique_equipped(&self) -> bool {
        (self.inner & Self::UNIQUE_EQUIPPED) != 0
    }

    /// You can only equip one of these
    ///
    pub const fn new_unique_equipped() -> Self {
        Self { inner: Self::UNIQUE_EQUIPPED }
    }

    pub fn set_unique_equipped(&mut self) -> Self {
        self.inner |= Self::UNIQUE_EQUIPPED;
        *self
    }

    pub fn clear_unique_equipped(&mut self) -> Self {
        self.inner &= Self::UNIQUE_EQUIPPED.reverse_bits();
        *self
    }

    pub const fn is_ignore_for_auras(&self) -> bool {
        (self.inner & Self::IGNORE_FOR_AURAS) != 0
    }

    pub const fn new_ignore_for_auras() -> Self {
        Self { inner: Self::IGNORE_FOR_AURAS }
    }

    pub fn set_ignore_for_auras(&mut self) -> Self {
        self.inner |= Self::IGNORE_FOR_AURAS;
        *self
    }

    pub fn clear_ignore_for_auras(&mut self) -> Self {
        self.inner &= Self::IGNORE_FOR_AURAS.reverse_bits();
        *self
    }

    pub const fn is_ignore_default_arena_restrictions(&self) -> bool {
        (self.inner & Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS) != 0
    }

    /// Item can be used during arena match
    ///
    pub const fn new_ignore_default_arena_restrictions() -> Self {
        Self { inner: Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS }
    }

    pub fn set_ignore_default_arena_restrictions(&mut self) -> Self {
        self.inner |= Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS;
        *self
    }

    pub fn clear_ignore_default_arena_restrictions(&mut self) -> Self {
        self.inner &= Self::IGNORE_DEFAULT_ARENA_RESTRICTIONS.reverse_bits();
        *self
    }

    pub const fn is_no_durability_loss(&self) -> bool {
        (self.inner & Self::NO_DURABILITY_LOSS) != 0
    }

    /// Some Thrown weapons have it (and only Thrown) but not all
    ///
    pub const fn new_no_durability_loss() -> Self {
        Self { inner: Self::NO_DURABILITY_LOSS }
    }

    pub fn set_no_durability_loss(&mut self) -> Self {
        self.inner |= Self::NO_DURABILITY_LOSS;
        *self
    }

    pub fn clear_no_durability_loss(&mut self) -> Self {
        self.inner &= Self::NO_DURABILITY_LOSS.reverse_bits();
        *self
    }

    pub const fn is_use_when_shapeshifted(&self) -> bool {
        (self.inner & Self::USE_WHEN_SHAPESHIFTED) != 0
    }

    /// Item can be used in shapeshift forms
    ///
    pub const fn new_use_when_shapeshifted() -> Self {
        Self { inner: Self::USE_WHEN_SHAPESHIFTED }
    }

    pub fn set_use_when_shapeshifted(&mut self) -> Self {
        self.inner |= Self::USE_WHEN_SHAPESHIFTED;
        *self
    }

    pub fn clear_use_when_shapeshifted(&mut self) -> Self {
        self.inner &= Self::USE_WHEN_SHAPESHIFTED.reverse_bits();
        *self
    }

    pub const fn is_has_quest_glow(&self) -> bool {
        (self.inner & Self::HAS_QUEST_GLOW) != 0
    }

    pub const fn new_has_quest_glow() -> Self {
        Self { inner: Self::HAS_QUEST_GLOW }
    }

    pub fn set_has_quest_glow(&mut self) -> Self {
        self.inner |= Self::HAS_QUEST_GLOW;
        *self
    }

    pub fn clear_has_quest_glow(&mut self) -> Self {
        self.inner &= Self::HAS_QUEST_GLOW.reverse_bits();
        *self
    }

    pub const fn is_hide_unusable_recipe(&self) -> bool {
        (self.inner & Self::HIDE_UNUSABLE_RECIPE) != 0
    }

    /// Profession recipes: can only be looted if you meet requirements and don't already know it
    ///
    pub const fn new_hide_unusable_recipe() -> Self {
        Self { inner: Self::HIDE_UNUSABLE_RECIPE }
    }

    pub fn set_hide_unusable_recipe(&mut self) -> Self {
        self.inner |= Self::HIDE_UNUSABLE_RECIPE;
        *self
    }

    pub fn clear_hide_unusable_recipe(&mut self) -> Self {
        self.inner &= Self::HIDE_UNUSABLE_RECIPE.reverse_bits();
        *self
    }

    pub const fn is_not_useable_in_arena(&self) -> bool {
        (self.inner & Self::NOT_USEABLE_IN_ARENA) != 0
    }

    /// Item cannot be used in arena
    ///
    pub const fn new_not_useable_in_arena() -> Self {
        Self { inner: Self::NOT_USEABLE_IN_ARENA }
    }

    pub fn set_not_useable_in_arena(&mut self) -> Self {
        self.inner |= Self::NOT_USEABLE_IN_ARENA;
        *self
    }

    pub fn clear_not_useable_in_arena(&mut self) -> Self {
        self.inner &= Self::NOT_USEABLE_IN_ARENA.reverse_bits();
        *self
    }

    pub const fn is_is_bound_to_account(&self) -> bool {
        (self.inner & Self::IS_BOUND_TO_ACCOUNT) != 0
    }

    /// Item binds to account and can be sent only to your own characters
    ///
    pub const fn new_is_bound_to_account() -> Self {
        Self { inner: Self::IS_BOUND_TO_ACCOUNT }
    }

    pub fn set_is_bound_to_account(&mut self) -> Self {
        self.inner |= Self::IS_BOUND_TO_ACCOUNT;
        *self
    }

    pub fn clear_is_bound_to_account(&mut self) -> Self {
        self.inner &= Self::IS_BOUND_TO_ACCOUNT.reverse_bits();
        *self
    }

    pub const fn is_no_reagent_cost(&self) -> bool {
        (self.inner & Self::NO_REAGENT_COST) != 0
    }

    /// Spell is cast ignoring reagents
    ///
    pub const fn new_no_reagent_cost() -> Self {
        Self { inner: Self::NO_REAGENT_COST }
    }

    pub fn set_no_reagent_cost(&mut self) -> Self {
        self.inner |= Self::NO_REAGENT_COST;
        *self
    }

    pub fn clear_no_reagent_cost(&mut self) -> Self {
        self.inner &= Self::NO_REAGENT_COST.reverse_bits();
        *self
    }

    pub const fn is_is_millable(&self) -> bool {
        (self.inner & Self::IS_MILLABLE) != 0
    }

    /// Item can be milled
    ///
    pub const fn new_is_millable() -> Self {
        Self { inner: Self::IS_MILLABLE }
    }

    pub fn set_is_millable(&mut self) -> Self {
        self.inner |= Self::IS_MILLABLE;
        *self
    }

    pub fn clear_is_millable(&mut self) -> Self {
        self.inner &= Self::IS_MILLABLE.reverse_bits();
        *self
    }

    pub const fn is_report_to_guild_chat(&self) -> bool {
        (self.inner & Self::REPORT_TO_GUILD_CHAT) != 0
    }

    pub const fn new_report_to_guild_chat() -> Self {
        Self { inner: Self::REPORT_TO_GUILD_CHAT }
    }

    pub fn set_report_to_guild_chat(&mut self) -> Self {
        self.inner |= Self::REPORT_TO_GUILD_CHAT;
        *self
    }

    pub fn clear_report_to_guild_chat(&mut self) -> Self {
        self.inner &= Self::REPORT_TO_GUILD_CHAT.reverse_bits();
        *self
    }

    pub const fn is_no_progressive_loot(&self) -> bool {
        (self.inner & Self::NO_PROGRESSIVE_LOOT) != 0
    }

    pub const fn new_no_progressive_loot() -> Self {
        Self { inner: Self::NO_PROGRESSIVE_LOOT }
    }

    pub fn set_no_progressive_loot(&mut self) -> Self {
        self.inner |= Self::NO_PROGRESSIVE_LOOT;
        *self
    }

    pub fn clear_no_progressive_loot(&mut self) -> Self {
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

