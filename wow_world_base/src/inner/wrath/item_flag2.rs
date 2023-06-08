/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:531`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L531):
/// ```text
/// flag ItemFlag2 : u32 {
///     FACTION_HORDE = 0x00000001;
///     FACTION_ALLIANCE = 0x00000002;
///     DONT_IGNORE_BUY_PRICE = 0x00000004;
///     CLASSIFY_AS_CASTER = 0x00000008;
///     CLASSIFY_AS_PHYSICAL = 0x00000010;
///     EVERYONE_CAN_ROLL_NEED = 0x00000020;
///     NO_TRADE_BIND_ON_ACQUIRE = 0x00000040;
///     CAN_TRADE_BIND_ON_ACQUIRE = 0x00000080;
///     CAN_ONLY_ROLL_GREED = 0x00000100;
///     CASTER_WEAPON = 0x00000200;
///     DELETE_ON_LOGIN = 0x00000400;
///     INTERNAL_ITEM = 0x00000800;
///     NO_VENDOR_VALUE = 0x00001000;
///     SHOW_BEFORE_DISCOVERED = 0x00002000;
///     OVERRIDE_GOLD_COST = 0x00004000;
///     IGNORE_DEFAULT_RATED_BG_RESTRICTIONS = 0x00008000;
///     NOT_USABLE_IN_RATED_BG = 0x00010000;
///     BNET_ACCOUNT_TRADE_OK = 0x00020000;
///     CONFIRM_BEFORE_USE = 0x00040000;
///     REEVALUATE_BONDING_ON_TRANSFORM = 0x00080000;
///     NO_TRANSFORM_ON_CHARGE_DEPLETION = 0x00100000;
///     NO_ALTER_ITEM_VISUAL = 0x00200000;
///     NO_SOURCE_FOR_ITEM_VISUAL = 0x00400000;
///     IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE = 0x00800000;
///     NO_DURABILITY = 0x01000000;
///     ROLE_TANK = 0x02000000;
///     ROLE_HEALER = 0x04000000;
///     ROLE_DAMAGE = 0x08000000;
///     CAN_DROP_IN_CHALLENGE_MODE = 0x10000000;
///     NEVER_STACK_IN_LOOT_UI = 0x20000000;
///     DISENCHANT_TO_LOOT_TABLE = 0x40000000;
///     USED_IN_A_TRADESKILL = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ItemFlag2 {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl ItemFlag2 {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_faction_horde() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FACTION_HORDE").unwrap();
            first = false;
        }
        if self.is_faction_alliance() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FACTION_ALLIANCE").unwrap();
            first = false;
        }
        if self.is_dont_ignore_buy_price() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DONT_IGNORE_BUY_PRICE").unwrap();
            first = false;
        }
        if self.is_classify_as_caster() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CLASSIFY_AS_CASTER").unwrap();
            first = false;
        }
        if self.is_classify_as_physical() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CLASSIFY_AS_PHYSICAL").unwrap();
            first = false;
        }
        if self.is_everyone_can_roll_need() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "EVERYONE_CAN_ROLL_NEED").unwrap();
            first = false;
        }
        if self.is_no_trade_bind_on_acquire() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_TRADE_BIND_ON_ACQUIRE").unwrap();
            first = false;
        }
        if self.is_can_trade_bind_on_acquire() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CAN_TRADE_BIND_ON_ACQUIRE").unwrap();
            first = false;
        }
        if self.is_can_only_roll_greed() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CAN_ONLY_ROLL_GREED").unwrap();
            first = false;
        }
        if self.is_caster_weapon() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CASTER_WEAPON").unwrap();
            first = false;
        }
        if self.is_delete_on_login() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DELETE_ON_LOGIN").unwrap();
            first = false;
        }
        if self.is_internal_item() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INTERNAL_ITEM").unwrap();
            first = false;
        }
        if self.is_no_vendor_value() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_VENDOR_VALUE").unwrap();
            first = false;
        }
        if self.is_show_before_discovered() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SHOW_BEFORE_DISCOVERED").unwrap();
            first = false;
        }
        if self.is_override_gold_cost() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "OVERRIDE_GOLD_COST").unwrap();
            first = false;
        }
        if self.is_ignore_default_rated_bg_restrictions() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_DEFAULT_RATED_BG_RESTRICTIONS").unwrap();
            first = false;
        }
        if self.is_not_usable_in_rated_bg() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NOT_USABLE_IN_RATED_BG").unwrap();
            first = false;
        }
        if self.is_bnet_account_trade_ok() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "BNET_ACCOUNT_TRADE_OK").unwrap();
            first = false;
        }
        if self.is_confirm_before_use() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CONFIRM_BEFORE_USE").unwrap();
            first = false;
        }
        if self.is_reevaluate_bonding_on_transform() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "REEVALUATE_BONDING_ON_TRANSFORM").unwrap();
            first = false;
        }
        if self.is_no_transform_on_charge_depletion() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_TRANSFORM_ON_CHARGE_DEPLETION").unwrap();
            first = false;
        }
        if self.is_no_alter_item_visual() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_ALTER_ITEM_VISUAL").unwrap();
            first = false;
        }
        if self.is_no_source_for_item_visual() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_SOURCE_FOR_ITEM_VISUAL").unwrap();
            first = false;
        }
        if self.is_ignore_quality_for_item_visual_source() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE").unwrap();
            first = false;
        }
        if self.is_no_durability() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_DURABILITY").unwrap();
            first = false;
        }
        if self.is_role_tank() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ROLE_TANK").unwrap();
            first = false;
        }
        if self.is_role_healer() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ROLE_HEALER").unwrap();
            first = false;
        }
        if self.is_role_damage() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ROLE_DAMAGE").unwrap();
            first = false;
        }
        if self.is_can_drop_in_challenge_mode() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CAN_DROP_IN_CHALLENGE_MODE").unwrap();
            first = false;
        }
        if self.is_never_stack_in_loot_ui() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NEVER_STACK_IN_LOOT_UI").unwrap();
            first = false;
        }
        if self.is_disenchant_to_loot_table() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DISENCHANT_TO_LOOT_TABLE").unwrap();
            first = false;
        }
        if self.is_used_in_a_tradeskill() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "USED_IN_A_TRADESKILL").unwrap();
            first = false;
        }
        s
    }

}

