/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/external_spell_1_12.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/external_spell_1_12.wowm#L42):
/// ```text
/// flag AttributesEx1 : u32 {
///     NONE = 0x00;
///     DISMISS_PET_FIRST = 0x00000001;
///     USE_ALL_MANA = 0x00000002;
///     IS_CHANNELED = 0x00000004;
///     NO_REDIRECTION = 0x00000008;
///     NO_SKILL_INCREASE = 0x00000010;
///     ALLOW_WHILE_STEALTHED = 0x00000020;
///     IS_SELF_CHANNELED = 0x00000040;
///     NO_REFLECTION = 0x00000080;
///     ONLY_PEACEFUL_TARGETS = 0x00000100;
///     INITIATES_COMBAT_ENABLES_AUTO_ATTACK = 0x00000200;
///     NO_THREAT = 0x00000400;
///     AURA_UNIQUE = 0x00000800;
///     FAILURE_BREAKS_STEALTH = 0x00001000;
///     TOGGLE_FARSIGHT = 0x00002000;
///     TRACK_TARGET_IN_CHANNEL = 0x00004000;
///     IMMUNITY_PURGES_EFFECT = 0x00008000;
///     IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS = 0x00010000;
///     NO_AUTOCAST_AI = 0x00020000;
///     PREVENTS_ANIM = 0x00040000;
///     EXCLUDE_CASTER = 0x00080000;
///     FINISHING_MOVE_DAMAGE = 0x00100000;
///     THREAT_ONLY_ON_MISS = 0x00200000;
///     FINISHING_MOVE_DURATION = 0x00400000;
///     UNK23 = 0x00800000;
///     SPECIAL_SKILLUP = 0x01000000;
///     AURA_STAYS_AFTER_COMBAT = 0x02000000;
///     REQUIRE_ALL_TARGETS = 0x04000000;
///     DISCOUNT_POWER_ON_MISS = 0x08000000;
///     NO_AURA_ICON = 0x10000000;
///     NAME_IN_CHANNEL_BAR = 0x20000000;
///     COMBO_ON_BLOCK = 0x40000000;
///     CAST_WHEN_LEARNED = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AttributesEx1 {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl AttributesEx1 {
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
        if self.is_dismiss_pet_first() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DISMISS_PET_FIRST").unwrap();
            first = false;
        }
        if self.is_use_all_mana() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "USE_ALL_MANA").unwrap();
            first = false;
        }
        if self.is_is_channeled() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IS_CHANNELED").unwrap();
            first = false;
        }
        if self.is_no_redirection() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_REDIRECTION").unwrap();
            first = false;
        }
        if self.is_no_skill_increase() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_SKILL_INCREASE").unwrap();
            first = false;
        }
        if self.is_allow_while_stealthed() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALLOW_WHILE_STEALTHED").unwrap();
            first = false;
        }
        if self.is_is_self_channeled() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IS_SELF_CHANNELED").unwrap();
            first = false;
        }
        if self.is_no_reflection() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_REFLECTION").unwrap();
            first = false;
        }
        if self.is_only_peaceful_targets() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLY_PEACEFUL_TARGETS").unwrap();
            first = false;
        }
        if self.is_initiates_combat_enables_auto_attack() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INITIATES_COMBAT_ENABLES_AUTO_ATTACK").unwrap();
            first = false;
        }
        if self.is_no_threat() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_THREAT").unwrap();
            first = false;
        }
        if self.is_aura_unique() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AURA_UNIQUE").unwrap();
            first = false;
        }
        if self.is_failure_breaks_stealth() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FAILURE_BREAKS_STEALTH").unwrap();
            first = false;
        }
        if self.is_toggle_farsight() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TOGGLE_FARSIGHT").unwrap();
            first = false;
        }
        if self.is_track_target_in_channel() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TRACK_TARGET_IN_CHANNEL").unwrap();
            first = false;
        }
        if self.is_immunity_purges_effect() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IMMUNITY_PURGES_EFFECT").unwrap();
            first = false;
        }
        if self.is_immunity_to_hostile_and_friendly_effects() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS").unwrap();
            first = false;
        }
        if self.is_no_autocast_ai() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_AUTOCAST_AI").unwrap();
            first = false;
        }
        if self.is_prevents_anim() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PREVENTS_ANIM").unwrap();
            first = false;
        }
        if self.is_exclude_caster() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "EXCLUDE_CASTER").unwrap();
            first = false;
        }
        if self.is_finishing_move_damage() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FINISHING_MOVE_DAMAGE").unwrap();
            first = false;
        }
        if self.is_threat_only_on_miss() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "THREAT_ONLY_ON_MISS").unwrap();
            first = false;
        }
        if self.is_finishing_move_duration() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "FINISHING_MOVE_DURATION").unwrap();
            first = false;
        }
        if self.is_unk23() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK23").unwrap();
            first = false;
        }
        if self.is_special_skillup() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SPECIAL_SKILLUP").unwrap();
            first = false;
        }
        if self.is_aura_stays_after_combat() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AURA_STAYS_AFTER_COMBAT").unwrap();
            first = false;
        }
        if self.is_require_all_targets() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "REQUIRE_ALL_TARGETS").unwrap();
            first = false;
        }
        if self.is_discount_power_on_miss() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DISCOUNT_POWER_ON_MISS").unwrap();
            first = false;
        }
        if self.is_no_aura_icon() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_AURA_ICON").unwrap();
            first = false;
        }
        if self.is_name_in_channel_bar() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NAME_IN_CHANNEL_BAR").unwrap();
            first = false;
        }
        if self.is_combo_on_block() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "COMBO_ON_BLOCK").unwrap();
            first = false;
        }
        if self.is_cast_when_learned() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CAST_WHEN_LEARNED").unwrap();
            first = false;
        }
        s
    }

}

