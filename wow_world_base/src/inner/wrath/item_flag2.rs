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

impl ItemFlag2 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const FACTION_HORDE: u32 = 0x01;
    pub(crate) const FACTION_ALLIANCE: u32 = 0x02;
    pub(crate) const DONT_IGNORE_BUY_PRICE: u32 = 0x04;
    pub(crate) const CLASSIFY_AS_CASTER: u32 = 0x08;
    pub(crate) const CLASSIFY_AS_PHYSICAL: u32 = 0x10;
    pub(crate) const EVERYONE_CAN_ROLL_NEED: u32 = 0x20;
    pub(crate) const NO_TRADE_BIND_ON_ACQUIRE: u32 = 0x40;
    pub(crate) const CAN_TRADE_BIND_ON_ACQUIRE: u32 = 0x80;
    pub(crate) const CAN_ONLY_ROLL_GREED: u32 = 0x100;
    pub(crate) const CASTER_WEAPON: u32 = 0x200;
    pub(crate) const DELETE_ON_LOGIN: u32 = 0x400;
    pub(crate) const INTERNAL_ITEM: u32 = 0x800;
    pub(crate) const NO_VENDOR_VALUE: u32 = 0x1000;
    pub(crate) const SHOW_BEFORE_DISCOVERED: u32 = 0x2000;
    pub(crate) const OVERRIDE_GOLD_COST: u32 = 0x4000;
    pub(crate) const IGNORE_DEFAULT_RATED_BG_RESTRICTIONS: u32 = 0x8000;
    pub(crate) const NOT_USABLE_IN_RATED_BG: u32 = 0x10000;
    pub(crate) const BNET_ACCOUNT_TRADE_OK: u32 = 0x20000;
    pub(crate) const CONFIRM_BEFORE_USE: u32 = 0x40000;
    pub(crate) const REEVALUATE_BONDING_ON_TRANSFORM: u32 = 0x80000;
    pub(crate) const NO_TRANSFORM_ON_CHARGE_DEPLETION: u32 = 0x100000;
    pub(crate) const NO_ALTER_ITEM_VISUAL: u32 = 0x200000;
    pub(crate) const NO_SOURCE_FOR_ITEM_VISUAL: u32 = 0x400000;
    pub(crate) const IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE: u32 = 0x800000;
    pub(crate) const NO_DURABILITY: u32 = 0x1000000;
    pub(crate) const ROLE_TANK: u32 = 0x2000000;
    pub(crate) const ROLE_HEALER: u32 = 0x4000000;
    pub(crate) const ROLE_DAMAGE: u32 = 0x8000000;
    pub(crate) const CAN_DROP_IN_CHALLENGE_MODE: u32 = 0x10000000;
    pub(crate) const NEVER_STACK_IN_LOOT_UI: u32 = 0x20000000;
    pub(crate) const DISENCHANT_TO_LOOT_TABLE: u32 = 0x40000000;
    pub(crate) const USED_IN_A_TRADESKILL: u32 = 0x80000000;

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

    pub const fn is_FACTION_HORDE(&self) -> bool {
        (self.inner & Self::FACTION_HORDE) != 0
    }

    pub const fn new_FACTION_HORDE() -> Self {
        Self { inner: Self::FACTION_HORDE }
    }

    pub fn set_FACTION_HORDE(&mut self) -> Self {
        self.inner |= Self::FACTION_HORDE;
        *self
    }

    pub fn clear_FACTION_HORDE(&mut self) -> Self {
        self.inner &= Self::FACTION_HORDE.reverse_bits();
        *self
    }

    pub const fn is_FACTION_ALLIANCE(&self) -> bool {
        (self.inner & Self::FACTION_ALLIANCE) != 0
    }

    pub const fn new_FACTION_ALLIANCE() -> Self {
        Self { inner: Self::FACTION_ALLIANCE }
    }

    pub fn set_FACTION_ALLIANCE(&mut self) -> Self {
        self.inner |= Self::FACTION_ALLIANCE;
        *self
    }

    pub fn clear_FACTION_ALLIANCE(&mut self) -> Self {
        self.inner &= Self::FACTION_ALLIANCE.reverse_bits();
        *self
    }

    pub const fn is_DONT_IGNORE_BUY_PRICE(&self) -> bool {
        (self.inner & Self::DONT_IGNORE_BUY_PRICE) != 0
    }

    /// when item uses extended cost, gold is also required
    ///
    pub const fn new_DONT_IGNORE_BUY_PRICE() -> Self {
        Self { inner: Self::DONT_IGNORE_BUY_PRICE }
    }

    pub fn set_DONT_IGNORE_BUY_PRICE(&mut self) -> Self {
        self.inner |= Self::DONT_IGNORE_BUY_PRICE;
        *self
    }

    pub fn clear_DONT_IGNORE_BUY_PRICE(&mut self) -> Self {
        self.inner &= Self::DONT_IGNORE_BUY_PRICE.reverse_bits();
        *self
    }

    pub const fn is_CLASSIFY_AS_CASTER(&self) -> bool {
        (self.inner & Self::CLASSIFY_AS_CASTER) != 0
    }

    pub const fn new_CLASSIFY_AS_CASTER() -> Self {
        Self { inner: Self::CLASSIFY_AS_CASTER }
    }

    pub fn set_CLASSIFY_AS_CASTER(&mut self) -> Self {
        self.inner |= Self::CLASSIFY_AS_CASTER;
        *self
    }

    pub fn clear_CLASSIFY_AS_CASTER(&mut self) -> Self {
        self.inner &= Self::CLASSIFY_AS_CASTER.reverse_bits();
        *self
    }

    pub const fn is_CLASSIFY_AS_PHYSICAL(&self) -> bool {
        (self.inner & Self::CLASSIFY_AS_PHYSICAL) != 0
    }

    pub const fn new_CLASSIFY_AS_PHYSICAL() -> Self {
        Self { inner: Self::CLASSIFY_AS_PHYSICAL }
    }

    pub fn set_CLASSIFY_AS_PHYSICAL(&mut self) -> Self {
        self.inner |= Self::CLASSIFY_AS_PHYSICAL;
        *self
    }

    pub fn clear_CLASSIFY_AS_PHYSICAL(&mut self) -> Self {
        self.inner &= Self::CLASSIFY_AS_PHYSICAL.reverse_bits();
        *self
    }

    pub const fn is_EVERYONE_CAN_ROLL_NEED(&self) -> bool {
        (self.inner & Self::EVERYONE_CAN_ROLL_NEED) != 0
    }

    pub const fn new_EVERYONE_CAN_ROLL_NEED() -> Self {
        Self { inner: Self::EVERYONE_CAN_ROLL_NEED }
    }

    pub fn set_EVERYONE_CAN_ROLL_NEED(&mut self) -> Self {
        self.inner |= Self::EVERYONE_CAN_ROLL_NEED;
        *self
    }

    pub fn clear_EVERYONE_CAN_ROLL_NEED(&mut self) -> Self {
        self.inner &= Self::EVERYONE_CAN_ROLL_NEED.reverse_bits();
        *self
    }

    pub const fn is_NO_TRADE_BIND_ON_ACQUIRE(&self) -> bool {
        (self.inner & Self::NO_TRADE_BIND_ON_ACQUIRE) != 0
    }

    pub const fn new_NO_TRADE_BIND_ON_ACQUIRE() -> Self {
        Self { inner: Self::NO_TRADE_BIND_ON_ACQUIRE }
    }

    pub fn set_NO_TRADE_BIND_ON_ACQUIRE(&mut self) -> Self {
        self.inner |= Self::NO_TRADE_BIND_ON_ACQUIRE;
        *self
    }

    pub fn clear_NO_TRADE_BIND_ON_ACQUIRE(&mut self) -> Self {
        self.inner &= Self::NO_TRADE_BIND_ON_ACQUIRE.reverse_bits();
        *self
    }

    pub const fn is_CAN_TRADE_BIND_ON_ACQUIRE(&self) -> bool {
        (self.inner & Self::CAN_TRADE_BIND_ON_ACQUIRE) != 0
    }

    pub const fn new_CAN_TRADE_BIND_ON_ACQUIRE() -> Self {
        Self { inner: Self::CAN_TRADE_BIND_ON_ACQUIRE }
    }

    pub fn set_CAN_TRADE_BIND_ON_ACQUIRE(&mut self) -> Self {
        self.inner |= Self::CAN_TRADE_BIND_ON_ACQUIRE;
        *self
    }

    pub fn clear_CAN_TRADE_BIND_ON_ACQUIRE(&mut self) -> Self {
        self.inner &= Self::CAN_TRADE_BIND_ON_ACQUIRE.reverse_bits();
        *self
    }

    pub const fn is_CAN_ONLY_ROLL_GREED(&self) -> bool {
        (self.inner & Self::CAN_ONLY_ROLL_GREED) != 0
    }

    pub const fn new_CAN_ONLY_ROLL_GREED() -> Self {
        Self { inner: Self::CAN_ONLY_ROLL_GREED }
    }

    pub fn set_CAN_ONLY_ROLL_GREED(&mut self) -> Self {
        self.inner |= Self::CAN_ONLY_ROLL_GREED;
        *self
    }

    pub fn clear_CAN_ONLY_ROLL_GREED(&mut self) -> Self {
        self.inner &= Self::CAN_ONLY_ROLL_GREED.reverse_bits();
        *self
    }

    pub const fn is_CASTER_WEAPON(&self) -> bool {
        (self.inner & Self::CASTER_WEAPON) != 0
    }

    pub const fn new_CASTER_WEAPON() -> Self {
        Self { inner: Self::CASTER_WEAPON }
    }

    pub fn set_CASTER_WEAPON(&mut self) -> Self {
        self.inner |= Self::CASTER_WEAPON;
        *self
    }

    pub fn clear_CASTER_WEAPON(&mut self) -> Self {
        self.inner &= Self::CASTER_WEAPON.reverse_bits();
        *self
    }

    pub const fn is_DELETE_ON_LOGIN(&self) -> bool {
        (self.inner & Self::DELETE_ON_LOGIN) != 0
    }

    pub const fn new_DELETE_ON_LOGIN() -> Self {
        Self { inner: Self::DELETE_ON_LOGIN }
    }

    pub fn set_DELETE_ON_LOGIN(&mut self) -> Self {
        self.inner |= Self::DELETE_ON_LOGIN;
        *self
    }

    pub fn clear_DELETE_ON_LOGIN(&mut self) -> Self {
        self.inner &= Self::DELETE_ON_LOGIN.reverse_bits();
        *self
    }

    pub const fn is_INTERNAL_ITEM(&self) -> bool {
        (self.inner & Self::INTERNAL_ITEM) != 0
    }

    pub const fn new_INTERNAL_ITEM() -> Self {
        Self { inner: Self::INTERNAL_ITEM }
    }

    pub fn set_INTERNAL_ITEM(&mut self) -> Self {
        self.inner |= Self::INTERNAL_ITEM;
        *self
    }

    pub fn clear_INTERNAL_ITEM(&mut self) -> Self {
        self.inner &= Self::INTERNAL_ITEM.reverse_bits();
        *self
    }

    pub const fn is_NO_VENDOR_VALUE(&self) -> bool {
        (self.inner & Self::NO_VENDOR_VALUE) != 0
    }

    pub const fn new_NO_VENDOR_VALUE() -> Self {
        Self { inner: Self::NO_VENDOR_VALUE }
    }

    pub fn set_NO_VENDOR_VALUE(&mut self) -> Self {
        self.inner |= Self::NO_VENDOR_VALUE;
        *self
    }

    pub fn clear_NO_VENDOR_VALUE(&mut self) -> Self {
        self.inner &= Self::NO_VENDOR_VALUE.reverse_bits();
        *self
    }

    pub const fn is_SHOW_BEFORE_DISCOVERED(&self) -> bool {
        (self.inner & Self::SHOW_BEFORE_DISCOVERED) != 0
    }

    pub const fn new_SHOW_BEFORE_DISCOVERED() -> Self {
        Self { inner: Self::SHOW_BEFORE_DISCOVERED }
    }

    pub fn set_SHOW_BEFORE_DISCOVERED(&mut self) -> Self {
        self.inner |= Self::SHOW_BEFORE_DISCOVERED;
        *self
    }

    pub fn clear_SHOW_BEFORE_DISCOVERED(&mut self) -> Self {
        self.inner &= Self::SHOW_BEFORE_DISCOVERED.reverse_bits();
        *self
    }

    pub const fn is_OVERRIDE_GOLD_COST(&self) -> bool {
        (self.inner & Self::OVERRIDE_GOLD_COST) != 0
    }

    pub const fn new_OVERRIDE_GOLD_COST() -> Self {
        Self { inner: Self::OVERRIDE_GOLD_COST }
    }

    pub fn set_OVERRIDE_GOLD_COST(&mut self) -> Self {
        self.inner |= Self::OVERRIDE_GOLD_COST;
        *self
    }

    pub fn clear_OVERRIDE_GOLD_COST(&mut self) -> Self {
        self.inner &= Self::OVERRIDE_GOLD_COST.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_DEFAULT_RATED_BG_RESTRICTIONS(&self) -> bool {
        (self.inner & Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS) != 0
    }

    pub const fn new_IGNORE_DEFAULT_RATED_BG_RESTRICTIONS() -> Self {
        Self { inner: Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS }
    }

    pub fn set_IGNORE_DEFAULT_RATED_BG_RESTRICTIONS(&mut self) -> Self {
        self.inner |= Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS;
        *self
    }

    pub fn clear_IGNORE_DEFAULT_RATED_BG_RESTRICTIONS(&mut self) -> Self {
        self.inner &= Self::IGNORE_DEFAULT_RATED_BG_RESTRICTIONS.reverse_bits();
        *self
    }

    pub const fn is_NOT_USABLE_IN_RATED_BG(&self) -> bool {
        (self.inner & Self::NOT_USABLE_IN_RATED_BG) != 0
    }

    pub const fn new_NOT_USABLE_IN_RATED_BG() -> Self {
        Self { inner: Self::NOT_USABLE_IN_RATED_BG }
    }

    pub fn set_NOT_USABLE_IN_RATED_BG(&mut self) -> Self {
        self.inner |= Self::NOT_USABLE_IN_RATED_BG;
        *self
    }

    pub fn clear_NOT_USABLE_IN_RATED_BG(&mut self) -> Self {
        self.inner &= Self::NOT_USABLE_IN_RATED_BG.reverse_bits();
        *self
    }

    pub const fn is_BNET_ACCOUNT_TRADE_OK(&self) -> bool {
        (self.inner & Self::BNET_ACCOUNT_TRADE_OK) != 0
    }

    pub const fn new_BNET_ACCOUNT_TRADE_OK() -> Self {
        Self { inner: Self::BNET_ACCOUNT_TRADE_OK }
    }

    pub fn set_BNET_ACCOUNT_TRADE_OK(&mut self) -> Self {
        self.inner |= Self::BNET_ACCOUNT_TRADE_OK;
        *self
    }

    pub fn clear_BNET_ACCOUNT_TRADE_OK(&mut self) -> Self {
        self.inner &= Self::BNET_ACCOUNT_TRADE_OK.reverse_bits();
        *self
    }

    pub const fn is_CONFIRM_BEFORE_USE(&self) -> bool {
        (self.inner & Self::CONFIRM_BEFORE_USE) != 0
    }

    pub const fn new_CONFIRM_BEFORE_USE() -> Self {
        Self { inner: Self::CONFIRM_BEFORE_USE }
    }

    pub fn set_CONFIRM_BEFORE_USE(&mut self) -> Self {
        self.inner |= Self::CONFIRM_BEFORE_USE;
        *self
    }

    pub fn clear_CONFIRM_BEFORE_USE(&mut self) -> Self {
        self.inner &= Self::CONFIRM_BEFORE_USE.reverse_bits();
        *self
    }

    pub const fn is_REEVALUATE_BONDING_ON_TRANSFORM(&self) -> bool {
        (self.inner & Self::REEVALUATE_BONDING_ON_TRANSFORM) != 0
    }

    pub const fn new_REEVALUATE_BONDING_ON_TRANSFORM() -> Self {
        Self { inner: Self::REEVALUATE_BONDING_ON_TRANSFORM }
    }

    pub fn set_REEVALUATE_BONDING_ON_TRANSFORM(&mut self) -> Self {
        self.inner |= Self::REEVALUATE_BONDING_ON_TRANSFORM;
        *self
    }

    pub fn clear_REEVALUATE_BONDING_ON_TRANSFORM(&mut self) -> Self {
        self.inner &= Self::REEVALUATE_BONDING_ON_TRANSFORM.reverse_bits();
        *self
    }

    pub const fn is_NO_TRANSFORM_ON_CHARGE_DEPLETION(&self) -> bool {
        (self.inner & Self::NO_TRANSFORM_ON_CHARGE_DEPLETION) != 0
    }

    pub const fn new_NO_TRANSFORM_ON_CHARGE_DEPLETION() -> Self {
        Self { inner: Self::NO_TRANSFORM_ON_CHARGE_DEPLETION }
    }

    pub fn set_NO_TRANSFORM_ON_CHARGE_DEPLETION(&mut self) -> Self {
        self.inner |= Self::NO_TRANSFORM_ON_CHARGE_DEPLETION;
        *self
    }

    pub fn clear_NO_TRANSFORM_ON_CHARGE_DEPLETION(&mut self) -> Self {
        self.inner &= Self::NO_TRANSFORM_ON_CHARGE_DEPLETION.reverse_bits();
        *self
    }

    pub const fn is_NO_ALTER_ITEM_VISUAL(&self) -> bool {
        (self.inner & Self::NO_ALTER_ITEM_VISUAL) != 0
    }

    pub const fn new_NO_ALTER_ITEM_VISUAL() -> Self {
        Self { inner: Self::NO_ALTER_ITEM_VISUAL }
    }

    pub fn set_NO_ALTER_ITEM_VISUAL(&mut self) -> Self {
        self.inner |= Self::NO_ALTER_ITEM_VISUAL;
        *self
    }

    pub fn clear_NO_ALTER_ITEM_VISUAL(&mut self) -> Self {
        self.inner &= Self::NO_ALTER_ITEM_VISUAL.reverse_bits();
        *self
    }

    pub const fn is_NO_SOURCE_FOR_ITEM_VISUAL(&self) -> bool {
        (self.inner & Self::NO_SOURCE_FOR_ITEM_VISUAL) != 0
    }

    pub const fn new_NO_SOURCE_FOR_ITEM_VISUAL() -> Self {
        Self { inner: Self::NO_SOURCE_FOR_ITEM_VISUAL }
    }

    pub fn set_NO_SOURCE_FOR_ITEM_VISUAL(&mut self) -> Self {
        self.inner |= Self::NO_SOURCE_FOR_ITEM_VISUAL;
        *self
    }

    pub fn clear_NO_SOURCE_FOR_ITEM_VISUAL(&mut self) -> Self {
        self.inner &= Self::NO_SOURCE_FOR_ITEM_VISUAL.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE(&self) -> bool {
        (self.inner & Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE) != 0
    }

    pub const fn new_IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE() -> Self {
        Self { inner: Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE }
    }

    pub fn set_IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE(&mut self) -> Self {
        self.inner |= Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE;
        *self
    }

    pub fn clear_IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE(&mut self) -> Self {
        self.inner &= Self::IGNORE_QUALITY_FOR_ITEM_VISUAL_SOURCE.reverse_bits();
        *self
    }

    pub const fn is_NO_DURABILITY(&self) -> bool {
        (self.inner & Self::NO_DURABILITY) != 0
    }

    pub const fn new_NO_DURABILITY() -> Self {
        Self { inner: Self::NO_DURABILITY }
    }

    pub fn set_NO_DURABILITY(&mut self) -> Self {
        self.inner |= Self::NO_DURABILITY;
        *self
    }

    pub fn clear_NO_DURABILITY(&mut self) -> Self {
        self.inner &= Self::NO_DURABILITY.reverse_bits();
        *self
    }

    pub const fn is_ROLE_TANK(&self) -> bool {
        (self.inner & Self::ROLE_TANK) != 0
    }

    pub const fn new_ROLE_TANK() -> Self {
        Self { inner: Self::ROLE_TANK }
    }

    pub fn set_ROLE_TANK(&mut self) -> Self {
        self.inner |= Self::ROLE_TANK;
        *self
    }

    pub fn clear_ROLE_TANK(&mut self) -> Self {
        self.inner &= Self::ROLE_TANK.reverse_bits();
        *self
    }

    pub const fn is_ROLE_HEALER(&self) -> bool {
        (self.inner & Self::ROLE_HEALER) != 0
    }

    pub const fn new_ROLE_HEALER() -> Self {
        Self { inner: Self::ROLE_HEALER }
    }

    pub fn set_ROLE_HEALER(&mut self) -> Self {
        self.inner |= Self::ROLE_HEALER;
        *self
    }

    pub fn clear_ROLE_HEALER(&mut self) -> Self {
        self.inner &= Self::ROLE_HEALER.reverse_bits();
        *self
    }

    pub const fn is_ROLE_DAMAGE(&self) -> bool {
        (self.inner & Self::ROLE_DAMAGE) != 0
    }

    pub const fn new_ROLE_DAMAGE() -> Self {
        Self { inner: Self::ROLE_DAMAGE }
    }

    pub fn set_ROLE_DAMAGE(&mut self) -> Self {
        self.inner |= Self::ROLE_DAMAGE;
        *self
    }

    pub fn clear_ROLE_DAMAGE(&mut self) -> Self {
        self.inner &= Self::ROLE_DAMAGE.reverse_bits();
        *self
    }

    pub const fn is_CAN_DROP_IN_CHALLENGE_MODE(&self) -> bool {
        (self.inner & Self::CAN_DROP_IN_CHALLENGE_MODE) != 0
    }

    pub const fn new_CAN_DROP_IN_CHALLENGE_MODE() -> Self {
        Self { inner: Self::CAN_DROP_IN_CHALLENGE_MODE }
    }

    pub fn set_CAN_DROP_IN_CHALLENGE_MODE(&mut self) -> Self {
        self.inner |= Self::CAN_DROP_IN_CHALLENGE_MODE;
        *self
    }

    pub fn clear_CAN_DROP_IN_CHALLENGE_MODE(&mut self) -> Self {
        self.inner &= Self::CAN_DROP_IN_CHALLENGE_MODE.reverse_bits();
        *self
    }

    pub const fn is_NEVER_STACK_IN_LOOT_UI(&self) -> bool {
        (self.inner & Self::NEVER_STACK_IN_LOOT_UI) != 0
    }

    pub const fn new_NEVER_STACK_IN_LOOT_UI() -> Self {
        Self { inner: Self::NEVER_STACK_IN_LOOT_UI }
    }

    pub fn set_NEVER_STACK_IN_LOOT_UI(&mut self) -> Self {
        self.inner |= Self::NEVER_STACK_IN_LOOT_UI;
        *self
    }

    pub fn clear_NEVER_STACK_IN_LOOT_UI(&mut self) -> Self {
        self.inner &= Self::NEVER_STACK_IN_LOOT_UI.reverse_bits();
        *self
    }

    pub const fn is_DISENCHANT_TO_LOOT_TABLE(&self) -> bool {
        (self.inner & Self::DISENCHANT_TO_LOOT_TABLE) != 0
    }

    pub const fn new_DISENCHANT_TO_LOOT_TABLE() -> Self {
        Self { inner: Self::DISENCHANT_TO_LOOT_TABLE }
    }

    pub fn set_DISENCHANT_TO_LOOT_TABLE(&mut self) -> Self {
        self.inner |= Self::DISENCHANT_TO_LOOT_TABLE;
        *self
    }

    pub fn clear_DISENCHANT_TO_LOOT_TABLE(&mut self) -> Self {
        self.inner &= Self::DISENCHANT_TO_LOOT_TABLE.reverse_bits();
        *self
    }

    pub const fn is_USED_IN_A_TRADESKILL(&self) -> bool {
        (self.inner & Self::USED_IN_A_TRADESKILL) != 0
    }

    pub const fn new_USED_IN_A_TRADESKILL() -> Self {
        Self { inner: Self::USED_IN_A_TRADESKILL }
    }

    pub fn set_USED_IN_A_TRADESKILL(&mut self) -> Self {
        self.inner |= Self::USED_IN_A_TRADESKILL;
        *self
    }

    pub fn clear_USED_IN_A_TRADESKILL(&mut self) -> Self {
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

