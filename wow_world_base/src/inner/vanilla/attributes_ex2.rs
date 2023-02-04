/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external_spell_1_12.wowm:79`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external_spell_1_12.wowm#L79):
/// ```text
/// flag AttributesEx2 : u32 {
///     NONE = 0x00;
///     ALLOW_DEAD_TARGET = 0x00000001;
///     NO_SHAPESHIFT_UI = 0x00000002;
///     IGNORE_LINE_OF_SIGHT = 0x00000004;
///     ALLOW_LOW_LEVEL_BUFF = 0x00000008;
///     USE_SHAPESHIFT_BAR = 0x00000010;
///     AUTO_REPEAT = 0x00000020;
///     CANNOT_CAST_ON_TAPPED = 0x00000040;
///     DO_NOT_REPORT_SPELL_FAILURE = 0x00000080;
///     INCLUDE_IN_ADVANCED_COMBAT_LOG = 0x00000100;
///     ALWAYS_CAST_AS_UNIT = 0x00000200;
///     SPECIAL_TAMING_FLAG = 0x00000400;
///     NO_TARGET_PER_SECOND_COSTS = 0x00000800;
///     CHAIN_FROM_CASTER = 0x00001000;
///     ENCHANT_OWN_ITEM_ONLY = 0x00002000;
///     ALLOW_WHILE_INVISIBLE = 0x00004000;
///     UNK15 = 0x00008000;
///     NO_ACTIVE_PETS = 0x00010000;
///     DO_NOT_RESET_COMBAT_TIMERS = 0x00020000;
///     REQ_DEAD_PET = 0x00040000;
///     ALLOW_WHILE_NOT_SHAPESHIFTED = 0x00080000;
///     INITIATE_COMBAT_POST_CAST = 0x00100000;
///     FAIL_ON_ALL_TARGETS_IMMUNE = 0x00200000;
///     NO_INITIAL_THREAT = 0x00400000;
///     PROC_COOLDOWN_ON_FAILURE = 0x00800000;
///     ITEM_CAST_WITH_OWNER_SKILL = 0x01000000;
///     DONT_BLOCK_MANA_REGEN = 0x02000000;
///     NO_SCHOOL_IMMUNITIES = 0x04000000;
///     IGNORE_WEAPONSKILL = 0x08000000;
///     NOT_AN_ACTION = 0x10000000;
///     CANT_CRIT = 0x20000000;
///     ACTIVE_THREAT = 0x40000000;
///     RETAIN_ITEM_CAST = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AttributesEx2 {
    inner: u32,
}