impl AttributesEx1 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const DISMISS_PET_FIRST: u32 = 0x01;
    pub const USE_ALL_MANA: u32 = 0x02;
    pub const IS_CHANNELED: u32 = 0x04;
    pub const NO_REDIRECTION: u32 = 0x08;
    pub const NO_SKILL_INCREASE: u32 = 0x10;
    pub const ALLOW_WHILE_STEALTHED: u32 = 0x20;
    pub const IS_SELF_CHANNELED: u32 = 0x40;
    pub const NO_REFLECTION: u32 = 0x80;
    pub const ONLY_PEACEFUL_TARGETS: u32 = 0x100;
    pub const INITIATES_COMBAT_ENABLES_AUTO_ATTACK: u32 = 0x200;
    pub const NO_THREAT: u32 = 0x400;
    pub const AURA_UNIQUE: u32 = 0x800;
    pub const FAILURE_BREAKS_STEALTH: u32 = 0x1000;
    pub const TOGGLE_FARSIGHT: u32 = 0x2000;
    pub const TRACK_TARGET_IN_CHANNEL: u32 = 0x4000;
    pub const IMMUNITY_PURGES_EFFECT: u32 = 0x8000;
    pub const IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS: u32 = 0x10000;
    pub const NO_AUTOCAST_AI: u32 = 0x20000;
    pub const PREVENTS_ANIM: u32 = 0x40000;
    pub const EXCLUDE_CASTER: u32 = 0x80000;
    pub const FINISHING_MOVE_DAMAGE: u32 = 0x100000;
    pub const THREAT_ONLY_ON_MISS: u32 = 0x200000;
    pub const FINISHING_MOVE_DURATION: u32 = 0x400000;
    pub const UNK23: u32 = 0x800000;
    pub const SPECIAL_SKILLUP: u32 = 0x1000000;
    pub const AURA_STAYS_AFTER_COMBAT: u32 = 0x2000000;
    pub const REQUIRE_ALL_TARGETS: u32 = 0x4000000;
    pub const DISCOUNT_POWER_ON_MISS: u32 = 0x8000000;
    pub const NO_AURA_ICON: u32 = 0x10000000;
    pub const NAME_IN_CHANNEL_BAR: u32 = 0x20000000;
    pub const COMBO_ON_BLOCK: u32 = 0x40000000;
    pub const CAST_WHEN_LEARNED: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::DISMISS_PET_FIRST
                | Self::USE_ALL_MANA
                | Self::IS_CHANNELED
                | Self::NO_REDIRECTION
                | Self::NO_SKILL_INCREASE
                | Self::ALLOW_WHILE_STEALTHED
                | Self::IS_SELF_CHANNELED
                | Self::NO_REFLECTION
                | Self::ONLY_PEACEFUL_TARGETS
                | Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK
                | Self::NO_THREAT
                | Self::AURA_UNIQUE
                | Self::FAILURE_BREAKS_STEALTH
                | Self::TOGGLE_FARSIGHT
                | Self::TRACK_TARGET_IN_CHANNEL
                | Self::IMMUNITY_PURGES_EFFECT
                | Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS
                | Self::NO_AUTOCAST_AI
                | Self::PREVENTS_ANIM
                | Self::EXCLUDE_CASTER
                | Self::FINISHING_MOVE_DAMAGE
                | Self::THREAT_ONLY_ON_MISS
                | Self::FINISHING_MOVE_DURATION
                | Self::UNK23
                | Self::SPECIAL_SKILLUP
                | Self::AURA_STAYS_AFTER_COMBAT
                | Self::REQUIRE_ALL_TARGETS
                | Self::DISCOUNT_POWER_ON_MISS
                | Self::NO_AURA_ICON
                | Self::NAME_IN_CHANNEL_BAR
                | Self::COMBO_ON_BLOCK
                | Self::CAST_WHEN_LEARNED
        }
    }

    pub const fn is_dismiss_pet_first(&self) -> bool {
        (self.inner & Self::DISMISS_PET_FIRST) != 0
    }

    pub const fn new_dismiss_pet_first() -> Self {
        Self { inner: Self::DISMISS_PET_FIRST }
    }

    pub fn set_dismiss_pet_first(&mut self) -> Self {
        self.inner |= Self::DISMISS_PET_FIRST;
        *self
    }

    pub fn clear_dismiss_pet_first(&mut self) -> Self {
        self.inner &= Self::DISMISS_PET_FIRST.reverse_bits();
        *self
    }

    pub const fn is_use_all_mana(&self) -> bool {
        (self.inner & Self::USE_ALL_MANA) != 0
    }

    pub const fn new_use_all_mana() -> Self {
        Self { inner: Self::USE_ALL_MANA }
    }

    pub fn set_use_all_mana(&mut self) -> Self {
        self.inner |= Self::USE_ALL_MANA;
        *self
    }

    pub fn clear_use_all_mana(&mut self) -> Self {
        self.inner &= Self::USE_ALL_MANA.reverse_bits();
        *self
    }

    pub const fn is_is_channeled(&self) -> bool {
        (self.inner & Self::IS_CHANNELED) != 0
    }

    pub const fn new_is_channeled() -> Self {
        Self { inner: Self::IS_CHANNELED }
    }

    pub fn set_is_channeled(&mut self) -> Self {
        self.inner |= Self::IS_CHANNELED;
        *self
    }

    pub fn clear_is_channeled(&mut self) -> Self {
        self.inner &= Self::IS_CHANNELED.reverse_bits();
        *self
    }

    pub const fn is_no_redirection(&self) -> bool {
        (self.inner & Self::NO_REDIRECTION) != 0
    }

    pub const fn new_no_redirection() -> Self {
        Self { inner: Self::NO_REDIRECTION }
    }

    pub fn set_no_redirection(&mut self) -> Self {
        self.inner |= Self::NO_REDIRECTION;
        *self
    }

    pub fn clear_no_redirection(&mut self) -> Self {
        self.inner &= Self::NO_REDIRECTION.reverse_bits();
        *self
    }

    pub const fn is_no_skill_increase(&self) -> bool {
        (self.inner & Self::NO_SKILL_INCREASE) != 0
    }

    pub const fn new_no_skill_increase() -> Self {
        Self { inner: Self::NO_SKILL_INCREASE }
    }

    pub fn set_no_skill_increase(&mut self) -> Self {
        self.inner |= Self::NO_SKILL_INCREASE;
        *self
    }

    pub fn clear_no_skill_increase(&mut self) -> Self {
        self.inner &= Self::NO_SKILL_INCREASE.reverse_bits();
        *self
    }

    pub const fn is_allow_while_stealthed(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_STEALTHED) != 0
    }

    pub const fn new_allow_while_stealthed() -> Self {
        Self { inner: Self::ALLOW_WHILE_STEALTHED }
    }

    pub fn set_allow_while_stealthed(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_STEALTHED;
        *self
    }

    pub fn clear_allow_while_stealthed(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_STEALTHED.reverse_bits();
        *self
    }

    pub const fn is_is_self_channeled(&self) -> bool {
        (self.inner & Self::IS_SELF_CHANNELED) != 0
    }

    pub const fn new_is_self_channeled() -> Self {
        Self { inner: Self::IS_SELF_CHANNELED }
    }

    pub fn set_is_self_channeled(&mut self) -> Self {
        self.inner |= Self::IS_SELF_CHANNELED;
        *self
    }

    pub fn clear_is_self_channeled(&mut self) -> Self {
        self.inner &= Self::IS_SELF_CHANNELED.reverse_bits();
        *self
    }

    pub const fn is_no_reflection(&self) -> bool {
        (self.inner & Self::NO_REFLECTION) != 0
    }

    pub const fn new_no_reflection() -> Self {
        Self { inner: Self::NO_REFLECTION }
    }

    pub fn set_no_reflection(&mut self) -> Self {
        self.inner |= Self::NO_REFLECTION;
        *self
    }

    pub fn clear_no_reflection(&mut self) -> Self {
        self.inner &= Self::NO_REFLECTION.reverse_bits();
        *self
    }

    pub const fn is_only_peaceful_targets(&self) -> bool {
        (self.inner & Self::ONLY_PEACEFUL_TARGETS) != 0
    }

    pub const fn new_only_peaceful_targets() -> Self {
        Self { inner: Self::ONLY_PEACEFUL_TARGETS }
    }

    pub fn set_only_peaceful_targets(&mut self) -> Self {
        self.inner |= Self::ONLY_PEACEFUL_TARGETS;
        *self
    }

    pub fn clear_only_peaceful_targets(&mut self) -> Self {
        self.inner &= Self::ONLY_PEACEFUL_TARGETS.reverse_bits();
        *self
    }

    pub const fn is_initiates_combat_enables_auto_attack(&self) -> bool {
        (self.inner & Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK) != 0
    }

    pub const fn new_initiates_combat_enables_auto_attack() -> Self {
        Self { inner: Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK }
    }

    pub fn set_initiates_combat_enables_auto_attack(&mut self) -> Self {
        self.inner |= Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK;
        *self
    }

    pub fn clear_initiates_combat_enables_auto_attack(&mut self) -> Self {
        self.inner &= Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK.reverse_bits();
        *self
    }

    pub const fn is_no_threat(&self) -> bool {
        (self.inner & Self::NO_THREAT) != 0
    }

    pub const fn new_no_threat() -> Self {
        Self { inner: Self::NO_THREAT }
    }

    pub fn set_no_threat(&mut self) -> Self {
        self.inner |= Self::NO_THREAT;
        *self
    }

    pub fn clear_no_threat(&mut self) -> Self {
        self.inner &= Self::NO_THREAT.reverse_bits();
        *self
    }

    pub const fn is_aura_unique(&self) -> bool {
        (self.inner & Self::AURA_UNIQUE) != 0
    }

    pub const fn new_aura_unique() -> Self {
        Self { inner: Self::AURA_UNIQUE }
    }

    pub fn set_aura_unique(&mut self) -> Self {
        self.inner |= Self::AURA_UNIQUE;
        *self
    }

    pub fn clear_aura_unique(&mut self) -> Self {
        self.inner &= Self::AURA_UNIQUE.reverse_bits();
        *self
    }

    pub const fn is_failure_breaks_stealth(&self) -> bool {
        (self.inner & Self::FAILURE_BREAKS_STEALTH) != 0
    }

    pub const fn new_failure_breaks_stealth() -> Self {
        Self { inner: Self::FAILURE_BREAKS_STEALTH }
    }

    pub fn set_failure_breaks_stealth(&mut self) -> Self {
        self.inner |= Self::FAILURE_BREAKS_STEALTH;
        *self
    }

    pub fn clear_failure_breaks_stealth(&mut self) -> Self {
        self.inner &= Self::FAILURE_BREAKS_STEALTH.reverse_bits();
        *self
    }

    pub const fn is_toggle_farsight(&self) -> bool {
        (self.inner & Self::TOGGLE_FARSIGHT) != 0
    }

    pub const fn new_toggle_farsight() -> Self {
        Self { inner: Self::TOGGLE_FARSIGHT }
    }

    pub fn set_toggle_farsight(&mut self) -> Self {
        self.inner |= Self::TOGGLE_FARSIGHT;
        *self
    }

    pub fn clear_toggle_farsight(&mut self) -> Self {
        self.inner &= Self::TOGGLE_FARSIGHT.reverse_bits();
        *self
    }

    pub const fn is_track_target_in_channel(&self) -> bool {
        (self.inner & Self::TRACK_TARGET_IN_CHANNEL) != 0
    }

    pub const fn new_track_target_in_channel() -> Self {
        Self { inner: Self::TRACK_TARGET_IN_CHANNEL }
    }

    pub fn set_track_target_in_channel(&mut self) -> Self {
        self.inner |= Self::TRACK_TARGET_IN_CHANNEL;
        *self
    }

    pub fn clear_track_target_in_channel(&mut self) -> Self {
        self.inner &= Self::TRACK_TARGET_IN_CHANNEL.reverse_bits();
        *self
    }

    pub const fn is_immunity_purges_effect(&self) -> bool {
        (self.inner & Self::IMMUNITY_PURGES_EFFECT) != 0
    }

    pub const fn new_immunity_purges_effect() -> Self {
        Self { inner: Self::IMMUNITY_PURGES_EFFECT }
    }

    pub fn set_immunity_purges_effect(&mut self) -> Self {
        self.inner |= Self::IMMUNITY_PURGES_EFFECT;
        *self
    }

    pub fn clear_immunity_purges_effect(&mut self) -> Self {
        self.inner &= Self::IMMUNITY_PURGES_EFFECT.reverse_bits();
        *self
    }

    pub const fn is_immunity_to_hostile_and_friendly_effects(&self) -> bool {
        (self.inner & Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS) != 0
    }

    pub const fn new_immunity_to_hostile_and_friendly_effects() -> Self {
        Self { inner: Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS }
    }

    pub fn set_immunity_to_hostile_and_friendly_effects(&mut self) -> Self {
        self.inner |= Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS;
        *self
    }

    pub fn clear_immunity_to_hostile_and_friendly_effects(&mut self) -> Self {
        self.inner &= Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS.reverse_bits();
        *self
    }

    pub const fn is_no_autocast_ai(&self) -> bool {
        (self.inner & Self::NO_AUTOCAST_AI) != 0
    }

    pub const fn new_no_autocast_ai() -> Self {
        Self { inner: Self::NO_AUTOCAST_AI }
    }

    pub fn set_no_autocast_ai(&mut self) -> Self {
        self.inner |= Self::NO_AUTOCAST_AI;
        *self
    }

    pub fn clear_no_autocast_ai(&mut self) -> Self {
        self.inner &= Self::NO_AUTOCAST_AI.reverse_bits();
        *self
    }

    pub const fn is_prevents_anim(&self) -> bool {
        (self.inner & Self::PREVENTS_ANIM) != 0
    }

    pub const fn new_prevents_anim() -> Self {
        Self { inner: Self::PREVENTS_ANIM }
    }

    pub fn set_prevents_anim(&mut self) -> Self {
        self.inner |= Self::PREVENTS_ANIM;
        *self
    }

    pub fn clear_prevents_anim(&mut self) -> Self {
        self.inner &= Self::PREVENTS_ANIM.reverse_bits();
        *self
    }

    pub const fn is_exclude_caster(&self) -> bool {
        (self.inner & Self::EXCLUDE_CASTER) != 0
    }

    pub const fn new_exclude_caster() -> Self {
        Self { inner: Self::EXCLUDE_CASTER }
    }

    pub fn set_exclude_caster(&mut self) -> Self {
        self.inner |= Self::EXCLUDE_CASTER;
        *self
    }

    pub fn clear_exclude_caster(&mut self) -> Self {
        self.inner &= Self::EXCLUDE_CASTER.reverse_bits();
        *self
    }

    pub const fn is_finishing_move_damage(&self) -> bool {
        (self.inner & Self::FINISHING_MOVE_DAMAGE) != 0
    }

    pub const fn new_finishing_move_damage() -> Self {
        Self { inner: Self::FINISHING_MOVE_DAMAGE }
    }

    pub fn set_finishing_move_damage(&mut self) -> Self {
        self.inner |= Self::FINISHING_MOVE_DAMAGE;
        *self
    }

    pub fn clear_finishing_move_damage(&mut self) -> Self {
        self.inner &= Self::FINISHING_MOVE_DAMAGE.reverse_bits();
        *self
    }

    pub const fn is_threat_only_on_miss(&self) -> bool {
        (self.inner & Self::THREAT_ONLY_ON_MISS) != 0
    }

    pub const fn new_threat_only_on_miss() -> Self {
        Self { inner: Self::THREAT_ONLY_ON_MISS }
    }

    pub fn set_threat_only_on_miss(&mut self) -> Self {
        self.inner |= Self::THREAT_ONLY_ON_MISS;
        *self
    }

    pub fn clear_threat_only_on_miss(&mut self) -> Self {
        self.inner &= Self::THREAT_ONLY_ON_MISS.reverse_bits();
        *self
    }

    pub const fn is_finishing_move_duration(&self) -> bool {
        (self.inner & Self::FINISHING_MOVE_DURATION) != 0
    }

    pub const fn new_finishing_move_duration() -> Self {
        Self { inner: Self::FINISHING_MOVE_DURATION }
    }

    pub fn set_finishing_move_duration(&mut self) -> Self {
        self.inner |= Self::FINISHING_MOVE_DURATION;
        *self
    }

    pub fn clear_finishing_move_duration(&mut self) -> Self {
        self.inner &= Self::FINISHING_MOVE_DURATION.reverse_bits();
        *self
    }

    pub const fn is_unk23(&self) -> bool {
        (self.inner & Self::UNK23) != 0
    }

    pub const fn new_unk23() -> Self {
        Self { inner: Self::UNK23 }
    }

    pub fn set_unk23(&mut self) -> Self {
        self.inner |= Self::UNK23;
        *self
    }

    pub fn clear_unk23(&mut self) -> Self {
        self.inner &= Self::UNK23.reverse_bits();
        *self
    }

    pub const fn is_special_skillup(&self) -> bool {
        (self.inner & Self::SPECIAL_SKILLUP) != 0
    }

    pub const fn new_special_skillup() -> Self {
        Self { inner: Self::SPECIAL_SKILLUP }
    }

    pub fn set_special_skillup(&mut self) -> Self {
        self.inner |= Self::SPECIAL_SKILLUP;
        *self
    }

    pub fn clear_special_skillup(&mut self) -> Self {
        self.inner &= Self::SPECIAL_SKILLUP.reverse_bits();
        *self
    }

    pub const fn is_aura_stays_after_combat(&self) -> bool {
        (self.inner & Self::AURA_STAYS_AFTER_COMBAT) != 0
    }

    pub const fn new_aura_stays_after_combat() -> Self {
        Self { inner: Self::AURA_STAYS_AFTER_COMBAT }
    }

    pub fn set_aura_stays_after_combat(&mut self) -> Self {
        self.inner |= Self::AURA_STAYS_AFTER_COMBAT;
        *self
    }

    pub fn clear_aura_stays_after_combat(&mut self) -> Self {
        self.inner &= Self::AURA_STAYS_AFTER_COMBAT.reverse_bits();
        *self
    }

    pub const fn is_require_all_targets(&self) -> bool {
        (self.inner & Self::REQUIRE_ALL_TARGETS) != 0
    }

    pub const fn new_require_all_targets() -> Self {
        Self { inner: Self::REQUIRE_ALL_TARGETS }
    }

    pub fn set_require_all_targets(&mut self) -> Self {
        self.inner |= Self::REQUIRE_ALL_TARGETS;
        *self
    }

    pub fn clear_require_all_targets(&mut self) -> Self {
        self.inner &= Self::REQUIRE_ALL_TARGETS.reverse_bits();
        *self
    }

    pub const fn is_discount_power_on_miss(&self) -> bool {
        (self.inner & Self::DISCOUNT_POWER_ON_MISS) != 0
    }

    pub const fn new_discount_power_on_miss() -> Self {
        Self { inner: Self::DISCOUNT_POWER_ON_MISS }
    }

    pub fn set_discount_power_on_miss(&mut self) -> Self {
        self.inner |= Self::DISCOUNT_POWER_ON_MISS;
        *self
    }

    pub fn clear_discount_power_on_miss(&mut self) -> Self {
        self.inner &= Self::DISCOUNT_POWER_ON_MISS.reverse_bits();
        *self
    }

    pub const fn is_no_aura_icon(&self) -> bool {
        (self.inner & Self::NO_AURA_ICON) != 0
    }

    pub const fn new_no_aura_icon() -> Self {
        Self { inner: Self::NO_AURA_ICON }
    }

    pub fn set_no_aura_icon(&mut self) -> Self {
        self.inner |= Self::NO_AURA_ICON;
        *self
    }

    pub fn clear_no_aura_icon(&mut self) -> Self {
        self.inner &= Self::NO_AURA_ICON.reverse_bits();
        *self
    }

    pub const fn is_name_in_channel_bar(&self) -> bool {
        (self.inner & Self::NAME_IN_CHANNEL_BAR) != 0
    }

    pub const fn new_name_in_channel_bar() -> Self {
        Self { inner: Self::NAME_IN_CHANNEL_BAR }
    }

    pub fn set_name_in_channel_bar(&mut self) -> Self {
        self.inner |= Self::NAME_IN_CHANNEL_BAR;
        *self
    }

    pub fn clear_name_in_channel_bar(&mut self) -> Self {
        self.inner &= Self::NAME_IN_CHANNEL_BAR.reverse_bits();
        *self
    }

    pub const fn is_combo_on_block(&self) -> bool {
        (self.inner & Self::COMBO_ON_BLOCK) != 0
    }

    pub const fn new_combo_on_block() -> Self {
        Self { inner: Self::COMBO_ON_BLOCK }
    }

    pub fn set_combo_on_block(&mut self) -> Self {
        self.inner |= Self::COMBO_ON_BLOCK;
        *self
    }

    pub fn clear_combo_on_block(&mut self) -> Self {
        self.inner &= Self::COMBO_ON_BLOCK.reverse_bits();
        *self
    }

    pub const fn is_cast_when_learned(&self) -> bool {
        (self.inner & Self::CAST_WHEN_LEARNED) != 0
    }

    pub const fn new_cast_when_learned() -> Self {
        Self { inner: Self::CAST_WHEN_LEARNED }
    }

    pub fn set_cast_when_learned(&mut self) -> Self {
        self.inner |= Self::CAST_WHEN_LEARNED;
        *self
    }

    pub fn clear_cast_when_learned(&mut self) -> Self {
        self.inner &= Self::CAST_WHEN_LEARNED.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AttributesEx1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AttributesEx1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AttributesEx1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AttributesEx1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AttributesEx1 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AttributesEx1 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AttributesEx1 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AttributesEx1 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AttributesEx1 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AttributesEx1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for AttributesEx1 {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for AttributesEx1 {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for AttributesEx1 {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for AttributesEx1 {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for AttributesEx1 {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for AttributesEx1 {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for AttributesEx1 {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for AttributesEx1 {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for AttributesEx1 {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

