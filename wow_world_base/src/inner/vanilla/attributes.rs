/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external_spell_1_12.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external_spell_1_12.wowm#L1):
/// ```text
/// flag Attributes : u32 {
///     NONE = 0x00;
///     PROC_FAILURE_BURNS_CHARGE = 0x00000001;
///     USES_RANGED_SLOT = 0x00000002;
///     ON_NEXT_SWING_NO_DAMAGE = 0x00000004;
///     NEED_EXOTIC_AMMO = 0x00000008;
///     IS_ABILITY = 0x00000010;
///     IS_TRADESKILL = 0x00000020;
///     PASSIVE = 0x00000040;
///     DO_NOT_DISPLAY = 0x00000080;
///     DO_NOT_LOG = 0x00000100;
///     HELD_ITEM_ONLY = 0x00000200;
///     ON_NEXT_SWING = 0x00000400;
///     WEARER_CASTS_PROC_TRIGGER = 0x00000800;
///     DAYTIME_ONLY = 0x00001000;
///     NIGHT_ONLY = 0x00002000;
///     ONLY_INDOORS = 0x00004000;
///     ONLY_OUTDOORS = 0x00008000;
///     NOT_SHAPESHIFT = 0x00010000;
///     ONLY_STEALTHED = 0x00020000;
///     DO_NOT_SHEATH = 0x00040000;
///     SCALES_WITH_CREATURE_LEVEL = 0x00080000;
///     CANCELS_AUTO_ATTACK_COMBAT = 0x00100000;
///     NO_ACTIVE_DEFENSE = 0x00200000;
///     TRACK_TARGET_IN_CAST_PLAYER_ONLY = 0x00400000;
///     ALLOW_CAST_WHILE_DEAD = 0x00800000;
///     ALLOW_WHILE_MOUNTED = 0x01000000;
///     COOLDOWN_ON_EVENT = 0x02000000;
///     AURA_IS_DEBUFF = 0x04000000;
///     ALLOW_WHILE_SITTING = 0x08000000;
///     NOT_IN_COMBAT_ONLY_PEACEFUL = 0x10000000;
///     NO_IMMUNITIES = 0x20000000;
///     HEARTBEAT_RESIST = 0x40000000;
///     NO_AURA_CANCEL = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Attributes {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl Attributes {
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
        if self.is_proc_failure_burns_charge() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PROC_FAILURE_BURNS_CHARGE").unwrap();
            first = false;
        }
        if self.is_uses_ranged_slot() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "USES_RANGED_SLOT").unwrap();
            first = false;
        }
        if self.is_on_next_swing_no_damage() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ON_NEXT_SWING_NO_DAMAGE").unwrap();
            first = false;
        }
        if self.is_need_exotic_ammo() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NEED_EXOTIC_AMMO").unwrap();
            first = false;
        }
        if self.is_is_ability() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IS_ABILITY").unwrap();
            first = false;
        }
        if self.is_is_tradeskill() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IS_TRADESKILL").unwrap();
            first = false;
        }
        if self.is_passive() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PASSIVE").unwrap();
            first = false;
        }
        if self.is_do_not_display() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DO_NOT_DISPLAY").unwrap();
            first = false;
        }
        if self.is_do_not_log() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DO_NOT_LOG").unwrap();
            first = false;
        }
        if self.is_held_item_only() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HELD_ITEM_ONLY").unwrap();
            first = false;
        }
        if self.is_on_next_swing() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ON_NEXT_SWING").unwrap();
            first = false;
        }
        if self.is_wearer_casts_proc_trigger() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "WEARER_CASTS_PROC_TRIGGER").unwrap();
            first = false;
        }
        if self.is_daytime_only() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DAYTIME_ONLY").unwrap();
            first = false;
        }
        if self.is_night_only() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NIGHT_ONLY").unwrap();
            first = false;
        }
        if self.is_only_indoors() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ONLY_INDOORS").unwrap();
            first = false;
        }
        if self.is_only_outdoors() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ONLY_OUTDOORS").unwrap();
            first = false;
        }
        if self.is_not_shapeshift() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NOT_SHAPESHIFT").unwrap();
            first = false;
        }
        if self.is_only_stealthed() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ONLY_STEALTHED").unwrap();
            first = false;
        }
        if self.is_do_not_sheath() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "DO_NOT_SHEATH").unwrap();
            first = false;
        }
        if self.is_scales_with_creature_level() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SCALES_WITH_CREATURE_LEVEL").unwrap();
            first = false;
        }
        if self.is_cancels_auto_attack_combat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CANCELS_AUTO_ATTACK_COMBAT").unwrap();
            first = false;
        }
        if self.is_no_active_defense() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_ACTIVE_DEFENSE").unwrap();
            first = false;
        }
        if self.is_track_target_in_cast_player_only() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "TRACK_TARGET_IN_CAST_PLAYER_ONLY").unwrap();
            first = false;
        }
        if self.is_allow_cast_while_dead() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_CAST_WHILE_DEAD").unwrap();
            first = false;
        }
        if self.is_allow_while_mounted() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_WHILE_MOUNTED").unwrap();
            first = false;
        }
        if self.is_cooldown_on_event() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "COOLDOWN_ON_EVENT").unwrap();
            first = false;
        }
        if self.is_aura_is_debuff() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AURA_IS_DEBUFF").unwrap();
            first = false;
        }
        if self.is_allow_while_sitting() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_WHILE_SITTING").unwrap();
            first = false;
        }
        if self.is_not_in_combat_only_peaceful() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NOT_IN_COMBAT_ONLY_PEACEFUL").unwrap();
            first = false;
        }
        if self.is_no_immunities() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_IMMUNITIES").unwrap();
            first = false;
        }
        if self.is_heartbeat_resist() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HEARTBEAT_RESIST").unwrap();
            first = false;
        }
        if self.is_no_aura_cancel() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_AURA_CANCEL").unwrap();
            first = false;
        }
        s
    }

}

