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

#[cfg(feature = "print-testcase")]
impl AttributesEx2 {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_allow_dead_target() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_DEAD_TARGET").unwrap();
            first = false;
        }
        if self.is_no_shapeshift_ui() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_SHAPESHIFT_UI").unwrap();
            first = false;
        }
        if self.is_ignore_line_of_sight() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_LINE_OF_SIGHT").unwrap();
            first = false;
        }
        if self.is_allow_low_level_buff() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_LOW_LEVEL_BUFF").unwrap();
            first = false;
        }
        if self.is_use_shapeshift_bar() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "USE_SHAPESHIFT_BAR").unwrap();
            first = false;
        }
        if self.is_auto_repeat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AUTO_REPEAT").unwrap();
            first = false;
        }
        if self.is_cannot_cast_on_tapped() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CANNOT_CAST_ON_TAPPED").unwrap();
            first = false;
        }
        if self.is_do_not_report_spell_failure() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DO_NOT_REPORT_SPELL_FAILURE").unwrap();
            first = false;
        }
        if self.is_include_in_advanced_combat_log() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INCLUDE_IN_ADVANCED_COMBAT_LOG").unwrap();
            first = false;
        }
        if self.is_always_cast_as_unit() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALWAYS_CAST_AS_UNIT").unwrap();
            first = false;
        }
        if self.is_special_taming_flag() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SPECIAL_TAMING_FLAG").unwrap();
            first = false;
        }
        if self.is_no_target_per_second_costs() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_TARGET_PER_SECOND_COSTS").unwrap();
            first = false;
        }
        if self.is_chain_from_caster() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CHAIN_FROM_CASTER").unwrap();
            first = false;
        }
        if self.is_enchant_own_item_only() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ENCHANT_OWN_ITEM_ONLY").unwrap();
            first = false;
        }
        if self.is_allow_while_invisible() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_WHILE_INVISIBLE").unwrap();
            first = false;
        }
        if self.is_unk15() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "UNK15").unwrap();
            first = false;
        }
        if self.is_no_active_pets() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_ACTIVE_PETS").unwrap();
            first = false;
        }
        if self.is_do_not_reset_combat_timers() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DO_NOT_RESET_COMBAT_TIMERS").unwrap();
            first = false;
        }
        if self.is_req_dead_pet() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "REQ_DEAD_PET").unwrap();
            first = false;
        }
        if self.is_allow_while_not_shapeshifted() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_WHILE_NOT_SHAPESHIFTED").unwrap();
            first = false;
        }
        if self.is_initiate_combat_post_cast() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INITIATE_COMBAT_POST_CAST").unwrap();
            first = false;
        }
        if self.is_fail_on_all_targets_immune() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "FAIL_ON_ALL_TARGETS_IMMUNE").unwrap();
            first = false;
        }
        if self.is_no_initial_threat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_INITIAL_THREAT").unwrap();
            first = false;
        }
        if self.is_proc_cooldown_on_failure() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PROC_COOLDOWN_ON_FAILURE").unwrap();
            first = false;
        }
        if self.is_item_cast_with_owner_skill() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ITEM_CAST_WITH_OWNER_SKILL").unwrap();
            first = false;
        }
        if self.is_dont_block_mana_regen() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DONT_BLOCK_MANA_REGEN").unwrap();
            first = false;
        }
        if self.is_no_school_immunities() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_SCHOOL_IMMUNITIES").unwrap();
            first = false;
        }
        if self.is_ignore_weaponskill() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_WEAPONSKILL").unwrap();
            first = false;
        }
        if self.is_not_an_action() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NOT_AN_ACTION").unwrap();
            first = false;
        }
        if self.is_cant_crit() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CANT_CRIT").unwrap();
            first = false;
        }
        if self.is_active_threat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ACTIVE_THREAT").unwrap();
            first = false;
        }
        if self.is_retain_item_cast() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RETAIN_ITEM_CAST").unwrap();
            first = false;
        }
        s
    }

}