impl AttributesEx2 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const ALLOW_DEAD_TARGET: u32 = 0x01;
    pub(crate) const NO_SHAPESHIFT_UI: u32 = 0x02;
    pub(crate) const IGNORE_LINE_OF_SIGHT: u32 = 0x04;
    pub(crate) const ALLOW_LOW_LEVEL_BUFF: u32 = 0x08;
    pub(crate) const USE_SHAPESHIFT_BAR: u32 = 0x10;
    pub(crate) const AUTO_REPEAT: u32 = 0x20;
    pub(crate) const CANNOT_CAST_ON_TAPPED: u32 = 0x40;
    pub(crate) const DO_NOT_REPORT_SPELL_FAILURE: u32 = 0x80;
    pub(crate) const INCLUDE_IN_ADVANCED_COMBAT_LOG: u32 = 0x100;
    pub(crate) const ALWAYS_CAST_AS_UNIT: u32 = 0x200;
    pub(crate) const SPECIAL_TAMING_FLAG: u32 = 0x400;
    pub(crate) const NO_TARGET_PER_SECOND_COSTS: u32 = 0x800;
    pub(crate) const CHAIN_FROM_CASTER: u32 = 0x1000;
    pub(crate) const ENCHANT_OWN_ITEM_ONLY: u32 = 0x2000;
    pub(crate) const ALLOW_WHILE_INVISIBLE: u32 = 0x4000;
    pub(crate) const UNK15: u32 = 0x8000;
    pub(crate) const NO_ACTIVE_PETS: u32 = 0x10000;
    pub(crate) const DO_NOT_RESET_COMBAT_TIMERS: u32 = 0x20000;
    pub(crate) const REQ_DEAD_PET: u32 = 0x40000;
    pub(crate) const ALLOW_WHILE_NOT_SHAPESHIFTED: u32 = 0x80000;
    pub(crate) const INITIATE_COMBAT_POST_CAST: u32 = 0x100000;
    pub(crate) const FAIL_ON_ALL_TARGETS_IMMUNE: u32 = 0x200000;
    pub(crate) const NO_INITIAL_THREAT: u32 = 0x400000;
    pub(crate) const PROC_COOLDOWN_ON_FAILURE: u32 = 0x800000;
    pub(crate) const ITEM_CAST_WITH_OWNER_SKILL: u32 = 0x1000000;
    pub(crate) const DONT_BLOCK_MANA_REGEN: u32 = 0x2000000;
    pub(crate) const NO_SCHOOL_IMMUNITIES: u32 = 0x4000000;
    pub(crate) const IGNORE_WEAPONSKILL: u32 = 0x8000000;
    pub(crate) const NOT_AN_ACTION: u32 = 0x10000000;
    pub(crate) const CANT_CRIT: u32 = 0x20000000;
    pub(crate) const ACTIVE_THREAT: u32 = 0x40000000;
    pub(crate) const RETAIN_ITEM_CAST: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::ALLOW_DEAD_TARGET
                | Self::NO_SHAPESHIFT_UI
                | Self::IGNORE_LINE_OF_SIGHT
                | Self::ALLOW_LOW_LEVEL_BUFF
                | Self::USE_SHAPESHIFT_BAR
                | Self::AUTO_REPEAT
                | Self::CANNOT_CAST_ON_TAPPED
                | Self::DO_NOT_REPORT_SPELL_FAILURE
                | Self::INCLUDE_IN_ADVANCED_COMBAT_LOG
                | Self::ALWAYS_CAST_AS_UNIT
                | Self::SPECIAL_TAMING_FLAG
                | Self::NO_TARGET_PER_SECOND_COSTS
                | Self::CHAIN_FROM_CASTER
                | Self::ENCHANT_OWN_ITEM_ONLY
                | Self::ALLOW_WHILE_INVISIBLE
                | Self::UNK15
                | Self::NO_ACTIVE_PETS
                | Self::DO_NOT_RESET_COMBAT_TIMERS
                | Self::REQ_DEAD_PET
                | Self::ALLOW_WHILE_NOT_SHAPESHIFTED
                | Self::INITIATE_COMBAT_POST_CAST
                | Self::FAIL_ON_ALL_TARGETS_IMMUNE
                | Self::NO_INITIAL_THREAT
                | Self::PROC_COOLDOWN_ON_FAILURE
                | Self::ITEM_CAST_WITH_OWNER_SKILL
                | Self::DONT_BLOCK_MANA_REGEN
                | Self::NO_SCHOOL_IMMUNITIES
                | Self::IGNORE_WEAPONSKILL
                | Self::NOT_AN_ACTION
                | Self::CANT_CRIT
                | Self::ACTIVE_THREAT
                | Self::RETAIN_ITEM_CAST
        }
    }

    pub const fn is_ALLOW_DEAD_TARGET(&self) -> bool {
        (self.inner & Self::ALLOW_DEAD_TARGET) != 0
    }

    pub const fn new_ALLOW_DEAD_TARGET() -> Self {
        Self { inner: Self::ALLOW_DEAD_TARGET }
    }

    pub fn set_ALLOW_DEAD_TARGET(&mut self) -> Self {
        self.inner |= Self::ALLOW_DEAD_TARGET;
        *self
    }

    pub fn clear_ALLOW_DEAD_TARGET(&mut self) -> Self {
        self.inner &= Self::ALLOW_DEAD_TARGET.reverse_bits();
        *self
    }

    pub const fn is_NO_SHAPESHIFT_UI(&self) -> bool {
        (self.inner & Self::NO_SHAPESHIFT_UI) != 0
    }

    pub const fn new_NO_SHAPESHIFT_UI() -> Self {
        Self { inner: Self::NO_SHAPESHIFT_UI }
    }

    pub fn set_NO_SHAPESHIFT_UI(&mut self) -> Self {
        self.inner |= Self::NO_SHAPESHIFT_UI;
        *self
    }

    pub fn clear_NO_SHAPESHIFT_UI(&mut self) -> Self {
        self.inner &= Self::NO_SHAPESHIFT_UI.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_LINE_OF_SIGHT(&self) -> bool {
        (self.inner & Self::IGNORE_LINE_OF_SIGHT) != 0
    }

    pub const fn new_IGNORE_LINE_OF_SIGHT() -> Self {
        Self { inner: Self::IGNORE_LINE_OF_SIGHT }
    }

    pub fn set_IGNORE_LINE_OF_SIGHT(&mut self) -> Self {
        self.inner |= Self::IGNORE_LINE_OF_SIGHT;
        *self
    }

    pub fn clear_IGNORE_LINE_OF_SIGHT(&mut self) -> Self {
        self.inner &= Self::IGNORE_LINE_OF_SIGHT.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_LOW_LEVEL_BUFF(&self) -> bool {
        (self.inner & Self::ALLOW_LOW_LEVEL_BUFF) != 0
    }

    pub const fn new_ALLOW_LOW_LEVEL_BUFF() -> Self {
        Self { inner: Self::ALLOW_LOW_LEVEL_BUFF }
    }

    pub fn set_ALLOW_LOW_LEVEL_BUFF(&mut self) -> Self {
        self.inner |= Self::ALLOW_LOW_LEVEL_BUFF;
        *self
    }

    pub fn clear_ALLOW_LOW_LEVEL_BUFF(&mut self) -> Self {
        self.inner &= Self::ALLOW_LOW_LEVEL_BUFF.reverse_bits();
        *self
    }

    pub const fn is_USE_SHAPESHIFT_BAR(&self) -> bool {
        (self.inner & Self::USE_SHAPESHIFT_BAR) != 0
    }

    pub const fn new_USE_SHAPESHIFT_BAR() -> Self {
        Self { inner: Self::USE_SHAPESHIFT_BAR }
    }

    pub fn set_USE_SHAPESHIFT_BAR(&mut self) -> Self {
        self.inner |= Self::USE_SHAPESHIFT_BAR;
        *self
    }

    pub fn clear_USE_SHAPESHIFT_BAR(&mut self) -> Self {
        self.inner &= Self::USE_SHAPESHIFT_BAR.reverse_bits();
        *self
    }

    pub const fn is_AUTO_REPEAT(&self) -> bool {
        (self.inner & Self::AUTO_REPEAT) != 0
    }

    pub const fn new_AUTO_REPEAT() -> Self {
        Self { inner: Self::AUTO_REPEAT }
    }

    pub fn set_AUTO_REPEAT(&mut self) -> Self {
        self.inner |= Self::AUTO_REPEAT;
        *self
    }

    pub fn clear_AUTO_REPEAT(&mut self) -> Self {
        self.inner &= Self::AUTO_REPEAT.reverse_bits();
        *self
    }

    pub const fn is_CANNOT_CAST_ON_TAPPED(&self) -> bool {
        (self.inner & Self::CANNOT_CAST_ON_TAPPED) != 0
    }

    pub const fn new_CANNOT_CAST_ON_TAPPED() -> Self {
        Self { inner: Self::CANNOT_CAST_ON_TAPPED }
    }

    pub fn set_CANNOT_CAST_ON_TAPPED(&mut self) -> Self {
        self.inner |= Self::CANNOT_CAST_ON_TAPPED;
        *self
    }

    pub fn clear_CANNOT_CAST_ON_TAPPED(&mut self) -> Self {
        self.inner &= Self::CANNOT_CAST_ON_TAPPED.reverse_bits();
        *self
    }

    pub const fn is_DO_NOT_REPORT_SPELL_FAILURE(&self) -> bool {
        (self.inner & Self::DO_NOT_REPORT_SPELL_FAILURE) != 0
    }

    pub const fn new_DO_NOT_REPORT_SPELL_FAILURE() -> Self {
        Self { inner: Self::DO_NOT_REPORT_SPELL_FAILURE }
    }

    pub fn set_DO_NOT_REPORT_SPELL_FAILURE(&mut self) -> Self {
        self.inner |= Self::DO_NOT_REPORT_SPELL_FAILURE;
        *self
    }

    pub fn clear_DO_NOT_REPORT_SPELL_FAILURE(&mut self) -> Self {
        self.inner &= Self::DO_NOT_REPORT_SPELL_FAILURE.reverse_bits();
        *self
    }

    pub const fn is_INCLUDE_IN_ADVANCED_COMBAT_LOG(&self) -> bool {
        (self.inner & Self::INCLUDE_IN_ADVANCED_COMBAT_LOG) != 0
    }

    pub const fn new_INCLUDE_IN_ADVANCED_COMBAT_LOG() -> Self {
        Self { inner: Self::INCLUDE_IN_ADVANCED_COMBAT_LOG }
    }

    pub fn set_INCLUDE_IN_ADVANCED_COMBAT_LOG(&mut self) -> Self {
        self.inner |= Self::INCLUDE_IN_ADVANCED_COMBAT_LOG;
        *self
    }

    pub fn clear_INCLUDE_IN_ADVANCED_COMBAT_LOG(&mut self) -> Self {
        self.inner &= Self::INCLUDE_IN_ADVANCED_COMBAT_LOG.reverse_bits();
        *self
    }

    pub const fn is_ALWAYS_CAST_AS_UNIT(&self) -> bool {
        (self.inner & Self::ALWAYS_CAST_AS_UNIT) != 0
    }

    pub const fn new_ALWAYS_CAST_AS_UNIT() -> Self {
        Self { inner: Self::ALWAYS_CAST_AS_UNIT }
    }

    pub fn set_ALWAYS_CAST_AS_UNIT(&mut self) -> Self {
        self.inner |= Self::ALWAYS_CAST_AS_UNIT;
        *self
    }

    pub fn clear_ALWAYS_CAST_AS_UNIT(&mut self) -> Self {
        self.inner &= Self::ALWAYS_CAST_AS_UNIT.reverse_bits();
        *self
    }

    pub const fn is_SPECIAL_TAMING_FLAG(&self) -> bool {
        (self.inner & Self::SPECIAL_TAMING_FLAG) != 0
    }

    pub const fn new_SPECIAL_TAMING_FLAG() -> Self {
        Self { inner: Self::SPECIAL_TAMING_FLAG }
    }

    pub fn set_SPECIAL_TAMING_FLAG(&mut self) -> Self {
        self.inner |= Self::SPECIAL_TAMING_FLAG;
        *self
    }

    pub fn clear_SPECIAL_TAMING_FLAG(&mut self) -> Self {
        self.inner &= Self::SPECIAL_TAMING_FLAG.reverse_bits();
        *self
    }

    pub const fn is_NO_TARGET_PER_SECOND_COSTS(&self) -> bool {
        (self.inner & Self::NO_TARGET_PER_SECOND_COSTS) != 0
    }

    pub const fn new_NO_TARGET_PER_SECOND_COSTS() -> Self {
        Self { inner: Self::NO_TARGET_PER_SECOND_COSTS }
    }

    pub fn set_NO_TARGET_PER_SECOND_COSTS(&mut self) -> Self {
        self.inner |= Self::NO_TARGET_PER_SECOND_COSTS;
        *self
    }

    pub fn clear_NO_TARGET_PER_SECOND_COSTS(&mut self) -> Self {
        self.inner &= Self::NO_TARGET_PER_SECOND_COSTS.reverse_bits();
        *self
    }

    pub const fn is_CHAIN_FROM_CASTER(&self) -> bool {
        (self.inner & Self::CHAIN_FROM_CASTER) != 0
    }

    pub const fn new_CHAIN_FROM_CASTER() -> Self {
        Self { inner: Self::CHAIN_FROM_CASTER }
    }

    pub fn set_CHAIN_FROM_CASTER(&mut self) -> Self {
        self.inner |= Self::CHAIN_FROM_CASTER;
        *self
    }

    pub fn clear_CHAIN_FROM_CASTER(&mut self) -> Self {
        self.inner &= Self::CHAIN_FROM_CASTER.reverse_bits();
        *self
    }

    pub const fn is_ENCHANT_OWN_ITEM_ONLY(&self) -> bool {
        (self.inner & Self::ENCHANT_OWN_ITEM_ONLY) != 0
    }

    pub const fn new_ENCHANT_OWN_ITEM_ONLY() -> Self {
        Self { inner: Self::ENCHANT_OWN_ITEM_ONLY }
    }

    pub fn set_ENCHANT_OWN_ITEM_ONLY(&mut self) -> Self {
        self.inner |= Self::ENCHANT_OWN_ITEM_ONLY;
        *self
    }

    pub fn clear_ENCHANT_OWN_ITEM_ONLY(&mut self) -> Self {
        self.inner &= Self::ENCHANT_OWN_ITEM_ONLY.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_WHILE_INVISIBLE(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_INVISIBLE) != 0
    }

    pub const fn new_ALLOW_WHILE_INVISIBLE() -> Self {
        Self { inner: Self::ALLOW_WHILE_INVISIBLE }
    }

    pub fn set_ALLOW_WHILE_INVISIBLE(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_INVISIBLE;
        *self
    }

    pub fn clear_ALLOW_WHILE_INVISIBLE(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_INVISIBLE.reverse_bits();
        *self
    }

    pub const fn is_UNK15(&self) -> bool {
        (self.inner & Self::UNK15) != 0
    }

    pub const fn new_UNK15() -> Self {
        Self { inner: Self::UNK15 }
    }

    pub fn set_UNK15(&mut self) -> Self {
        self.inner |= Self::UNK15;
        *self
    }

    pub fn clear_UNK15(&mut self) -> Self {
        self.inner &= Self::UNK15.reverse_bits();
        *self
    }

    pub const fn is_NO_ACTIVE_PETS(&self) -> bool {
        (self.inner & Self::NO_ACTIVE_PETS) != 0
    }

    pub const fn new_NO_ACTIVE_PETS() -> Self {
        Self { inner: Self::NO_ACTIVE_PETS }
    }

    pub fn set_NO_ACTIVE_PETS(&mut self) -> Self {
        self.inner |= Self::NO_ACTIVE_PETS;
        *self
    }

    pub fn clear_NO_ACTIVE_PETS(&mut self) -> Self {
        self.inner &= Self::NO_ACTIVE_PETS.reverse_bits();
        *self
    }

    pub const fn is_DO_NOT_RESET_COMBAT_TIMERS(&self) -> bool {
        (self.inner & Self::DO_NOT_RESET_COMBAT_TIMERS) != 0
    }

    pub const fn new_DO_NOT_RESET_COMBAT_TIMERS() -> Self {
        Self { inner: Self::DO_NOT_RESET_COMBAT_TIMERS }
    }

    pub fn set_DO_NOT_RESET_COMBAT_TIMERS(&mut self) -> Self {
        self.inner |= Self::DO_NOT_RESET_COMBAT_TIMERS;
        *self
    }

    pub fn clear_DO_NOT_RESET_COMBAT_TIMERS(&mut self) -> Self {
        self.inner &= Self::DO_NOT_RESET_COMBAT_TIMERS.reverse_bits();
        *self
    }

    pub const fn is_REQ_DEAD_PET(&self) -> bool {
        (self.inner & Self::REQ_DEAD_PET) != 0
    }

    pub const fn new_REQ_DEAD_PET() -> Self {
        Self { inner: Self::REQ_DEAD_PET }
    }

    pub fn set_REQ_DEAD_PET(&mut self) -> Self {
        self.inner |= Self::REQ_DEAD_PET;
        *self
    }

    pub fn clear_REQ_DEAD_PET(&mut self) -> Self {
        self.inner &= Self::REQ_DEAD_PET.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_WHILE_NOT_SHAPESHIFTED(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_NOT_SHAPESHIFTED) != 0
    }

    pub const fn new_ALLOW_WHILE_NOT_SHAPESHIFTED() -> Self {
        Self { inner: Self::ALLOW_WHILE_NOT_SHAPESHIFTED }
    }

    pub fn set_ALLOW_WHILE_NOT_SHAPESHIFTED(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_NOT_SHAPESHIFTED;
        *self
    }

    pub fn clear_ALLOW_WHILE_NOT_SHAPESHIFTED(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_NOT_SHAPESHIFTED.reverse_bits();
        *self
    }

    pub const fn is_INITIATE_COMBAT_POST_CAST(&self) -> bool {
        (self.inner & Self::INITIATE_COMBAT_POST_CAST) != 0
    }

    pub const fn new_INITIATE_COMBAT_POST_CAST() -> Self {
        Self { inner: Self::INITIATE_COMBAT_POST_CAST }
    }

    pub fn set_INITIATE_COMBAT_POST_CAST(&mut self) -> Self {
        self.inner |= Self::INITIATE_COMBAT_POST_CAST;
        *self
    }

    pub fn clear_INITIATE_COMBAT_POST_CAST(&mut self) -> Self {
        self.inner &= Self::INITIATE_COMBAT_POST_CAST.reverse_bits();
        *self
    }

    pub const fn is_FAIL_ON_ALL_TARGETS_IMMUNE(&self) -> bool {
        (self.inner & Self::FAIL_ON_ALL_TARGETS_IMMUNE) != 0
    }

    pub const fn new_FAIL_ON_ALL_TARGETS_IMMUNE() -> Self {
        Self { inner: Self::FAIL_ON_ALL_TARGETS_IMMUNE }
    }

    pub fn set_FAIL_ON_ALL_TARGETS_IMMUNE(&mut self) -> Self {
        self.inner |= Self::FAIL_ON_ALL_TARGETS_IMMUNE;
        *self
    }

    pub fn clear_FAIL_ON_ALL_TARGETS_IMMUNE(&mut self) -> Self {
        self.inner &= Self::FAIL_ON_ALL_TARGETS_IMMUNE.reverse_bits();
        *self
    }

    pub const fn is_NO_INITIAL_THREAT(&self) -> bool {
        (self.inner & Self::NO_INITIAL_THREAT) != 0
    }

    pub const fn new_NO_INITIAL_THREAT() -> Self {
        Self { inner: Self::NO_INITIAL_THREAT }
    }

    pub fn set_NO_INITIAL_THREAT(&mut self) -> Self {
        self.inner |= Self::NO_INITIAL_THREAT;
        *self
    }

    pub fn clear_NO_INITIAL_THREAT(&mut self) -> Self {
        self.inner &= Self::NO_INITIAL_THREAT.reverse_bits();
        *self
    }

    pub const fn is_PROC_COOLDOWN_ON_FAILURE(&self) -> bool {
        (self.inner & Self::PROC_COOLDOWN_ON_FAILURE) != 0
    }

    pub const fn new_PROC_COOLDOWN_ON_FAILURE() -> Self {
        Self { inner: Self::PROC_COOLDOWN_ON_FAILURE }
    }

    pub fn set_PROC_COOLDOWN_ON_FAILURE(&mut self) -> Self {
        self.inner |= Self::PROC_COOLDOWN_ON_FAILURE;
        *self
    }

    pub fn clear_PROC_COOLDOWN_ON_FAILURE(&mut self) -> Self {
        self.inner &= Self::PROC_COOLDOWN_ON_FAILURE.reverse_bits();
        *self
    }

    pub const fn is_ITEM_CAST_WITH_OWNER_SKILL(&self) -> bool {
        (self.inner & Self::ITEM_CAST_WITH_OWNER_SKILL) != 0
    }

    pub const fn new_ITEM_CAST_WITH_OWNER_SKILL() -> Self {
        Self { inner: Self::ITEM_CAST_WITH_OWNER_SKILL }
    }

    pub fn set_ITEM_CAST_WITH_OWNER_SKILL(&mut self) -> Self {
        self.inner |= Self::ITEM_CAST_WITH_OWNER_SKILL;
        *self
    }

    pub fn clear_ITEM_CAST_WITH_OWNER_SKILL(&mut self) -> Self {
        self.inner &= Self::ITEM_CAST_WITH_OWNER_SKILL.reverse_bits();
        *self
    }

    pub const fn is_DONT_BLOCK_MANA_REGEN(&self) -> bool {
        (self.inner & Self::DONT_BLOCK_MANA_REGEN) != 0
    }

    pub const fn new_DONT_BLOCK_MANA_REGEN() -> Self {
        Self { inner: Self::DONT_BLOCK_MANA_REGEN }
    }

    pub fn set_DONT_BLOCK_MANA_REGEN(&mut self) -> Self {
        self.inner |= Self::DONT_BLOCK_MANA_REGEN;
        *self
    }

    pub fn clear_DONT_BLOCK_MANA_REGEN(&mut self) -> Self {
        self.inner &= Self::DONT_BLOCK_MANA_REGEN.reverse_bits();
        *self
    }

    pub const fn is_NO_SCHOOL_IMMUNITIES(&self) -> bool {
        (self.inner & Self::NO_SCHOOL_IMMUNITIES) != 0
    }

    pub const fn new_NO_SCHOOL_IMMUNITIES() -> Self {
        Self { inner: Self::NO_SCHOOL_IMMUNITIES }
    }

    pub fn set_NO_SCHOOL_IMMUNITIES(&mut self) -> Self {
        self.inner |= Self::NO_SCHOOL_IMMUNITIES;
        *self
    }

    pub fn clear_NO_SCHOOL_IMMUNITIES(&mut self) -> Self {
        self.inner &= Self::NO_SCHOOL_IMMUNITIES.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_WEAPONSKILL(&self) -> bool {
        (self.inner & Self::IGNORE_WEAPONSKILL) != 0
    }

    pub const fn new_IGNORE_WEAPONSKILL() -> Self {
        Self { inner: Self::IGNORE_WEAPONSKILL }
    }

    pub fn set_IGNORE_WEAPONSKILL(&mut self) -> Self {
        self.inner |= Self::IGNORE_WEAPONSKILL;
        *self
    }

    pub fn clear_IGNORE_WEAPONSKILL(&mut self) -> Self {
        self.inner &= Self::IGNORE_WEAPONSKILL.reverse_bits();
        *self
    }

    pub const fn is_NOT_AN_ACTION(&self) -> bool {
        (self.inner & Self::NOT_AN_ACTION) != 0
    }

    pub const fn new_NOT_AN_ACTION() -> Self {
        Self { inner: Self::NOT_AN_ACTION }
    }

    pub fn set_NOT_AN_ACTION(&mut self) -> Self {
        self.inner |= Self::NOT_AN_ACTION;
        *self
    }

    pub fn clear_NOT_AN_ACTION(&mut self) -> Self {
        self.inner &= Self::NOT_AN_ACTION.reverse_bits();
        *self
    }

    pub const fn is_CANT_CRIT(&self) -> bool {
        (self.inner & Self::CANT_CRIT) != 0
    }

    pub const fn new_CANT_CRIT() -> Self {
        Self { inner: Self::CANT_CRIT }
    }

    pub fn set_CANT_CRIT(&mut self) -> Self {
        self.inner |= Self::CANT_CRIT;
        *self
    }

    pub fn clear_CANT_CRIT(&mut self) -> Self {
        self.inner &= Self::CANT_CRIT.reverse_bits();
        *self
    }

    pub const fn is_ACTIVE_THREAT(&self) -> bool {
        (self.inner & Self::ACTIVE_THREAT) != 0
    }

    pub const fn new_ACTIVE_THREAT() -> Self {
        Self { inner: Self::ACTIVE_THREAT }
    }

    pub fn set_ACTIVE_THREAT(&mut self) -> Self {
        self.inner |= Self::ACTIVE_THREAT;
        *self
    }

    pub fn clear_ACTIVE_THREAT(&mut self) -> Self {
        self.inner &= Self::ACTIVE_THREAT.reverse_bits();
        *self
    }

    pub const fn is_RETAIN_ITEM_CAST(&self) -> bool {
        (self.inner & Self::RETAIN_ITEM_CAST) != 0
    }

    pub const fn new_RETAIN_ITEM_CAST() -> Self {
        Self { inner: Self::RETAIN_ITEM_CAST }
    }

    pub fn set_RETAIN_ITEM_CAST(&mut self) -> Self {
        self.inner |= Self::RETAIN_ITEM_CAST;
        *self
    }

    pub fn clear_RETAIN_ITEM_CAST(&mut self) -> Self {
        self.inner &= Self::RETAIN_ITEM_CAST.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AttributesEx2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AttributesEx2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AttributesEx2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AttributesEx2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