impl Attributes {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const PROC_FAILURE_BURNS_CHARGE: u32 = 0x01;
    pub const USES_RANGED_SLOT: u32 = 0x02;
    pub const ON_NEXT_SWING_NO_DAMAGE: u32 = 0x04;
    pub const NEED_EXOTIC_AMMO: u32 = 0x08;
    pub const IS_ABILITY: u32 = 0x10;
    pub const IS_TRADESKILL: u32 = 0x20;
    pub const PASSIVE: u32 = 0x40;
    pub const DO_NOT_DISPLAY: u32 = 0x80;
    pub const DO_NOT_LOG: u32 = 0x100;
    pub const HELD_ITEM_ONLY: u32 = 0x200;
    pub const ON_NEXT_SWING: u32 = 0x400;
    pub const WEARER_CASTS_PROC_TRIGGER: u32 = 0x800;
    pub const DAYTIME_ONLY: u32 = 0x1000;
    pub const NIGHT_ONLY: u32 = 0x2000;
    pub const ONLY_INDOORS: u32 = 0x4000;
    pub const ONLY_OUTDOORS: u32 = 0x8000;
    pub const NOT_SHAPESHIFT: u32 = 0x10000;
    pub const ONLY_STEALTHED: u32 = 0x20000;
    pub const DO_NOT_SHEATH: u32 = 0x40000;
    pub const SCALES_WITH_CREATURE_LEVEL: u32 = 0x80000;
    pub const CANCELS_AUTO_ATTACK_COMBAT: u32 = 0x100000;
    pub const NO_ACTIVE_DEFENSE: u32 = 0x200000;
    pub const TRACK_TARGET_IN_CAST_PLAYER_ONLY: u32 = 0x400000;
    pub const ALLOW_CAST_WHILE_DEAD: u32 = 0x800000;
    pub const ALLOW_WHILE_MOUNTED: u32 = 0x1000000;
    pub const COOLDOWN_ON_EVENT: u32 = 0x2000000;
    pub const AURA_IS_DEBUFF: u32 = 0x4000000;
    pub const ALLOW_WHILE_SITTING: u32 = 0x8000000;
    pub const NOT_IN_COMBAT_ONLY_PEACEFUL: u32 = 0x10000000;
    pub const NO_IMMUNITIES: u32 = 0x20000000;
    pub const HEARTBEAT_RESIST: u32 = 0x40000000;
    pub const NO_AURA_CANCEL: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::PROC_FAILURE_BURNS_CHARGE
                | Self::USES_RANGED_SLOT
                | Self::ON_NEXT_SWING_NO_DAMAGE
                | Self::NEED_EXOTIC_AMMO
                | Self::IS_ABILITY
                | Self::IS_TRADESKILL
                | Self::PASSIVE
                | Self::DO_NOT_DISPLAY
                | Self::DO_NOT_LOG
                | Self::HELD_ITEM_ONLY
                | Self::ON_NEXT_SWING
                | Self::WEARER_CASTS_PROC_TRIGGER
                | Self::DAYTIME_ONLY
                | Self::NIGHT_ONLY
                | Self::ONLY_INDOORS
                | Self::ONLY_OUTDOORS
                | Self::NOT_SHAPESHIFT
                | Self::ONLY_STEALTHED
                | Self::DO_NOT_SHEATH
                | Self::SCALES_WITH_CREATURE_LEVEL
                | Self::CANCELS_AUTO_ATTACK_COMBAT
                | Self::NO_ACTIVE_DEFENSE
                | Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY
                | Self::ALLOW_CAST_WHILE_DEAD
                | Self::ALLOW_WHILE_MOUNTED
                | Self::COOLDOWN_ON_EVENT
                | Self::AURA_IS_DEBUFF
                | Self::ALLOW_WHILE_SITTING
                | Self::NOT_IN_COMBAT_ONLY_PEACEFUL
                | Self::NO_IMMUNITIES
                | Self::HEARTBEAT_RESIST
                | Self::NO_AURA_CANCEL
        }
    }

    pub const fn is_proc_failure_burns_charge(&self) -> bool {
        (self.inner & Self::PROC_FAILURE_BURNS_CHARGE) != 0
    }

    pub const fn new_proc_failure_burns_charge() -> Self {
        Self { inner: Self::PROC_FAILURE_BURNS_CHARGE }
    }

    pub fn set_proc_failure_burns_charge(&mut self) -> Self {
        self.inner |= Self::PROC_FAILURE_BURNS_CHARGE;
        *self
    }

    pub fn clear_proc_failure_burns_charge(&mut self) -> Self {
        self.inner &= Self::PROC_FAILURE_BURNS_CHARGE.reverse_bits();
        *self
    }

    pub const fn is_uses_ranged_slot(&self) -> bool {
        (self.inner & Self::USES_RANGED_SLOT) != 0
    }

    pub const fn new_uses_ranged_slot() -> Self {
        Self { inner: Self::USES_RANGED_SLOT }
    }

    pub fn set_uses_ranged_slot(&mut self) -> Self {
        self.inner |= Self::USES_RANGED_SLOT;
        *self
    }

    pub fn clear_uses_ranged_slot(&mut self) -> Self {
        self.inner &= Self::USES_RANGED_SLOT.reverse_bits();
        *self
    }

    pub const fn is_on_next_swing_no_damage(&self) -> bool {
        (self.inner & Self::ON_NEXT_SWING_NO_DAMAGE) != 0
    }

    pub const fn new_on_next_swing_no_damage() -> Self {
        Self { inner: Self::ON_NEXT_SWING_NO_DAMAGE }
    }

    pub fn set_on_next_swing_no_damage(&mut self) -> Self {
        self.inner |= Self::ON_NEXT_SWING_NO_DAMAGE;
        *self
    }

    pub fn clear_on_next_swing_no_damage(&mut self) -> Self {
        self.inner &= Self::ON_NEXT_SWING_NO_DAMAGE.reverse_bits();
        *self
    }

    pub const fn is_need_exotic_ammo(&self) -> bool {
        (self.inner & Self::NEED_EXOTIC_AMMO) != 0
    }

    pub const fn new_need_exotic_ammo() -> Self {
        Self { inner: Self::NEED_EXOTIC_AMMO }
    }

    pub fn set_need_exotic_ammo(&mut self) -> Self {
        self.inner |= Self::NEED_EXOTIC_AMMO;
        *self
    }

    pub fn clear_need_exotic_ammo(&mut self) -> Self {
        self.inner &= Self::NEED_EXOTIC_AMMO.reverse_bits();
        *self
    }

    pub const fn is_is_ability(&self) -> bool {
        (self.inner & Self::IS_ABILITY) != 0
    }

    pub const fn new_is_ability() -> Self {
        Self { inner: Self::IS_ABILITY }
    }

    pub fn set_is_ability(&mut self) -> Self {
        self.inner |= Self::IS_ABILITY;
        *self
    }

    pub fn clear_is_ability(&mut self) -> Self {
        self.inner &= Self::IS_ABILITY.reverse_bits();
        *self
    }

    pub const fn is_is_tradeskill(&self) -> bool {
        (self.inner & Self::IS_TRADESKILL) != 0
    }

    pub const fn new_is_tradeskill() -> Self {
        Self { inner: Self::IS_TRADESKILL }
    }

    pub fn set_is_tradeskill(&mut self) -> Self {
        self.inner |= Self::IS_TRADESKILL;
        *self
    }

    pub fn clear_is_tradeskill(&mut self) -> Self {
        self.inner &= Self::IS_TRADESKILL.reverse_bits();
        *self
    }

    pub const fn is_passive(&self) -> bool {
        (self.inner & Self::PASSIVE) != 0
    }

    pub const fn new_passive() -> Self {
        Self { inner: Self::PASSIVE }
    }

    pub fn set_passive(&mut self) -> Self {
        self.inner |= Self::PASSIVE;
        *self
    }

    pub fn clear_passive(&mut self) -> Self {
        self.inner &= Self::PASSIVE.reverse_bits();
        *self
    }

    pub const fn is_do_not_display(&self) -> bool {
        (self.inner & Self::DO_NOT_DISPLAY) != 0
    }

    pub const fn new_do_not_display() -> Self {
        Self { inner: Self::DO_NOT_DISPLAY }
    }

    pub fn set_do_not_display(&mut self) -> Self {
        self.inner |= Self::DO_NOT_DISPLAY;
        *self
    }

    pub fn clear_do_not_display(&mut self) -> Self {
        self.inner &= Self::DO_NOT_DISPLAY.reverse_bits();
        *self
    }

    pub const fn is_do_not_log(&self) -> bool {
        (self.inner & Self::DO_NOT_LOG) != 0
    }

    pub const fn new_do_not_log() -> Self {
        Self { inner: Self::DO_NOT_LOG }
    }

    pub fn set_do_not_log(&mut self) -> Self {
        self.inner |= Self::DO_NOT_LOG;
        *self
    }

    pub fn clear_do_not_log(&mut self) -> Self {
        self.inner &= Self::DO_NOT_LOG.reverse_bits();
        *self
    }

    pub const fn is_held_item_only(&self) -> bool {
        (self.inner & Self::HELD_ITEM_ONLY) != 0
    }

    pub const fn new_held_item_only() -> Self {
        Self { inner: Self::HELD_ITEM_ONLY }
    }

    pub fn set_held_item_only(&mut self) -> Self {
        self.inner |= Self::HELD_ITEM_ONLY;
        *self
    }

    pub fn clear_held_item_only(&mut self) -> Self {
        self.inner &= Self::HELD_ITEM_ONLY.reverse_bits();
        *self
    }

    pub const fn is_on_next_swing(&self) -> bool {
        (self.inner & Self::ON_NEXT_SWING) != 0
    }

    pub const fn new_on_next_swing() -> Self {
        Self { inner: Self::ON_NEXT_SWING }
    }

    pub fn set_on_next_swing(&mut self) -> Self {
        self.inner |= Self::ON_NEXT_SWING;
        *self
    }

    pub fn clear_on_next_swing(&mut self) -> Self {
        self.inner &= Self::ON_NEXT_SWING.reverse_bits();
        *self
    }

    pub const fn is_wearer_casts_proc_trigger(&self) -> bool {
        (self.inner & Self::WEARER_CASTS_PROC_TRIGGER) != 0
    }

    pub const fn new_wearer_casts_proc_trigger() -> Self {
        Self { inner: Self::WEARER_CASTS_PROC_TRIGGER }
    }

    pub fn set_wearer_casts_proc_trigger(&mut self) -> Self {
        self.inner |= Self::WEARER_CASTS_PROC_TRIGGER;
        *self
    }

    pub fn clear_wearer_casts_proc_trigger(&mut self) -> Self {
        self.inner &= Self::WEARER_CASTS_PROC_TRIGGER.reverse_bits();
        *self
    }

    pub const fn is_daytime_only(&self) -> bool {
        (self.inner & Self::DAYTIME_ONLY) != 0
    }

    pub const fn new_daytime_only() -> Self {
        Self { inner: Self::DAYTIME_ONLY }
    }

    pub fn set_daytime_only(&mut self) -> Self {
        self.inner |= Self::DAYTIME_ONLY;
        *self
    }

    pub fn clear_daytime_only(&mut self) -> Self {
        self.inner &= Self::DAYTIME_ONLY.reverse_bits();
        *self
    }

    pub const fn is_night_only(&self) -> bool {
        (self.inner & Self::NIGHT_ONLY) != 0
    }

    pub const fn new_night_only() -> Self {
        Self { inner: Self::NIGHT_ONLY }
    }

    pub fn set_night_only(&mut self) -> Self {
        self.inner |= Self::NIGHT_ONLY;
        *self
    }

    pub fn clear_night_only(&mut self) -> Self {
        self.inner &= Self::NIGHT_ONLY.reverse_bits();
        *self
    }

    pub const fn is_only_indoors(&self) -> bool {
        (self.inner & Self::ONLY_INDOORS) != 0
    }

    pub const fn new_only_indoors() -> Self {
        Self { inner: Self::ONLY_INDOORS }
    }

    pub fn set_only_indoors(&mut self) -> Self {
        self.inner |= Self::ONLY_INDOORS;
        *self
    }

    pub fn clear_only_indoors(&mut self) -> Self {
        self.inner &= Self::ONLY_INDOORS.reverse_bits();
        *self
    }

    pub const fn is_only_outdoors(&self) -> bool {
        (self.inner & Self::ONLY_OUTDOORS) != 0
    }

    pub const fn new_only_outdoors() -> Self {
        Self { inner: Self::ONLY_OUTDOORS }
    }

    pub fn set_only_outdoors(&mut self) -> Self {
        self.inner |= Self::ONLY_OUTDOORS;
        *self
    }

    pub fn clear_only_outdoors(&mut self) -> Self {
        self.inner &= Self::ONLY_OUTDOORS.reverse_bits();
        *self
    }

    pub const fn is_not_shapeshift(&self) -> bool {
        (self.inner & Self::NOT_SHAPESHIFT) != 0
    }

    pub const fn new_not_shapeshift() -> Self {
        Self { inner: Self::NOT_SHAPESHIFT }
    }

    pub fn set_not_shapeshift(&mut self) -> Self {
        self.inner |= Self::NOT_SHAPESHIFT;
        *self
    }

    pub fn clear_not_shapeshift(&mut self) -> Self {
        self.inner &= Self::NOT_SHAPESHIFT.reverse_bits();
        *self
    }

    pub const fn is_only_stealthed(&self) -> bool {
        (self.inner & Self::ONLY_STEALTHED) != 0
    }

    pub const fn new_only_stealthed() -> Self {
        Self { inner: Self::ONLY_STEALTHED }
    }

    pub fn set_only_stealthed(&mut self) -> Self {
        self.inner |= Self::ONLY_STEALTHED;
        *self
    }

    pub fn clear_only_stealthed(&mut self) -> Self {
        self.inner &= Self::ONLY_STEALTHED.reverse_bits();
        *self
    }

    pub const fn is_do_not_sheath(&self) -> bool {
        (self.inner & Self::DO_NOT_SHEATH) != 0
    }

    pub const fn new_do_not_sheath() -> Self {
        Self { inner: Self::DO_NOT_SHEATH }
    }

    pub fn set_do_not_sheath(&mut self) -> Self {
        self.inner |= Self::DO_NOT_SHEATH;
        *self
    }

    pub fn clear_do_not_sheath(&mut self) -> Self {
        self.inner &= Self::DO_NOT_SHEATH.reverse_bits();
        *self
    }

    pub const fn is_scales_with_creature_level(&self) -> bool {
        (self.inner & Self::SCALES_WITH_CREATURE_LEVEL) != 0
    }

    pub const fn new_scales_with_creature_level() -> Self {
        Self { inner: Self::SCALES_WITH_CREATURE_LEVEL }
    }

    pub fn set_scales_with_creature_level(&mut self) -> Self {
        self.inner |= Self::SCALES_WITH_CREATURE_LEVEL;
        *self
    }

    pub fn clear_scales_with_creature_level(&mut self) -> Self {
        self.inner &= Self::SCALES_WITH_CREATURE_LEVEL.reverse_bits();
        *self
    }

    pub const fn is_cancels_auto_attack_combat(&self) -> bool {
        (self.inner & Self::CANCELS_AUTO_ATTACK_COMBAT) != 0
    }

    pub const fn new_cancels_auto_attack_combat() -> Self {
        Self { inner: Self::CANCELS_AUTO_ATTACK_COMBAT }
    }

    pub fn set_cancels_auto_attack_combat(&mut self) -> Self {
        self.inner |= Self::CANCELS_AUTO_ATTACK_COMBAT;
        *self
    }

    pub fn clear_cancels_auto_attack_combat(&mut self) -> Self {
        self.inner &= Self::CANCELS_AUTO_ATTACK_COMBAT.reverse_bits();
        *self
    }

    pub const fn is_no_active_defense(&self) -> bool {
        (self.inner & Self::NO_ACTIVE_DEFENSE) != 0
    }

    pub const fn new_no_active_defense() -> Self {
        Self { inner: Self::NO_ACTIVE_DEFENSE }
    }

    pub fn set_no_active_defense(&mut self) -> Self {
        self.inner |= Self::NO_ACTIVE_DEFENSE;
        *self
    }

    pub fn clear_no_active_defense(&mut self) -> Self {
        self.inner &= Self::NO_ACTIVE_DEFENSE.reverse_bits();
        *self
    }

    pub const fn is_track_target_in_cast_player_only(&self) -> bool {
        (self.inner & Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY) != 0
    }

    pub const fn new_track_target_in_cast_player_only() -> Self {
        Self { inner: Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY }
    }

    pub fn set_track_target_in_cast_player_only(&mut self) -> Self {
        self.inner |= Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY;
        *self
    }

    pub fn clear_track_target_in_cast_player_only(&mut self) -> Self {
        self.inner &= Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY.reverse_bits();
        *self
    }

    pub const fn is_allow_cast_while_dead(&self) -> bool {
        (self.inner & Self::ALLOW_CAST_WHILE_DEAD) != 0
    }

    pub const fn new_allow_cast_while_dead() -> Self {
        Self { inner: Self::ALLOW_CAST_WHILE_DEAD }
    }

    pub fn set_allow_cast_while_dead(&mut self) -> Self {
        self.inner |= Self::ALLOW_CAST_WHILE_DEAD;
        *self
    }

    pub fn clear_allow_cast_while_dead(&mut self) -> Self {
        self.inner &= Self::ALLOW_CAST_WHILE_DEAD.reverse_bits();
        *self
    }

    pub const fn is_allow_while_mounted(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_MOUNTED) != 0
    }

    pub const fn new_allow_while_mounted() -> Self {
        Self { inner: Self::ALLOW_WHILE_MOUNTED }
    }

    pub fn set_allow_while_mounted(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_MOUNTED;
        *self
    }

    pub fn clear_allow_while_mounted(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_MOUNTED.reverse_bits();
        *self
    }

    pub const fn is_cooldown_on_event(&self) -> bool {
        (self.inner & Self::COOLDOWN_ON_EVENT) != 0
    }

    pub const fn new_cooldown_on_event() -> Self {
        Self { inner: Self::COOLDOWN_ON_EVENT }
    }

    pub fn set_cooldown_on_event(&mut self) -> Self {
        self.inner |= Self::COOLDOWN_ON_EVENT;
        *self
    }

    pub fn clear_cooldown_on_event(&mut self) -> Self {
        self.inner &= Self::COOLDOWN_ON_EVENT.reverse_bits();
        *self
    }

    pub const fn is_aura_is_debuff(&self) -> bool {
        (self.inner & Self::AURA_IS_DEBUFF) != 0
    }

    pub const fn new_aura_is_debuff() -> Self {
        Self { inner: Self::AURA_IS_DEBUFF }
    }

    pub fn set_aura_is_debuff(&mut self) -> Self {
        self.inner |= Self::AURA_IS_DEBUFF;
        *self
    }

    pub fn clear_aura_is_debuff(&mut self) -> Self {
        self.inner &= Self::AURA_IS_DEBUFF.reverse_bits();
        *self
    }

    pub const fn is_allow_while_sitting(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_SITTING) != 0
    }

    pub const fn new_allow_while_sitting() -> Self {
        Self { inner: Self::ALLOW_WHILE_SITTING }
    }

    pub fn set_allow_while_sitting(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_SITTING;
        *self
    }

    pub fn clear_allow_while_sitting(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_SITTING.reverse_bits();
        *self
    }

    pub const fn is_not_in_combat_only_peaceful(&self) -> bool {
        (self.inner & Self::NOT_IN_COMBAT_ONLY_PEACEFUL) != 0
    }

    pub const fn new_not_in_combat_only_peaceful() -> Self {
        Self { inner: Self::NOT_IN_COMBAT_ONLY_PEACEFUL }
    }

    pub fn set_not_in_combat_only_peaceful(&mut self) -> Self {
        self.inner |= Self::NOT_IN_COMBAT_ONLY_PEACEFUL;
        *self
    }

    pub fn clear_not_in_combat_only_peaceful(&mut self) -> Self {
        self.inner &= Self::NOT_IN_COMBAT_ONLY_PEACEFUL.reverse_bits();
        *self
    }

    pub const fn is_no_immunities(&self) -> bool {
        (self.inner & Self::NO_IMMUNITIES) != 0
    }

    pub const fn new_no_immunities() -> Self {
        Self { inner: Self::NO_IMMUNITIES }
    }

    pub fn set_no_immunities(&mut self) -> Self {
        self.inner |= Self::NO_IMMUNITIES;
        *self
    }

    pub fn clear_no_immunities(&mut self) -> Self {
        self.inner &= Self::NO_IMMUNITIES.reverse_bits();
        *self
    }

    pub const fn is_heartbeat_resist(&self) -> bool {
        (self.inner & Self::HEARTBEAT_RESIST) != 0
    }

    pub const fn new_heartbeat_resist() -> Self {
        Self { inner: Self::HEARTBEAT_RESIST }
    }

    pub fn set_heartbeat_resist(&mut self) -> Self {
        self.inner |= Self::HEARTBEAT_RESIST;
        *self
    }

    pub fn clear_heartbeat_resist(&mut self) -> Self {
        self.inner &= Self::HEARTBEAT_RESIST.reverse_bits();
        *self
    }

    pub const fn is_no_aura_cancel(&self) -> bool {
        (self.inner & Self::NO_AURA_CANCEL) != 0
    }

    pub const fn new_no_aura_cancel() -> Self {
        Self { inner: Self::NO_AURA_CANCEL }
    }

    pub fn set_no_aura_cancel(&mut self) -> Self {
        self.inner |= Self::NO_AURA_CANCEL;
        *self
    }

    pub fn clear_no_aura_cancel(&mut self) -> Self {
        self.inner &= Self::NO_AURA_CANCEL.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for Attributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for Attributes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for Attributes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for Attributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for Attributes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for Attributes {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for Attributes {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