impl AttributesEx2 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const ALLOW_DEAD_TARGET: u32 = 0x01;
    pub const NO_SHAPESHIFT_UI: u32 = 0x02;
    pub const IGNORE_LINE_OF_SIGHT: u32 = 0x04;
    pub const ALLOW_LOW_LEVEL_BUFF: u32 = 0x08;
    pub const USE_SHAPESHIFT_BAR: u32 = 0x10;
    pub const AUTO_REPEAT: u32 = 0x20;
    pub const CANNOT_CAST_ON_TAPPED: u32 = 0x40;
    pub const DO_NOT_REPORT_SPELL_FAILURE: u32 = 0x80;
    pub const INCLUDE_IN_ADVANCED_COMBAT_LOG: u32 = 0x100;
    pub const ALWAYS_CAST_AS_UNIT: u32 = 0x200;
    pub const SPECIAL_TAMING_FLAG: u32 = 0x400;
    pub const NO_TARGET_PER_SECOND_COSTS: u32 = 0x800;
    pub const CHAIN_FROM_CASTER: u32 = 0x1000;
    pub const ENCHANT_OWN_ITEM_ONLY: u32 = 0x2000;
    pub const ALLOW_WHILE_INVISIBLE: u32 = 0x4000;
    pub const UNK15: u32 = 0x8000;
    pub const NO_ACTIVE_PETS: u32 = 0x10000;
    pub const DO_NOT_RESET_COMBAT_TIMERS: u32 = 0x20000;
    pub const REQ_DEAD_PET: u32 = 0x40000;
    pub const ALLOW_WHILE_NOT_SHAPESHIFTED: u32 = 0x80000;
    pub const INITIATE_COMBAT_POST_CAST: u32 = 0x100000;
    pub const FAIL_ON_ALL_TARGETS_IMMUNE: u32 = 0x200000;
    pub const NO_INITIAL_THREAT: u32 = 0x400000;
    pub const PROC_COOLDOWN_ON_FAILURE: u32 = 0x800000;
    pub const ITEM_CAST_WITH_OWNER_SKILL: u32 = 0x1000000;
    pub const DONT_BLOCK_MANA_REGEN: u32 = 0x2000000;
    pub const NO_SCHOOL_IMMUNITIES: u32 = 0x4000000;
    pub const IGNORE_WEAPONSKILL: u32 = 0x8000000;
    pub const NOT_AN_ACTION: u32 = 0x10000000;
    pub const CANT_CRIT: u32 = 0x20000000;
    pub const ACTIVE_THREAT: u32 = 0x40000000;
    pub const RETAIN_ITEM_CAST: u32 = 0x80000000;

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

    pub const fn is_allow_dead_target(&self) -> bool {
        (self.inner & Self::ALLOW_DEAD_TARGET) != 0
    }

    pub const fn new_allow_dead_target() -> Self {
        Self { inner: Self::ALLOW_DEAD_TARGET }
    }

    pub fn set_allow_dead_target(&mut self) -> Self {
        self.inner |= Self::ALLOW_DEAD_TARGET;
        *self
    }

    pub fn clear_allow_dead_target(&mut self) -> Self {
        self.inner &= Self::ALLOW_DEAD_TARGET.reverse_bits();
        *self
    }

    pub const fn is_no_shapeshift_ui(&self) -> bool {
        (self.inner & Self::NO_SHAPESHIFT_UI) != 0
    }

    pub const fn new_no_shapeshift_ui() -> Self {
        Self { inner: Self::NO_SHAPESHIFT_UI }
    }

    pub fn set_no_shapeshift_ui(&mut self) -> Self {
        self.inner |= Self::NO_SHAPESHIFT_UI;
        *self
    }

    pub fn clear_no_shapeshift_ui(&mut self) -> Self {
        self.inner &= Self::NO_SHAPESHIFT_UI.reverse_bits();
        *self
    }

    pub const fn is_ignore_line_of_sight(&self) -> bool {
        (self.inner & Self::IGNORE_LINE_OF_SIGHT) != 0
    }

    pub const fn new_ignore_line_of_sight() -> Self {
        Self { inner: Self::IGNORE_LINE_OF_SIGHT }
    }

    pub fn set_ignore_line_of_sight(&mut self) -> Self {
        self.inner |= Self::IGNORE_LINE_OF_SIGHT;
        *self
    }

    pub fn clear_ignore_line_of_sight(&mut self) -> Self {
        self.inner &= Self::IGNORE_LINE_OF_SIGHT.reverse_bits();
        *self
    }

    pub const fn is_allow_low_level_buff(&self) -> bool {
        (self.inner & Self::ALLOW_LOW_LEVEL_BUFF) != 0
    }

    pub const fn new_allow_low_level_buff() -> Self {
        Self { inner: Self::ALLOW_LOW_LEVEL_BUFF }
    }

    pub fn set_allow_low_level_buff(&mut self) -> Self {
        self.inner |= Self::ALLOW_LOW_LEVEL_BUFF;
        *self
    }

    pub fn clear_allow_low_level_buff(&mut self) -> Self {
        self.inner &= Self::ALLOW_LOW_LEVEL_BUFF.reverse_bits();
        *self
    }

    pub const fn is_use_shapeshift_bar(&self) -> bool {
        (self.inner & Self::USE_SHAPESHIFT_BAR) != 0
    }

    pub const fn new_use_shapeshift_bar() -> Self {
        Self { inner: Self::USE_SHAPESHIFT_BAR }
    }

    pub fn set_use_shapeshift_bar(&mut self) -> Self {
        self.inner |= Self::USE_SHAPESHIFT_BAR;
        *self
    }

    pub fn clear_use_shapeshift_bar(&mut self) -> Self {
        self.inner &= Self::USE_SHAPESHIFT_BAR.reverse_bits();
        *self
    }

    pub const fn is_auto_repeat(&self) -> bool {
        (self.inner & Self::AUTO_REPEAT) != 0
    }

    pub const fn new_auto_repeat() -> Self {
        Self { inner: Self::AUTO_REPEAT }
    }

    pub fn set_auto_repeat(&mut self) -> Self {
        self.inner |= Self::AUTO_REPEAT;
        *self
    }

    pub fn clear_auto_repeat(&mut self) -> Self {
        self.inner &= Self::AUTO_REPEAT.reverse_bits();
        *self
    }

    pub const fn is_cannot_cast_on_tapped(&self) -> bool {
        (self.inner & Self::CANNOT_CAST_ON_TAPPED) != 0
    }

    pub const fn new_cannot_cast_on_tapped() -> Self {
        Self { inner: Self::CANNOT_CAST_ON_TAPPED }
    }

    pub fn set_cannot_cast_on_tapped(&mut self) -> Self {
        self.inner |= Self::CANNOT_CAST_ON_TAPPED;
        *self
    }

    pub fn clear_cannot_cast_on_tapped(&mut self) -> Self {
        self.inner &= Self::CANNOT_CAST_ON_TAPPED.reverse_bits();
        *self
    }

    pub const fn is_do_not_report_spell_failure(&self) -> bool {
        (self.inner & Self::DO_NOT_REPORT_SPELL_FAILURE) != 0
    }

    pub const fn new_do_not_report_spell_failure() -> Self {
        Self { inner: Self::DO_NOT_REPORT_SPELL_FAILURE }
    }

    pub fn set_do_not_report_spell_failure(&mut self) -> Self {
        self.inner |= Self::DO_NOT_REPORT_SPELL_FAILURE;
        *self
    }

    pub fn clear_do_not_report_spell_failure(&mut self) -> Self {
        self.inner &= Self::DO_NOT_REPORT_SPELL_FAILURE.reverse_bits();
        *self
    }

    pub const fn is_include_in_advanced_combat_log(&self) -> bool {
        (self.inner & Self::INCLUDE_IN_ADVANCED_COMBAT_LOG) != 0
    }

    pub const fn new_include_in_advanced_combat_log() -> Self {
        Self { inner: Self::INCLUDE_IN_ADVANCED_COMBAT_LOG }
    }

    pub fn set_include_in_advanced_combat_log(&mut self) -> Self {
        self.inner |= Self::INCLUDE_IN_ADVANCED_COMBAT_LOG;
        *self
    }

    pub fn clear_include_in_advanced_combat_log(&mut self) -> Self {
        self.inner &= Self::INCLUDE_IN_ADVANCED_COMBAT_LOG.reverse_bits();
        *self
    }

    pub const fn is_always_cast_as_unit(&self) -> bool {
        (self.inner & Self::ALWAYS_CAST_AS_UNIT) != 0
    }

    pub const fn new_always_cast_as_unit() -> Self {
        Self { inner: Self::ALWAYS_CAST_AS_UNIT }
    }

    pub fn set_always_cast_as_unit(&mut self) -> Self {
        self.inner |= Self::ALWAYS_CAST_AS_UNIT;
        *self
    }

    pub fn clear_always_cast_as_unit(&mut self) -> Self {
        self.inner &= Self::ALWAYS_CAST_AS_UNIT.reverse_bits();
        *self
    }

    pub const fn is_special_taming_flag(&self) -> bool {
        (self.inner & Self::SPECIAL_TAMING_FLAG) != 0
    }

    pub const fn new_special_taming_flag() -> Self {
        Self { inner: Self::SPECIAL_TAMING_FLAG }
    }

    pub fn set_special_taming_flag(&mut self) -> Self {
        self.inner |= Self::SPECIAL_TAMING_FLAG;
        *self
    }

    pub fn clear_special_taming_flag(&mut self) -> Self {
        self.inner &= Self::SPECIAL_TAMING_FLAG.reverse_bits();
        *self
    }

    pub const fn is_no_target_per_second_costs(&self) -> bool {
        (self.inner & Self::NO_TARGET_PER_SECOND_COSTS) != 0
    }

    pub const fn new_no_target_per_second_costs() -> Self {
        Self { inner: Self::NO_TARGET_PER_SECOND_COSTS }
    }

    pub fn set_no_target_per_second_costs(&mut self) -> Self {
        self.inner |= Self::NO_TARGET_PER_SECOND_COSTS;
        *self
    }

    pub fn clear_no_target_per_second_costs(&mut self) -> Self {
        self.inner &= Self::NO_TARGET_PER_SECOND_COSTS.reverse_bits();
        *self
    }

    pub const fn is_chain_from_caster(&self) -> bool {
        (self.inner & Self::CHAIN_FROM_CASTER) != 0
    }

    pub const fn new_chain_from_caster() -> Self {
        Self { inner: Self::CHAIN_FROM_CASTER }
    }

    pub fn set_chain_from_caster(&mut self) -> Self {
        self.inner |= Self::CHAIN_FROM_CASTER;
        *self
    }

    pub fn clear_chain_from_caster(&mut self) -> Self {
        self.inner &= Self::CHAIN_FROM_CASTER.reverse_bits();
        *self
    }

    pub const fn is_enchant_own_item_only(&self) -> bool {
        (self.inner & Self::ENCHANT_OWN_ITEM_ONLY) != 0
    }

    pub const fn new_enchant_own_item_only() -> Self {
        Self { inner: Self::ENCHANT_OWN_ITEM_ONLY }
    }

    pub fn set_enchant_own_item_only(&mut self) -> Self {
        self.inner |= Self::ENCHANT_OWN_ITEM_ONLY;
        *self
    }

    pub fn clear_enchant_own_item_only(&mut self) -> Self {
        self.inner &= Self::ENCHANT_OWN_ITEM_ONLY.reverse_bits();
        *self
    }

    pub const fn is_allow_while_invisible(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_INVISIBLE) != 0
    }

    pub const fn new_allow_while_invisible() -> Self {
        Self { inner: Self::ALLOW_WHILE_INVISIBLE }
    }

    pub fn set_allow_while_invisible(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_INVISIBLE;
        *self
    }

    pub fn clear_allow_while_invisible(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_INVISIBLE.reverse_bits();
        *self
    }

    pub const fn is_unk15(&self) -> bool {
        (self.inner & Self::UNK15) != 0
    }

    pub const fn new_unk15() -> Self {
        Self { inner: Self::UNK15 }
    }

    pub fn set_unk15(&mut self) -> Self {
        self.inner |= Self::UNK15;
        *self
    }

    pub fn clear_unk15(&mut self) -> Self {
        self.inner &= Self::UNK15.reverse_bits();
        *self
    }

    pub const fn is_no_active_pets(&self) -> bool {
        (self.inner & Self::NO_ACTIVE_PETS) != 0
    }

    pub const fn new_no_active_pets() -> Self {
        Self { inner: Self::NO_ACTIVE_PETS }
    }

    pub fn set_no_active_pets(&mut self) -> Self {
        self.inner |= Self::NO_ACTIVE_PETS;
        *self
    }

    pub fn clear_no_active_pets(&mut self) -> Self {
        self.inner &= Self::NO_ACTIVE_PETS.reverse_bits();
        *self
    }

    pub const fn is_do_not_reset_combat_timers(&self) -> bool {
        (self.inner & Self::DO_NOT_RESET_COMBAT_TIMERS) != 0
    }

    pub const fn new_do_not_reset_combat_timers() -> Self {
        Self { inner: Self::DO_NOT_RESET_COMBAT_TIMERS }
    }

    pub fn set_do_not_reset_combat_timers(&mut self) -> Self {
        self.inner |= Self::DO_NOT_RESET_COMBAT_TIMERS;
        *self
    }

    pub fn clear_do_not_reset_combat_timers(&mut self) -> Self {
        self.inner &= Self::DO_NOT_RESET_COMBAT_TIMERS.reverse_bits();
        *self
    }

    pub const fn is_req_dead_pet(&self) -> bool {
        (self.inner & Self::REQ_DEAD_PET) != 0
    }

    pub const fn new_req_dead_pet() -> Self {
        Self { inner: Self::REQ_DEAD_PET }
    }

    pub fn set_req_dead_pet(&mut self) -> Self {
        self.inner |= Self::REQ_DEAD_PET;
        *self
    }

    pub fn clear_req_dead_pet(&mut self) -> Self {
        self.inner &= Self::REQ_DEAD_PET.reverse_bits();
        *self
    }

    pub const fn is_allow_while_not_shapeshifted(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_NOT_SHAPESHIFTED) != 0
    }

    pub const fn new_allow_while_not_shapeshifted() -> Self {
        Self { inner: Self::ALLOW_WHILE_NOT_SHAPESHIFTED }
    }

    pub fn set_allow_while_not_shapeshifted(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_NOT_SHAPESHIFTED;
        *self
    }

    pub fn clear_allow_while_not_shapeshifted(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_NOT_SHAPESHIFTED.reverse_bits();
        *self
    }

    pub const fn is_initiate_combat_post_cast(&self) -> bool {
        (self.inner & Self::INITIATE_COMBAT_POST_CAST) != 0
    }

    pub const fn new_initiate_combat_post_cast() -> Self {
        Self { inner: Self::INITIATE_COMBAT_POST_CAST }
    }

    pub fn set_initiate_combat_post_cast(&mut self) -> Self {
        self.inner |= Self::INITIATE_COMBAT_POST_CAST;
        *self
    }

    pub fn clear_initiate_combat_post_cast(&mut self) -> Self {
        self.inner &= Self::INITIATE_COMBAT_POST_CAST.reverse_bits();
        *self
    }

    pub const fn is_fail_on_all_targets_immune(&self) -> bool {
        (self.inner & Self::FAIL_ON_ALL_TARGETS_IMMUNE) != 0
    }

    pub const fn new_fail_on_all_targets_immune() -> Self {
        Self { inner: Self::FAIL_ON_ALL_TARGETS_IMMUNE }
    }

    pub fn set_fail_on_all_targets_immune(&mut self) -> Self {
        self.inner |= Self::FAIL_ON_ALL_TARGETS_IMMUNE;
        *self
    }

    pub fn clear_fail_on_all_targets_immune(&mut self) -> Self {
        self.inner &= Self::FAIL_ON_ALL_TARGETS_IMMUNE.reverse_bits();
        *self
    }

    pub const fn is_no_initial_threat(&self) -> bool {
        (self.inner & Self::NO_INITIAL_THREAT) != 0
    }

    pub const fn new_no_initial_threat() -> Self {
        Self { inner: Self::NO_INITIAL_THREAT }
    }

    pub fn set_no_initial_threat(&mut self) -> Self {
        self.inner |= Self::NO_INITIAL_THREAT;
        *self
    }

    pub fn clear_no_initial_threat(&mut self) -> Self {
        self.inner &= Self::NO_INITIAL_THREAT.reverse_bits();
        *self
    }

    pub const fn is_proc_cooldown_on_failure(&self) -> bool {
        (self.inner & Self::PROC_COOLDOWN_ON_FAILURE) != 0
    }

    pub const fn new_proc_cooldown_on_failure() -> Self {
        Self { inner: Self::PROC_COOLDOWN_ON_FAILURE }
    }

    pub fn set_proc_cooldown_on_failure(&mut self) -> Self {
        self.inner |= Self::PROC_COOLDOWN_ON_FAILURE;
        *self
    }

    pub fn clear_proc_cooldown_on_failure(&mut self) -> Self {
        self.inner &= Self::PROC_COOLDOWN_ON_FAILURE.reverse_bits();
        *self
    }

    pub const fn is_item_cast_with_owner_skill(&self) -> bool {
        (self.inner & Self::ITEM_CAST_WITH_OWNER_SKILL) != 0
    }

    pub const fn new_item_cast_with_owner_skill() -> Self {
        Self { inner: Self::ITEM_CAST_WITH_OWNER_SKILL }
    }

    pub fn set_item_cast_with_owner_skill(&mut self) -> Self {
        self.inner |= Self::ITEM_CAST_WITH_OWNER_SKILL;
        *self
    }

    pub fn clear_item_cast_with_owner_skill(&mut self) -> Self {
        self.inner &= Self::ITEM_CAST_WITH_OWNER_SKILL.reverse_bits();
        *self
    }

    pub const fn is_dont_block_mana_regen(&self) -> bool {
        (self.inner & Self::DONT_BLOCK_MANA_REGEN) != 0
    }

    pub const fn new_dont_block_mana_regen() -> Self {
        Self { inner: Self::DONT_BLOCK_MANA_REGEN }
    }

    pub fn set_dont_block_mana_regen(&mut self) -> Self {
        self.inner |= Self::DONT_BLOCK_MANA_REGEN;
        *self
    }

    pub fn clear_dont_block_mana_regen(&mut self) -> Self {
        self.inner &= Self::DONT_BLOCK_MANA_REGEN.reverse_bits();
        *self
    }

    pub const fn is_no_school_immunities(&self) -> bool {
        (self.inner & Self::NO_SCHOOL_IMMUNITIES) != 0
    }

    pub const fn new_no_school_immunities() -> Self {
        Self { inner: Self::NO_SCHOOL_IMMUNITIES }
    }

    pub fn set_no_school_immunities(&mut self) -> Self {
        self.inner |= Self::NO_SCHOOL_IMMUNITIES;
        *self
    }

    pub fn clear_no_school_immunities(&mut self) -> Self {
        self.inner &= Self::NO_SCHOOL_IMMUNITIES.reverse_bits();
        *self
    }

    pub const fn is_ignore_weaponskill(&self) -> bool {
        (self.inner & Self::IGNORE_WEAPONSKILL) != 0
    }

    pub const fn new_ignore_weaponskill() -> Self {
        Self { inner: Self::IGNORE_WEAPONSKILL }
    }

    pub fn set_ignore_weaponskill(&mut self) -> Self {
        self.inner |= Self::IGNORE_WEAPONSKILL;
        *self
    }

    pub fn clear_ignore_weaponskill(&mut self) -> Self {
        self.inner &= Self::IGNORE_WEAPONSKILL.reverse_bits();
        *self
    }

    pub const fn is_not_an_action(&self) -> bool {
        (self.inner & Self::NOT_AN_ACTION) != 0
    }

    pub const fn new_not_an_action() -> Self {
        Self { inner: Self::NOT_AN_ACTION }
    }

    pub fn set_not_an_action(&mut self) -> Self {
        self.inner |= Self::NOT_AN_ACTION;
        *self
    }

    pub fn clear_not_an_action(&mut self) -> Self {
        self.inner &= Self::NOT_AN_ACTION.reverse_bits();
        *self
    }

    pub const fn is_cant_crit(&self) -> bool {
        (self.inner & Self::CANT_CRIT) != 0
    }

    pub const fn new_cant_crit() -> Self {
        Self { inner: Self::CANT_CRIT }
    }

    pub fn set_cant_crit(&mut self) -> Self {
        self.inner |= Self::CANT_CRIT;
        *self
    }

    pub fn clear_cant_crit(&mut self) -> Self {
        self.inner &= Self::CANT_CRIT.reverse_bits();
        *self
    }

    pub const fn is_active_threat(&self) -> bool {
        (self.inner & Self::ACTIVE_THREAT) != 0
    }

    pub const fn new_active_threat() -> Self {
        Self { inner: Self::ACTIVE_THREAT }
    }

    pub fn set_active_threat(&mut self) -> Self {
        self.inner |= Self::ACTIVE_THREAT;
        *self
    }

    pub fn clear_active_threat(&mut self) -> Self {
        self.inner &= Self::ACTIVE_THREAT.reverse_bits();
        *self
    }

    pub const fn is_retain_item_cast(&self) -> bool {
        (self.inner & Self::RETAIN_ITEM_CAST) != 0
    }

    pub const fn new_retain_item_cast() -> Self {
        Self { inner: Self::RETAIN_ITEM_CAST }
    }

    pub fn set_retain_item_cast(&mut self) -> Self {
        self.inner |= Self::RETAIN_ITEM_CAST;
        *self
    }

    pub fn clear_retain_item_cast(&mut self) -> Self {
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

impl std::ops::BitAnd for AttributesEx2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AttributesEx2 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AttributesEx2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AttributesEx2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AttributesEx2 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AttributesEx2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

