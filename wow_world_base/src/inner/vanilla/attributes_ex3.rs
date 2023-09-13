/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/external_spell_1_12.wowm:120`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/external_spell_1_12.wowm#L120):
/// ```text
/// flag AttributesEx3 : u32 {
///     NONE = 0x00;
///     PVP_ENABLING = 0x00000001;
///     NO_PROC_EQUIP_REQUIREMENT = 0x00000002;
///     NO_CASTING_BAR_TEXT = 0x00000004;
///     COMPLETELY_BLOCKED = 0x00000008;
///     NO_RES_TIMER = 0x00000010;
///     NO_DURABILITY_LOSS = 0x00000020;
///     NO_AVOIDANCE = 0x00000040;
///     DOT_STACKING_RULE = 0x00000080;
///     ONLY_ON_PLAYER = 0x00000100;
///     NOT_A_PROC = 0x00000200;
///     REQUIRES_MAIN_HAND_WEAPON = 0x00000400;
///     ONLY_BATTLEGROUNDS = 0x00000800;
///     ONLY_ON_GHOSTS = 0x00001000;
///     HIDE_CHANNEL_BAR = 0x00002000;
///     HIDE_IN_RAID_FILTER = 0x00004000;
///     NORMAL_RANGED_ATTACK = 0x00008000;
///     SUPPRESS_CASTER_PROCS = 0x00010000;
///     SUPPRESS_TARGET_PROCS = 0x00020000;
///     ALWAYS_HIT = 0x00040000;
///     INSTANT_TARGET_PROCS = 0x00080000;
///     ALLOW_AURA_WHILE_DEAD = 0x00100000;
///     ONLY_PROC_OUTDOORS = 0x00200000;
///     CASTING_CANCELS_AUTOREPEAT = 0x00400000;
///     NO_DAMAGE_HISTORY = 0x00800000;
///     REQUIRES_OFFHAND_WEAPON = 0x01000000;
///     TREAT_AS_PERIODIC = 0x02000000;
///     CAN_PROC_FROM_PROCS = 0x04000000;
///     ONLY_PROC_ON_CASTER = 0x08000000;
///     IGNORE_CASTER_AND_TARGET_RESTRICTIONS = 0x10000000;
///     IGNORE_CASTER_MODIFIERS = 0x20000000;
///     DO_NOT_DISPLAY_RANGE = 0x40000000;
///     NOT_ON_AOE_IMMUNE = 0x80000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AttributesEx3 {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl AttributesEx3 {
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
        if self.is_pvp_enabling() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PVP_ENABLING").unwrap();
            first = false;
        }
        if self.is_no_proc_equip_requirement() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_PROC_EQUIP_REQUIREMENT").unwrap();
            first = false;
        }
        if self.is_no_casting_bar_text() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_CASTING_BAR_TEXT").unwrap();
            first = false;
        }
        if self.is_completely_blocked() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "COMPLETELY_BLOCKED").unwrap();
            first = false;
        }
        if self.is_no_res_timer() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_RES_TIMER").unwrap();
            first = false;
        }
        if self.is_no_durability_loss() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_DURABILITY_LOSS").unwrap();
            first = false;
        }
        if self.is_no_avoidance() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_AVOIDANCE").unwrap();
            first = false;
        }
        if self.is_dot_stacking_rule() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DOT_STACKING_RULE").unwrap();
            first = false;
        }
        if self.is_only_on_player() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLY_ON_PLAYER").unwrap();
            first = false;
        }
        if self.is_not_a_proc() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NOT_A_PROC").unwrap();
            first = false;
        }
        if self.is_requires_main_hand_weapon() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "REQUIRES_MAIN_HAND_WEAPON").unwrap();
            first = false;
        }
        if self.is_only_battlegrounds() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLY_BATTLEGROUNDS").unwrap();
            first = false;
        }
        if self.is_only_on_ghosts() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLY_ON_GHOSTS").unwrap();
            first = false;
        }
        if self.is_hide_channel_bar() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HIDE_CHANNEL_BAR").unwrap();
            first = false;
        }
        if self.is_hide_in_raid_filter() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HIDE_IN_RAID_FILTER").unwrap();
            first = false;
        }
        if self.is_normal_ranged_attack() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NORMAL_RANGED_ATTACK").unwrap();
            first = false;
        }
        if self.is_suppress_caster_procs() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SUPPRESS_CASTER_PROCS").unwrap();
            first = false;
        }
        if self.is_suppress_target_procs() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SUPPRESS_TARGET_PROCS").unwrap();
            first = false;
        }
        if self.is_always_hit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALWAYS_HIT").unwrap();
            first = false;
        }
        if self.is_instant_target_procs() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "INSTANT_TARGET_PROCS").unwrap();
            first = false;
        }
        if self.is_allow_aura_while_dead() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ALLOW_AURA_WHILE_DEAD").unwrap();
            first = false;
        }
        if self.is_only_proc_outdoors() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLY_PROC_OUTDOORS").unwrap();
            first = false;
        }
        if self.is_casting_cancels_autorepeat() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CASTING_CANCELS_AUTOREPEAT").unwrap();
            first = false;
        }
        if self.is_no_damage_history() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_DAMAGE_HISTORY").unwrap();
            first = false;
        }
        if self.is_requires_offhand_weapon() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "REQUIRES_OFFHAND_WEAPON").unwrap();
            first = false;
        }
        if self.is_treat_as_periodic() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TREAT_AS_PERIODIC").unwrap();
            first = false;
        }
        if self.is_can_proc_from_procs() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CAN_PROC_FROM_PROCS").unwrap();
            first = false;
        }
        if self.is_only_proc_on_caster() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLY_PROC_ON_CASTER").unwrap();
            first = false;
        }
        if self.is_ignore_caster_and_target_restrictions() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IGNORE_CASTER_AND_TARGET_RESTRICTIONS").unwrap();
            first = false;
        }
        if self.is_ignore_caster_modifiers() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "IGNORE_CASTER_MODIFIERS").unwrap();
            first = false;
        }
        if self.is_do_not_display_range() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DO_NOT_DISPLAY_RANGE").unwrap();
            first = false;
        }
        if self.is_not_on_aoe_immune() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NOT_ON_AOE_IMMUNE").unwrap();
            first = false;
        }
        s
    }

}