impl ItemFlag2 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const FACTION_HORDE: u32 = 0x01;
    pub const FACTION_ALLIANCE: u32 = 0x02;
    pub const DONT_IGNORE_BUY_PRICE: u32 = 0x04;
    pub const CLASSIFY_AS_CASTER: u32 = 0x08;
    pub const CLASSIFY_AS_PHYSICAL: u32 = 0x10;
    pub const EVERYONE_CAN_ROLL_NEED: u32 = 0x20;
    pub const NO_TRADE_BIND_ON_ACQUIRE: u32 = 0x40;
    pub const CAN_TRADE_BIND_ON_ACQUIRE: u32 = 0x80;
    pub const CAN_ONLY_ROLL_GREED: u32 = 0x100;
    pub const CASTER_WEAPON: u32 = 0x200;
    pub const DELETE_ON_LOGIN: u32 = 0x400;
    pub const INTERNAL_ITEM: u32 = 0x800;
    pub const NO_VENDOR_VALUE: u32 = 0x1000;
    pub const SHOW_BEFORE_DISCOVERED: u32 = 0x2000;
    pub const OVERRIDE_GOLD_COST: u32 = 0x4000;
    pub const IGNORE_DEFAULT_RATED_BG_RESTRICTIONS: u32 = 0x8000;
    pub const NOT_USABLE_IN_RATED_BG: u32 = 0x10000;
    pub const BNET_ACCOUNT_TRADE_OK: u32 = 0x20000;
    pub const CONFIRM_BEFORE_USE: u32 = 0x40000;
    pub const REEVALUATE_BONDING_ON_TRANSFORM: u32 = 0x80000;
    pub const NO_TRANSFORM_ON_CHARGE_DEPLETION: u32 = 0x100000;
    pub const NO_ALTER_ITEM_VISUAL: u32 = 0x200000;
    pub const NO_SOURCE_FOR_ITEM_VISUAL: u32 = 0x400000;
    pub const IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE: u32 = 0x800000;
    pub const NO_DURABILITY: u32 = 0x1000000;
    pub const ROLE_TANK: u32 = 0x2000000;
    pub const ROLE_HEALER: u32 = 0x4000000;
    pub const ROLE_DAMAGE: u32 = 0x8000000;
    pub const CAN_DROP_IN_CHALLENGE_MODE: u32 = 0x10000000;
    pub const NEVER_STACK_IN_LOOT_UI: u32 = 0x20000000;
    pub const DISENCHANT_TO_LOOT_TABLE: u32 = 0x40000000;
    pub const USED_IN_A_TRADESKILL: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::FACTION_HORDE
                | Self::FACTION_ALLIANCE
                | Self::DONT_IGNORE_BUY_PRICE
                | Self::CLASSIFY_AS_CASTER
                | Self::CLASSIFY_AS_PHYSICAL
                | Self::EVERYONE_CAN_ROLL_NEED
                | Self::NO_TRADE_BIND_ON_ACQUIRE
                | Self::CAN_TRADE_BIND_ON_ACQUIRE
                | Self::CAN_ONLY_ROLL_GREED
                | Self::CASTER_WEAPON
                | Self::DELETE_ON_LOGIN
                | Self::INTERNAL_ITEM
                | Self::NO_VENDOR_VALUE
                | Self::SHOW_BEFORE_DISCOVERED
                | Self::OVERRIDE_GOLD_COST
                | Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS
                | Self::NOT_USABLE_IN_RATED_BG
                | Self::BNET_ACCOUNT_TRADE_OK
                | Self::CONFIRM_BEFORE_USE
                | Self::REEVALUATE_BONDING_ON_TRANSFORM
                | Self::NO_TRANSFORM_ON_CHARGE_DEPLETION
                | Self::NO_ALTER_ITEM_VISUAL
                | Self::NO_SOURCE_FOR_ITEM_VISUAL
                | Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE
                | Self::NO_DURABILITY
                | Self::ROLE_TANK
                | Self::ROLE_HEALER
                | Self::ROLE_DAMAGE
                | Self::CAN_DROP_IN_CHALLENGE_MODE
                | Self::NEVER_STACK_IN_LOOT_UI
                | Self::DISENCHANT_TO_LOOT_TABLE
                | Self::USED_IN_A_TRADESKILL
        }
    }

    pub const fn is_faction_horde(&self) -> bool {
        (self.inner & Self::FACTION_HORDE) != 0
    }

    pub const fn new_faction_horde() -> Self {
        Self { inner: Self::FACTION_HORDE }
    }

    pub fn set_faction_horde(&mut self) -> Self {
        self.inner |= Self::FACTION_HORDE;
        *self
    }

    pub fn clear_faction_horde(&mut self) -> Self {
        self.inner &= Self::FACTION_HORDE.reverse_bits();
        *self
    }

    pub const fn is_faction_alliance(&self) -> bool {
        (self.inner & Self::FACTION_ALLIANCE) != 0
    }

    pub const fn new_faction_alliance() -> Self {
        Self { inner: Self::FACTION_ALLIANCE }
    }

    pub fn set_faction_alliance(&mut self) -> Self {
        self.inner |= Self::FACTION_ALLIANCE;
        *self
    }

    pub fn clear_faction_alliance(&mut self) -> Self {
        self.inner &= Self::FACTION_ALLIANCE.reverse_bits();
        *self
    }

    pub const fn is_dont_ignore_buy_price(&self) -> bool {
        (self.inner & Self::DONT_IGNORE_BUY_PRICE) != 0
    }

    /// when item uses extended cost, gold is also required
    pub const fn new_dont_ignore_buy_price() -> Self {
        Self { inner: Self::DONT_IGNORE_BUY_PRICE }
    }

    pub fn set_dont_ignore_buy_price(&mut self) -> Self {
        self.inner |= Self::DONT_IGNORE_BUY_PRICE;
        *self
    }

    pub fn clear_dont_ignore_buy_price(&mut self) -> Self {
        self.inner &= Self::DONT_IGNORE_BUY_PRICE.reverse_bits();
        *self
    }

    pub const fn is_classify_as_caster(&self) -> bool {
        (self.inner & Self::CLASSIFY_AS_CASTER) != 0
    }

    pub const fn new_classify_as_caster() -> Self {
        Self { inner: Self::CLASSIFY_AS_CASTER }
    }

    pub fn set_classify_as_caster(&mut self) -> Self {
        self.inner |= Self::CLASSIFY_AS_CASTER;
        *self
    }

    pub fn clear_classify_as_caster(&mut self) -> Self {
        self.inner &= Self::CLASSIFY_AS_CASTER.reverse_bits();
        *self
    }

    pub const fn is_classify_as_physical(&self) -> bool {
        (self.inner & Self::CLASSIFY_AS_PHYSICAL) != 0
    }

    pub const fn new_classify_as_physical() -> Self {
        Self { inner: Self::CLASSIFY_AS_PHYSICAL }
    }

    pub fn set_classify_as_physical(&mut self) -> Self {
        self.inner |= Self::CLASSIFY_AS_PHYSICAL;
        *self
    }

    pub fn clear_classify_as_physical(&mut self) -> Self {
        self.inner &= Self::CLASSIFY_AS_PHYSICAL.reverse_bits();
        *self
    }

    pub const fn is_everyone_can_roll_need(&self) -> bool {
        (self.inner & Self::EVERYONE_CAN_ROLL_NEED) != 0
    }

    pub const fn new_everyone_can_roll_need() -> Self {
        Self { inner: Self::EVERYONE_CAN_ROLL_NEED }
    }

    pub fn set_everyone_can_roll_need(&mut self) -> Self {
        self.inner |= Self::EVERYONE_CAN_ROLL_NEED;
        *self
    }

    pub fn clear_everyone_can_roll_need(&mut self) -> Self {
        self.inner &= Self::EVERYONE_CAN_ROLL_NEED.reverse_bits();
        *self
    }

    pub const fn is_no_trade_bind_on_acquire(&self) -> bool {
        (self.inner & Self::NO_TRADE_BIND_ON_ACQUIRE) != 0
    }

    pub const fn new_no_trade_bind_on_acquire() -> Self {
        Self { inner: Self::NO_TRADE_BIND_ON_ACQUIRE }
    }

    pub fn set_no_trade_bind_on_acquire(&mut self) -> Self {
        self.inner |= Self::NO_TRADE_BIND_ON_ACQUIRE;
        *self
    }

    pub fn clear_no_trade_bind_on_acquire(&mut self) -> Self {
        self.inner &= Self::NO_TRADE_BIND_ON_ACQUIRE.reverse_bits();
        *self
    }

    pub const fn is_can_trade_bind_on_acquire(&self) -> bool {
        (self.inner & Self::CAN_TRADE_BIND_ON_ACQUIRE) != 0
    }

    pub const fn new_can_trade_bind_on_acquire() -> Self {
        Self { inner: Self::CAN_TRADE_BIND_ON_ACQUIRE }
    }

    pub fn set_can_trade_bind_on_acquire(&mut self) -> Self {
        self.inner |= Self::CAN_TRADE_BIND_ON_ACQUIRE;
        *self
    }

    pub fn clear_can_trade_bind_on_acquire(&mut self) -> Self {
        self.inner &= Self::CAN_TRADE_BIND_ON_ACQUIRE.reverse_bits();
        *self
    }

    pub const fn is_can_only_roll_greed(&self) -> bool {
        (self.inner & Self::CAN_ONLY_ROLL_GREED) != 0
    }

    pub const fn new_can_only_roll_greed() -> Self {
        Self { inner: Self::CAN_ONLY_ROLL_GREED }
    }

    pub fn set_can_only_roll_greed(&mut self) -> Self {
        self.inner |= Self::CAN_ONLY_ROLL_GREED;
        *self
    }

    pub fn clear_can_only_roll_greed(&mut self) -> Self {
        self.inner &= Self::CAN_ONLY_ROLL_GREED.reverse_bits();
        *self
    }

    pub const fn is_caster_weapon(&self) -> bool {
        (self.inner & Self::CASTER_WEAPON) != 0
    }

    pub const fn new_caster_weapon() -> Self {
        Self { inner: Self::CASTER_WEAPON }
    }

    pub fn set_caster_weapon(&mut self) -> Self {
        self.inner |= Self::CASTER_WEAPON;
        *self
    }

    pub fn clear_caster_weapon(&mut self) -> Self {
        self.inner &= Self::CASTER_WEAPON.reverse_bits();
        *self
    }

    pub const fn is_delete_on_login(&self) -> bool {
        (self.inner & Self::DELETE_ON_LOGIN) != 0
    }

    pub const fn new_delete_on_login() -> Self {
        Self { inner: Self::DELETE_ON_LOGIN }
    }

    pub fn set_delete_on_login(&mut self) -> Self {
        self.inner |= Self::DELETE_ON_LOGIN;
        *self
    }

    pub fn clear_delete_on_login(&mut self) -> Self {
        self.inner &= Self::DELETE_ON_LOGIN.reverse_bits();
        *self
    }

    pub const fn is_internal_item(&self) -> bool {
        (self.inner & Self::INTERNAL_ITEM) != 0
    }

    pub const fn new_internal_item() -> Self {
        Self { inner: Self::INTERNAL_ITEM }
    }

    pub fn set_internal_item(&mut self) -> Self {
        self.inner |= Self::INTERNAL_ITEM;
        *self
    }

    pub fn clear_internal_item(&mut self) -> Self {
        self.inner &= Self::INTERNAL_ITEM.reverse_bits();
        *self
    }

    pub const fn is_no_vendor_value(&self) -> bool {
        (self.inner & Self::NO_VENDOR_VALUE) != 0
    }

    pub const fn new_no_vendor_value() -> Self {
        Self { inner: Self::NO_VENDOR_VALUE }
    }

    pub fn set_no_vendor_value(&mut self) -> Self {
        self.inner |= Self::NO_VENDOR_VALUE;
        *self
    }

    pub fn clear_no_vendor_value(&mut self) -> Self {
        self.inner &= Self::NO_VENDOR_VALUE.reverse_bits();
        *self
    }

    pub const fn is_show_before_discovered(&self) -> bool {
        (self.inner & Self::SHOW_BEFORE_DISCOVERED) != 0
    }

    pub const fn new_show_before_discovered() -> Self {
        Self { inner: Self::SHOW_BEFORE_DISCOVERED }
    }

    pub fn set_show_before_discovered(&mut self) -> Self {
        self.inner |= Self::SHOW_BEFORE_DISCOVERED;
        *self
    }

    pub fn clear_show_before_discovered(&mut self) -> Self {
        self.inner &= Self::SHOW_BEFORE_DISCOVERED.reverse_bits();
        *self
    }

    pub const fn is_override_gold_cost(&self) -> bool {
        (self.inner & Self::OVERRIDE_GOLD_COST) != 0
    }

    pub const fn new_override_gold_cost() -> Self {
        Self { inner: Self::OVERRIDE_GOLD_COST }
    }

    pub fn set_override_gold_cost(&mut self) -> Self {
        self.inner |= Self::OVERRIDE_GOLD_COST;
        *self
    }

    pub fn clear_override_gold_cost(&mut self) -> Self {
        self.inner &= Self::OVERRIDE_GOLD_COST.reverse_bits();
        *self
    }

    pub const fn is_ignore_default_rated_bg_restrictions(&self) -> bool {
        (self.inner & Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS) != 0
    }

    pub const fn new_ignore_default_rated_bg_restrictions() -> Self {
        Self { inner: Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS }
    }

    pub fn set_ignore_default_rated_bg_restrictions(&mut self) -> Self {
        self.inner |= Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS;
        *self
    }

    pub fn clear_ignore_default_rated_bg_restrictions(&mut self) -> Self {
        self.inner &= Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS.reverse_bits();
        *self
    }

    pub const fn is_not_usable_in_rated_bg(&self) -> bool {
        (self.inner & Self::NOT_USABLE_IN_RATED_BG) != 0
    }

    pub const fn new_not_usable_in_rated_bg() -> Self {
        Self { inner: Self::NOT_USABLE_IN_RATED_BG }
    }

    pub fn set_not_usable_in_rated_bg(&mut self) -> Self {
        self.inner |= Self::NOT_USABLE_IN_RATED_BG;
        *self
    }

    pub fn clear_not_usable_in_rated_bg(&mut self) -> Self {
        self.inner &= Self::NOT_USABLE_IN_RATED_BG.reverse_bits();
        *self
    }

    pub const fn is_bnet_account_trade_ok(&self) -> bool {
        (self.inner & Self::BNET_ACCOUNT_TRADE_OK) != 0
    }

    pub const fn new_bnet_account_trade_ok() -> Self {
        Self { inner: Self::BNET_ACCOUNT_TRADE_OK }
    }

    pub fn set_bnet_account_trade_ok(&mut self) -> Self {
        self.inner |= Self::BNET_ACCOUNT_TRADE_OK;
        *self
    }

    pub fn clear_bnet_account_trade_ok(&mut self) -> Self {
        self.inner &= Self::BNET_ACCOUNT_TRADE_OK.reverse_bits();
        *self
    }

    pub const fn is_confirm_before_use(&self) -> bool {
        (self.inner & Self::CONFIRM_BEFORE_USE) != 0
    }

    pub const fn new_confirm_before_use() -> Self {
        Self { inner: Self::CONFIRM_BEFORE_USE }
    }

    pub fn set_confirm_before_use(&mut self) -> Self {
        self.inner |= Self::CONFIRM_BEFORE_USE;
        *self
    }

    pub fn clear_confirm_before_use(&mut self) -> Self {
        self.inner &= Self::CONFIRM_BEFORE_USE.reverse_bits();
        *self
    }

    pub const fn is_reevaluate_bonding_on_transform(&self) -> bool {
        (self.inner & Self::REEVALUATE_BONDING_ON_TRANSFORM) != 0
    }

    pub const fn new_reevaluate_bonding_on_transform() -> Self {
        Self { inner: Self::REEVALUATE_BONDING_ON_TRANSFORM }
    }

    pub fn set_reevaluate_bonding_on_transform(&mut self) -> Self {
        self.inner |= Self::REEVALUATE_BONDING_ON_TRANSFORM;
        *self
    }

    pub fn clear_reevaluate_bonding_on_transform(&mut self) -> Self {
        self.inner &= Self::REEVALUATE_BONDING_ON_TRANSFORM.reverse_bits();
        *self
    }

    pub const fn is_no_transform_on_charge_depletion(&self) -> bool {
        (self.inner & Self::NO_TRANSFORM_ON_CHARGE_DEPLETION) != 0
    }

    pub const fn new_no_transform_on_charge_depletion() -> Self {
        Self { inner: Self::NO_TRANSFORM_ON_CHARGE_DEPLETION }
    }

    pub fn set_no_transform_on_charge_depletion(&mut self) -> Self {
        self.inner |= Self::NO_TRANSFORM_ON_CHARGE_DEPLETION;
        *self
    }

    pub fn clear_no_transform_on_charge_depletion(&mut self) -> Self {
        self.inner &= Self::NO_TRANSFORM_ON_CHARGE_DEPLETION.reverse_bits();
        *self
    }

    pub const fn is_no_alter_item_visual(&self) -> bool {
        (self.inner & Self::NO_ALTER_ITEM_VISUAL) != 0
    }

    pub const fn new_no_alter_item_visual() -> Self {
        Self { inner: Self::NO_ALTER_ITEM_VISUAL }
    }

    pub fn set_no_alter_item_visual(&mut self) -> Self {
        self.inner |= Self::NO_ALTER_ITEM_VISUAL;
        *self
    }

    pub fn clear_no_alter_item_visual(&mut self) -> Self {
        self.inner &= Self::NO_ALTER_ITEM_VISUAL.reverse_bits();
        *self
    }

    pub const fn is_no_source_for_item_visual(&self) -> bool {
        (self.inner & Self::NO_SOURCE_FOR_ITEM_VISUAL) != 0
    }

    pub const fn new_no_source_for_item_visual() -> Self {
        Self { inner: Self::NO_SOURCE_FOR_ITEM_VISUAL }
    }

    pub fn set_no_source_for_item_visual(&mut self) -> Self {
        self.inner |= Self::NO_SOURCE_FOR_ITEM_VISUAL;
        *self
    }

    pub fn clear_no_source_for_item_visual(&mut self) -> Self {
        self.inner &= Self::NO_SOURCE_FOR_ITEM_VISUAL.reverse_bits();
        *self
    }

    pub const fn is_ignore_quality_for_item_visual_source(&self) -> bool {
        (self.inner & Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE) != 0
    }

    pub const fn new_ignore_quality_for_item_visual_source() -> Self {
        Self { inner: Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE }
    }

    pub fn set_ignore_quality_for_item_visual_source(&mut self) -> Self {
        self.inner |= Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE;
        *self
    }

    pub fn clear_ignore_quality_for_item_visual_source(&mut self) -> Self {
        self.inner &= Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE.reverse_bits();
        *self
    }

    pub const fn is_no_durability(&self) -> bool {
        (self.inner & Self::NO_DURABILITY) != 0
    }

    pub const fn new_no_durability() -> Self {
        Self { inner: Self::NO_DURABILITY }
    }

    pub fn set_no_durability(&mut self) -> Self {
        self.inner |= Self::NO_DURABILITY;
        *self
    }

    pub fn clear_no_durability(&mut self) -> Self {
        self.inner &= Self::NO_DURABILITY.reverse_bits();
        *self
    }

    pub const fn is_role_tank(&self) -> bool {
        (self.inner & Self::ROLE_TANK) != 0
    }

    pub const fn new_role_tank() -> Self {
        Self { inner: Self::ROLE_TANK }
    }

    pub fn set_role_tank(&mut self) -> Self {
        self.inner |= Self::ROLE_TANK;
        *self
    }

    pub fn clear_role_tank(&mut self) -> Self {
        self.inner &= Self::ROLE_TANK.reverse_bits();
        *self
    }

    pub const fn is_role_healer(&self) -> bool {
        (self.inner & Self::ROLE_HEALER) != 0
    }

    pub const fn new_role_healer() -> Self {
        Self { inner: Self::ROLE_HEALER }
    }

    pub fn set_role_healer(&mut self) -> Self {
        self.inner |= Self::ROLE_HEALER;
        *self
    }

    pub fn clear_role_healer(&mut self) -> Self {
        self.inner &= Self::ROLE_HEALER.reverse_bits();
        *self
    }

    pub const fn is_role_damage(&self) -> bool {
        (self.inner & Self::ROLE_DAMAGE) != 0
    }

    pub const fn new_role_damage() -> Self {
        Self { inner: Self::ROLE_DAMAGE }
    }

    pub fn set_role_damage(&mut self) -> Self {
        self.inner |= Self::ROLE_DAMAGE;
        *self
    }

    pub fn clear_role_damage(&mut self) -> Self {
        self.inner &= Self::ROLE_DAMAGE.reverse_bits();
        *self
    }

    pub const fn is_can_drop_in_challenge_mode(&self) -> bool {
        (self.inner & Self::CAN_DROP_IN_CHALLENGE_MODE) != 0
    }

    pub const fn new_can_drop_in_challenge_mode() -> Self {
        Self { inner: Self::CAN_DROP_IN_CHALLENGE_MODE }
    }

    pub fn set_can_drop_in_challenge_mode(&mut self) -> Self {
        self.inner |= Self::CAN_DROP_IN_CHALLENGE_MODE;
        *self
    }

    pub fn clear_can_drop_in_challenge_mode(&mut self) -> Self {
        self.inner &= Self::CAN_DROP_IN_CHALLENGE_MODE.reverse_bits();
        *self
    }

    pub const fn is_never_stack_in_loot_ui(&self) -> bool {
        (self.inner & Self::NEVER_STACK_IN_LOOT_UI) != 0
    }

    pub const fn new_never_stack_in_loot_ui() -> Self {
        Self { inner: Self::NEVER_STACK_IN_LOOT_UI }
    }

    pub fn set_never_stack_in_loot_ui(&mut self) -> Self {
        self.inner |= Self::NEVER_STACK_IN_LOOT_UI;
        *self
    }

    pub fn clear_never_stack_in_loot_ui(&mut self) -> Self {
        self.inner &= Self::NEVER_STACK_IN_LOOT_UI.reverse_bits();
        *self
    }

    pub const fn is_disenchant_to_loot_table(&self) -> bool {
        (self.inner & Self::DISENCHANT_TO_LOOT_TABLE) != 0
    }

    pub const fn new_disenchant_to_loot_table() -> Self {
        Self { inner: Self::DISENCHANT_TO_LOOT_TABLE }
    }

    pub fn set_disenchant_to_loot_table(&mut self) -> Self {
        self.inner |= Self::DISENCHANT_TO_LOOT_TABLE;
        *self
    }

    pub fn clear_disenchant_to_loot_table(&mut self) -> Self {
        self.inner &= Self::DISENCHANT_TO_LOOT_TABLE.reverse_bits();
        *self
    }

    pub const fn is_used_in_a_tradeskill(&self) -> bool {
        (self.inner & Self::USED_IN_A_TRADESKILL) != 0
    }

    pub const fn new_used_in_a_tradeskill() -> Self {
        Self { inner: Self::USED_IN_A_TRADESKILL }
    }

    pub fn set_used_in_a_tradeskill(&mut self) -> Self {
        self.inner |= Self::USED_IN_A_TRADESKILL;
        *self
    }

    pub fn clear_used_in_a_tradeskill(&mut self) -> Self {
        self.inner &= Self::USED_IN_A_TRADESKILL.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for ItemFlag2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for ItemFlag2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for ItemFlag2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for ItemFlag2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for ItemFlag2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for ItemFlag2 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for ItemFlag2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for ItemFlag2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for ItemFlag2 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for ItemFlag2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