impl AttributesEx3 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const PVP_ENABLING: u32 = 0x01;
    pub const NO_PROC_EQUIP_REQUIREMENT: u32 = 0x02;
    pub const NO_CASTING_BAR_TEXT: u32 = 0x04;
    pub const COMPLETELY_BLOCKED: u32 = 0x08;
    pub const NO_RES_TIMER: u32 = 0x10;
    pub const NO_DURABILITY_LOSS: u32 = 0x20;
    pub const NO_AVOIDANCE: u32 = 0x40;
    pub const DOT_STACKING_RULE: u32 = 0x80;
    pub const ONLY_ON_PLAYER: u32 = 0x100;
    pub const NOT_A_PROC: u32 = 0x200;
    pub const REQUIRES_MAIN_HAND_WEAPON: u32 = 0x400;
    pub const ONLY_BATTLEGROUNDS: u32 = 0x800;
    pub const ONLY_ON_GHOSTS: u32 = 0x1000;
    pub const HIDE_CHANNEL_BAR: u32 = 0x2000;
    pub const HIDE_IN_RAID_FILTER: u32 = 0x4000;
    pub const NORMAL_RANGED_ATTACK: u32 = 0x8000;
    pub const SUPPRESS_CASTER_PROCS: u32 = 0x10000;
    pub const SUPPRESS_TARGET_PROCS: u32 = 0x20000;
    pub const ALWAYS_HIT: u32 = 0x40000;
    pub const INSTANT_TARGET_PROCS: u32 = 0x80000;
    pub const ALLOW_AURA_WHILE_DEAD: u32 = 0x100000;
    pub const ONLY_PROC_OUTDOORS: u32 = 0x200000;
    pub const CASTING_CANCELS_AUTOREPEAT: u32 = 0x400000;
    pub const NO_DAMAGE_HISTORY: u32 = 0x800000;
    pub const REQUIRES_OFFHAND_WEAPON: u32 = 0x1000000;
    pub const TREAT_AS_PERIODIC: u32 = 0x2000000;
    pub const CAN_PROC_FROM_PROCS: u32 = 0x4000000;
    pub const ONLY_PROC_ON_CASTER: u32 = 0x8000000;
    pub const IGNORE_CASTER_AND_TARGET_RESTRICTIONS: u32 = 0x10000000;
    pub const IGNORE_CASTER_MODIFIERS: u32 = 0x20000000;
    pub const DO_NOT_DISPLAY_RANGE: u32 = 0x40000000;
    pub const NOT_ON_AOE_IMMUNE: u32 = 0x80000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::PVP_ENABLING
                | Self::NO_PROC_EQUIP_REQUIREMENT
                | Self::NO_CASTING_BAR_TEXT
                | Self::COMPLETELY_BLOCKED
                | Self::NO_RES_TIMER
                | Self::NO_DURABILITY_LOSS
                | Self::NO_AVOIDANCE
                | Self::DOT_STACKING_RULE
                | Self::ONLY_ON_PLAYER
                | Self::NOT_A_PROC
                | Self::REQUIRES_MAIN_HAND_WEAPON
                | Self::ONLY_BATTLEGROUNDS
                | Self::ONLY_ON_GHOSTS
                | Self::HIDE_CHANNEL_BAR
                | Self::HIDE_IN_RAID_FILTER
                | Self::NORMAL_RANGED_ATTACK
                | Self::SUPPRESS_CASTER_PROCS
                | Self::SUPPRESS_TARGET_PROCS
                | Self::ALWAYS_HIT
                | Self::INSTANT_TARGET_PROCS
                | Self::ALLOW_AURA_WHILE_DEAD
                | Self::ONLY_PROC_OUTDOORS
                | Self::CASTING_CANCELS_AUTOREPEAT
                | Self::NO_DAMAGE_HISTORY
                | Self::REQUIRES_OFFHAND_WEAPON
                | Self::TREAT_AS_PERIODIC
                | Self::CAN_PROC_FROM_PROCS
                | Self::ONLY_PROC_ON_CASTER
                | Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS
                | Self::IGNORE_CASTER_MODIFIERS
                | Self::DO_NOT_DISPLAY_RANGE
                | Self::NOT_ON_AOE_IMMUNE
        }
    }

    pub const fn is_pvp_enabling(&self) -> bool {
        (self.inner & Self::PVP_ENABLING) != 0
    }

    pub const fn new_pvp_enabling() -> Self {
        Self { inner: Self::PVP_ENABLING }
    }

    pub fn set_pvp_enabling(&mut self) -> Self {
        self.inner |= Self::PVP_ENABLING;
        *self
    }

    pub fn clear_pvp_enabling(&mut self) -> Self {
        self.inner &= Self::PVP_ENABLING.reverse_bits();
        *self
    }

    pub const fn is_no_proc_equip_requirement(&self) -> bool {
        (self.inner & Self::NO_PROC_EQUIP_REQUIREMENT) != 0
    }

    pub const fn new_no_proc_equip_requirement() -> Self {
        Self { inner: Self::NO_PROC_EQUIP_REQUIREMENT }
    }

    pub fn set_no_proc_equip_requirement(&mut self) -> Self {
        self.inner |= Self::NO_PROC_EQUIP_REQUIREMENT;
        *self
    }

    pub fn clear_no_proc_equip_requirement(&mut self) -> Self {
        self.inner &= Self::NO_PROC_EQUIP_REQUIREMENT.reverse_bits();
        *self
    }

    pub const fn is_no_casting_bar_text(&self) -> bool {
        (self.inner & Self::NO_CASTING_BAR_TEXT) != 0
    }

    pub const fn new_no_casting_bar_text() -> Self {
        Self { inner: Self::NO_CASTING_BAR_TEXT }
    }

    pub fn set_no_casting_bar_text(&mut self) -> Self {
        self.inner |= Self::NO_CASTING_BAR_TEXT;
        *self
    }

    pub fn clear_no_casting_bar_text(&mut self) -> Self {
        self.inner &= Self::NO_CASTING_BAR_TEXT.reverse_bits();
        *self
    }

    pub const fn is_completely_blocked(&self) -> bool {
        (self.inner & Self::COMPLETELY_BLOCKED) != 0
    }

    pub const fn new_completely_blocked() -> Self {
        Self { inner: Self::COMPLETELY_BLOCKED }
    }

    pub fn set_completely_blocked(&mut self) -> Self {
        self.inner |= Self::COMPLETELY_BLOCKED;
        *self
    }

    pub fn clear_completely_blocked(&mut self) -> Self {
        self.inner &= Self::COMPLETELY_BLOCKED.reverse_bits();
        *self
    }

    pub const fn is_no_res_timer(&self) -> bool {
        (self.inner & Self::NO_RES_TIMER) != 0
    }

    pub const fn new_no_res_timer() -> Self {
        Self { inner: Self::NO_RES_TIMER }
    }

    pub fn set_no_res_timer(&mut self) -> Self {
        self.inner |= Self::NO_RES_TIMER;
        *self
    }

    pub fn clear_no_res_timer(&mut self) -> Self {
        self.inner &= Self::NO_RES_TIMER.reverse_bits();
        *self
    }

    pub const fn is_no_durability_loss(&self) -> bool {
        (self.inner & Self::NO_DURABILITY_LOSS) != 0
    }

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

    pub const fn is_no_avoidance(&self) -> bool {
        (self.inner & Self::NO_AVOIDANCE) != 0
    }

    pub const fn new_no_avoidance() -> Self {
        Self { inner: Self::NO_AVOIDANCE }
    }

    pub fn set_no_avoidance(&mut self) -> Self {
        self.inner |= Self::NO_AVOIDANCE;
        *self
    }

    pub fn clear_no_avoidance(&mut self) -> Self {
        self.inner &= Self::NO_AVOIDANCE.reverse_bits();
        *self
    }

    pub const fn is_dot_stacking_rule(&self) -> bool {
        (self.inner & Self::DOT_STACKING_RULE) != 0
    }

    pub const fn new_dot_stacking_rule() -> Self {
        Self { inner: Self::DOT_STACKING_RULE }
    }

    pub fn set_dot_stacking_rule(&mut self) -> Self {
        self.inner |= Self::DOT_STACKING_RULE;
        *self
    }

    pub fn clear_dot_stacking_rule(&mut self) -> Self {
        self.inner &= Self::DOT_STACKING_RULE.reverse_bits();
        *self
    }

    pub const fn is_only_on_player(&self) -> bool {
        (self.inner & Self::ONLY_ON_PLAYER) != 0
    }

    pub const fn new_only_on_player() -> Self {
        Self { inner: Self::ONLY_ON_PLAYER }
    }

    pub fn set_only_on_player(&mut self) -> Self {
        self.inner |= Self::ONLY_ON_PLAYER;
        *self
    }

    pub fn clear_only_on_player(&mut self) -> Self {
        self.inner &= Self::ONLY_ON_PLAYER.reverse_bits();
        *self
    }

    pub const fn is_not_a_proc(&self) -> bool {
        (self.inner & Self::NOT_A_PROC) != 0
    }

    pub const fn new_not_a_proc() -> Self {
        Self { inner: Self::NOT_A_PROC }
    }

    pub fn set_not_a_proc(&mut self) -> Self {
        self.inner |= Self::NOT_A_PROC;
        *self
    }

    pub fn clear_not_a_proc(&mut self) -> Self {
        self.inner &= Self::NOT_A_PROC.reverse_bits();
        *self
    }

    pub const fn is_requires_main_hand_weapon(&self) -> bool {
        (self.inner & Self::REQUIRES_MAIN_HAND_WEAPON) != 0
    }

    pub const fn new_requires_main_hand_weapon() -> Self {
        Self { inner: Self::REQUIRES_MAIN_HAND_WEAPON }
    }

    pub fn set_requires_main_hand_weapon(&mut self) -> Self {
        self.inner |= Self::REQUIRES_MAIN_HAND_WEAPON;
        *self
    }

    pub fn clear_requires_main_hand_weapon(&mut self) -> Self {
        self.inner &= Self::REQUIRES_MAIN_HAND_WEAPON.reverse_bits();
        *self
    }

    pub const fn is_only_battlegrounds(&self) -> bool {
        (self.inner & Self::ONLY_BATTLEGROUNDS) != 0
    }

    pub const fn new_only_battlegrounds() -> Self {
        Self { inner: Self::ONLY_BATTLEGROUNDS }
    }

    pub fn set_only_battlegrounds(&mut self) -> Self {
        self.inner |= Self::ONLY_BATTLEGROUNDS;
        *self
    }

    pub fn clear_only_battlegrounds(&mut self) -> Self {
        self.inner &= Self::ONLY_BATTLEGROUNDS.reverse_bits();
        *self
    }

    pub const fn is_only_on_ghosts(&self) -> bool {
        (self.inner & Self::ONLY_ON_GHOSTS) != 0
    }

    pub const fn new_only_on_ghosts() -> Self {
        Self { inner: Self::ONLY_ON_GHOSTS }
    }

    pub fn set_only_on_ghosts(&mut self) -> Self {
        self.inner |= Self::ONLY_ON_GHOSTS;
        *self
    }

    pub fn clear_only_on_ghosts(&mut self) -> Self {
        self.inner &= Self::ONLY_ON_GHOSTS.reverse_bits();
        *self
    }

    pub const fn is_hide_channel_bar(&self) -> bool {
        (self.inner & Self::HIDE_CHANNEL_BAR) != 0
    }

    pub const fn new_hide_channel_bar() -> Self {
        Self { inner: Self::HIDE_CHANNEL_BAR }
    }

    pub fn set_hide_channel_bar(&mut self) -> Self {
        self.inner |= Self::HIDE_CHANNEL_BAR;
        *self
    }

    pub fn clear_hide_channel_bar(&mut self) -> Self {
        self.inner &= Self::HIDE_CHANNEL_BAR.reverse_bits();
        *self
    }

    pub const fn is_hide_in_raid_filter(&self) -> bool {
        (self.inner & Self::HIDE_IN_RAID_FILTER) != 0
    }

    pub const fn new_hide_in_raid_filter() -> Self {
        Self { inner: Self::HIDE_IN_RAID_FILTER }
    }

    pub fn set_hide_in_raid_filter(&mut self) -> Self {
        self.inner |= Self::HIDE_IN_RAID_FILTER;
        *self
    }

    pub fn clear_hide_in_raid_filter(&mut self) -> Self {
        self.inner &= Self::HIDE_IN_RAID_FILTER.reverse_bits();
        *self
    }

    pub const fn is_normal_ranged_attack(&self) -> bool {
        (self.inner & Self::NORMAL_RANGED_ATTACK) != 0
    }

    pub const fn new_normal_ranged_attack() -> Self {
        Self { inner: Self::NORMAL_RANGED_ATTACK }
    }

    pub fn set_normal_ranged_attack(&mut self) -> Self {
        self.inner |= Self::NORMAL_RANGED_ATTACK;
        *self
    }

    pub fn clear_normal_ranged_attack(&mut self) -> Self {
        self.inner &= Self::NORMAL_RANGED_ATTACK.reverse_bits();
        *self
    }

    pub const fn is_suppress_caster_procs(&self) -> bool {
        (self.inner & Self::SUPPRESS_CASTER_PROCS) != 0
    }

    pub const fn new_suppress_caster_procs() -> Self {
        Self { inner: Self::SUPPRESS_CASTER_PROCS }
    }

    pub fn set_suppress_caster_procs(&mut self) -> Self {
        self.inner |= Self::SUPPRESS_CASTER_PROCS;
        *self
    }

    pub fn clear_suppress_caster_procs(&mut self) -> Self {
        self.inner &= Self::SUPPRESS_CASTER_PROCS.reverse_bits();
        *self
    }

    pub const fn is_suppress_target_procs(&self) -> bool {
        (self.inner & Self::SUPPRESS_TARGET_PROCS) != 0
    }

    pub const fn new_suppress_target_procs() -> Self {
        Self { inner: Self::SUPPRESS_TARGET_PROCS }
    }

    pub fn set_suppress_target_procs(&mut self) -> Self {
        self.inner |= Self::SUPPRESS_TARGET_PROCS;
        *self
    }

    pub fn clear_suppress_target_procs(&mut self) -> Self {
        self.inner &= Self::SUPPRESS_TARGET_PROCS.reverse_bits();
        *self
    }

    pub const fn is_always_hit(&self) -> bool {
        (self.inner & Self::ALWAYS_HIT) != 0
    }

    pub const fn new_always_hit() -> Self {
        Self { inner: Self::ALWAYS_HIT }
    }

    pub fn set_always_hit(&mut self) -> Self {
        self.inner |= Self::ALWAYS_HIT;
        *self
    }

    pub fn clear_always_hit(&mut self) -> Self {
        self.inner &= Self::ALWAYS_HIT.reverse_bits();
        *self
    }

    pub const fn is_instant_target_procs(&self) -> bool {
        (self.inner & Self::INSTANT_TARGET_PROCS) != 0
    }

    pub const fn new_instant_target_procs() -> Self {
        Self { inner: Self::INSTANT_TARGET_PROCS }
    }

    pub fn set_instant_target_procs(&mut self) -> Self {
        self.inner |= Self::INSTANT_TARGET_PROCS;
        *self
    }

    pub fn clear_instant_target_procs(&mut self) -> Self {
        self.inner &= Self::INSTANT_TARGET_PROCS.reverse_bits();
        *self
    }

    pub const fn is_allow_aura_while_dead(&self) -> bool {
        (self.inner & Self::ALLOW_AURA_WHILE_DEAD) != 0
    }

    pub const fn new_allow_aura_while_dead() -> Self {
        Self { inner: Self::ALLOW_AURA_WHILE_DEAD }
    }

    pub fn set_allow_aura_while_dead(&mut self) -> Self {
        self.inner |= Self::ALLOW_AURA_WHILE_DEAD;
        *self
    }

    pub fn clear_allow_aura_while_dead(&mut self) -> Self {
        self.inner &= Self::ALLOW_AURA_WHILE_DEAD.reverse_bits();
        *self
    }

    pub const fn is_only_proc_outdoors(&self) -> bool {
        (self.inner & Self::ONLY_PROC_OUTDOORS) != 0
    }

    pub const fn new_only_proc_outdoors() -> Self {
        Self { inner: Self::ONLY_PROC_OUTDOORS }
    }

    pub fn set_only_proc_outdoors(&mut self) -> Self {
        self.inner |= Self::ONLY_PROC_OUTDOORS;
        *self
    }

    pub fn clear_only_proc_outdoors(&mut self) -> Self {
        self.inner &= Self::ONLY_PROC_OUTDOORS.reverse_bits();
        *self
    }

    pub const fn is_casting_cancels_autorepeat(&self) -> bool {
        (self.inner & Self::CASTING_CANCELS_AUTOREPEAT) != 0
    }

    pub const fn new_casting_cancels_autorepeat() -> Self {
        Self { inner: Self::CASTING_CANCELS_AUTOREPEAT }
    }

    pub fn set_casting_cancels_autorepeat(&mut self) -> Self {
        self.inner |= Self::CASTING_CANCELS_AUTOREPEAT;
        *self
    }

    pub fn clear_casting_cancels_autorepeat(&mut self) -> Self {
        self.inner &= Self::CASTING_CANCELS_AUTOREPEAT.reverse_bits();
        *self
    }

    pub const fn is_no_damage_history(&self) -> bool {
        (self.inner & Self::NO_DAMAGE_HISTORY) != 0
    }

    pub const fn new_no_damage_history() -> Self {
        Self { inner: Self::NO_DAMAGE_HISTORY }
    }

    pub fn set_no_damage_history(&mut self) -> Self {
        self.inner |= Self::NO_DAMAGE_HISTORY;
        *self
    }

    pub fn clear_no_damage_history(&mut self) -> Self {
        self.inner &= Self::NO_DAMAGE_HISTORY.reverse_bits();
        *self
    }

    pub const fn is_requires_offhand_weapon(&self) -> bool {
        (self.inner & Self::REQUIRES_OFFHAND_WEAPON) != 0
    }

    pub const fn new_requires_offhand_weapon() -> Self {
        Self { inner: Self::REQUIRES_OFFHAND_WEAPON }
    }

    pub fn set_requires_offhand_weapon(&mut self) -> Self {
        self.inner |= Self::REQUIRES_OFFHAND_WEAPON;
        *self
    }

    pub fn clear_requires_offhand_weapon(&mut self) -> Self {
        self.inner &= Self::REQUIRES_OFFHAND_WEAPON.reverse_bits();
        *self
    }

    pub const fn is_treat_as_periodic(&self) -> bool {
        (self.inner & Self::TREAT_AS_PERIODIC) != 0
    }

    pub const fn new_treat_as_periodic() -> Self {
        Self { inner: Self::TREAT_AS_PERIODIC }
    }

    pub fn set_treat_as_periodic(&mut self) -> Self {
        self.inner |= Self::TREAT_AS_PERIODIC;
        *self
    }

    pub fn clear_treat_as_periodic(&mut self) -> Self {
        self.inner &= Self::TREAT_AS_PERIODIC.reverse_bits();
        *self
    }

    pub const fn is_can_proc_from_procs(&self) -> bool {
        (self.inner & Self::CAN_PROC_FROM_PROCS) != 0
    }

    pub const fn new_can_proc_from_procs() -> Self {
        Self { inner: Self::CAN_PROC_FROM_PROCS }
    }

    pub fn set_can_proc_from_procs(&mut self) -> Self {
        self.inner |= Self::CAN_PROC_FROM_PROCS;
        *self
    }

    pub fn clear_can_proc_from_procs(&mut self) -> Self {
        self.inner &= Self::CAN_PROC_FROM_PROCS.reverse_bits();
        *self
    }

    pub const fn is_only_proc_on_caster(&self) -> bool {
        (self.inner & Self::ONLY_PROC_ON_CASTER) != 0
    }

    pub const fn new_only_proc_on_caster() -> Self {
        Self { inner: Self::ONLY_PROC_ON_CASTER }
    }

    pub fn set_only_proc_on_caster(&mut self) -> Self {
        self.inner |= Self::ONLY_PROC_ON_CASTER;
        *self
    }

    pub fn clear_only_proc_on_caster(&mut self) -> Self {
        self.inner &= Self::ONLY_PROC_ON_CASTER.reverse_bits();
        *self
    }

    pub const fn is_ignore_caster_and_target_restrictions(&self) -> bool {
        (self.inner & Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS) != 0
    }

    pub const fn new_ignore_caster_and_target_restrictions() -> Self {
        Self { inner: Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS }
    }

    pub fn set_ignore_caster_and_target_restrictions(&mut self) -> Self {
        self.inner |= Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS;
        *self
    }

    pub fn clear_ignore_caster_and_target_restrictions(&mut self) -> Self {
        self.inner &= Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS.reverse_bits();
        *self
    }

    pub const fn is_ignore_caster_modifiers(&self) -> bool {
        (self.inner & Self::IGNORE_CASTER_MODIFIERS) != 0
    }

    pub const fn new_ignore_caster_modifiers() -> Self {
        Self { inner: Self::IGNORE_CASTER_MODIFIERS }
    }

    pub fn set_ignore_caster_modifiers(&mut self) -> Self {
        self.inner |= Self::IGNORE_CASTER_MODIFIERS;
        *self
    }

    pub fn clear_ignore_caster_modifiers(&mut self) -> Self {
        self.inner &= Self::IGNORE_CASTER_MODIFIERS.reverse_bits();
        *self
    }

    pub const fn is_do_not_display_range(&self) -> bool {
        (self.inner & Self::DO_NOT_DISPLAY_RANGE) != 0
    }

    pub const fn new_do_not_display_range() -> Self {
        Self { inner: Self::DO_NOT_DISPLAY_RANGE }
    }

    pub fn set_do_not_display_range(&mut self) -> Self {
        self.inner |= Self::DO_NOT_DISPLAY_RANGE;
        *self
    }

    pub fn clear_do_not_display_range(&mut self) -> Self {
        self.inner &= Self::DO_NOT_DISPLAY_RANGE.reverse_bits();
        *self
    }

    pub const fn is_not_on_aoe_immune(&self) -> bool {
        (self.inner & Self::NOT_ON_AOE_IMMUNE) != 0
    }

    pub const fn new_not_on_aoe_immune() -> Self {
        Self { inner: Self::NOT_ON_AOE_IMMUNE }
    }

    pub fn set_not_on_aoe_immune(&mut self) -> Self {
        self.inner |= Self::NOT_ON_AOE_IMMUNE;
        *self
    }

    pub fn clear_not_on_aoe_immune(&mut self) -> Self {
        self.inner &= Self::NOT_ON_AOE_IMMUNE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AttributesEx3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AttributesEx3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AttributesEx3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AttributesEx3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for AttributesEx3 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AttributesEx3 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AttributesEx3 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AttributesEx3 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AttributesEx3 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AttributesEx3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for AttributesEx3 {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for AttributesEx3 {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for AttributesEx3 {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for AttributesEx3 {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for AttributesEx3 {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for AttributesEx3 {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for AttributesEx3 {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for AttributesEx3 {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for AttributesEx3 {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

