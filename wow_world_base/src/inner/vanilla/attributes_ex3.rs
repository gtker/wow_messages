/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external_spell_1_12.wowm:118`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external_spell_1_12.wowm#L118):
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

    pub const fn is_PVP_ENABLING(&self) -> bool {
        (self.inner & Self::PVP_ENABLING) != 0
    }

    pub const fn new_PVP_ENABLING() -> Self {
        Self { inner: Self::PVP_ENABLING }
    }

    pub fn set_PVP_ENABLING(&mut self) -> Self {
        self.inner |= Self::PVP_ENABLING;
        *self
    }

    pub fn clear_PVP_ENABLING(&mut self) -> Self {
        self.inner &= Self::PVP_ENABLING.reverse_bits();
        *self
    }

    pub const fn is_NO_PROC_EQUIP_REQUIREMENT(&self) -> bool {
        (self.inner & Self::NO_PROC_EQUIP_REQUIREMENT) != 0
    }

    pub const fn new_NO_PROC_EQUIP_REQUIREMENT() -> Self {
        Self { inner: Self::NO_PROC_EQUIP_REQUIREMENT }
    }

    pub fn set_NO_PROC_EQUIP_REQUIREMENT(&mut self) -> Self {
        self.inner |= Self::NO_PROC_EQUIP_REQUIREMENT;
        *self
    }

    pub fn clear_NO_PROC_EQUIP_REQUIREMENT(&mut self) -> Self {
        self.inner &= Self::NO_PROC_EQUIP_REQUIREMENT.reverse_bits();
        *self
    }

    pub const fn is_NO_CASTING_BAR_TEXT(&self) -> bool {
        (self.inner & Self::NO_CASTING_BAR_TEXT) != 0
    }

    pub const fn new_NO_CASTING_BAR_TEXT() -> Self {
        Self { inner: Self::NO_CASTING_BAR_TEXT }
    }

    pub fn set_NO_CASTING_BAR_TEXT(&mut self) -> Self {
        self.inner |= Self::NO_CASTING_BAR_TEXT;
        *self
    }

    pub fn clear_NO_CASTING_BAR_TEXT(&mut self) -> Self {
        self.inner &= Self::NO_CASTING_BAR_TEXT.reverse_bits();
        *self
    }

    pub const fn is_COMPLETELY_BLOCKED(&self) -> bool {
        (self.inner & Self::COMPLETELY_BLOCKED) != 0
    }

    pub const fn new_COMPLETELY_BLOCKED() -> Self {
        Self { inner: Self::COMPLETELY_BLOCKED }
    }

    pub fn set_COMPLETELY_BLOCKED(&mut self) -> Self {
        self.inner |= Self::COMPLETELY_BLOCKED;
        *self
    }

    pub fn clear_COMPLETELY_BLOCKED(&mut self) -> Self {
        self.inner &= Self::COMPLETELY_BLOCKED.reverse_bits();
        *self
    }

    pub const fn is_NO_RES_TIMER(&self) -> bool {
        (self.inner & Self::NO_RES_TIMER) != 0
    }

    pub const fn new_NO_RES_TIMER() -> Self {
        Self { inner: Self::NO_RES_TIMER }
    }

    pub fn set_NO_RES_TIMER(&mut self) -> Self {
        self.inner |= Self::NO_RES_TIMER;
        *self
    }

    pub fn clear_NO_RES_TIMER(&mut self) -> Self {
        self.inner &= Self::NO_RES_TIMER.reverse_bits();
        *self
    }

    pub const fn is_NO_DURABILITY_LOSS(&self) -> bool {
        (self.inner & Self::NO_DURABILITY_LOSS) != 0
    }

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

    pub const fn is_NO_AVOIDANCE(&self) -> bool {
        (self.inner & Self::NO_AVOIDANCE) != 0
    }

    pub const fn new_NO_AVOIDANCE() -> Self {
        Self { inner: Self::NO_AVOIDANCE }
    }

    pub fn set_NO_AVOIDANCE(&mut self) -> Self {
        self.inner |= Self::NO_AVOIDANCE;
        *self
    }

    pub fn clear_NO_AVOIDANCE(&mut self) -> Self {
        self.inner &= Self::NO_AVOIDANCE.reverse_bits();
        *self
    }

    pub const fn is_DOT_STACKING_RULE(&self) -> bool {
        (self.inner & Self::DOT_STACKING_RULE) != 0
    }

    pub const fn new_DOT_STACKING_RULE() -> Self {
        Self { inner: Self::DOT_STACKING_RULE }
    }

    pub fn set_DOT_STACKING_RULE(&mut self) -> Self {
        self.inner |= Self::DOT_STACKING_RULE;
        *self
    }

    pub fn clear_DOT_STACKING_RULE(&mut self) -> Self {
        self.inner &= Self::DOT_STACKING_RULE.reverse_bits();
        *self
    }

    pub const fn is_ONLY_ON_PLAYER(&self) -> bool {
        (self.inner & Self::ONLY_ON_PLAYER) != 0
    }

    pub const fn new_ONLY_ON_PLAYER() -> Self {
        Self { inner: Self::ONLY_ON_PLAYER }
    }

    pub fn set_ONLY_ON_PLAYER(&mut self) -> Self {
        self.inner |= Self::ONLY_ON_PLAYER;
        *self
    }

    pub fn clear_ONLY_ON_PLAYER(&mut self) -> Self {
        self.inner &= Self::ONLY_ON_PLAYER.reverse_bits();
        *self
    }

    pub const fn is_NOT_A_PROC(&self) -> bool {
        (self.inner & Self::NOT_A_PROC) != 0
    }

    pub const fn new_NOT_A_PROC() -> Self {
        Self { inner: Self::NOT_A_PROC }
    }

    pub fn set_NOT_A_PROC(&mut self) -> Self {
        self.inner |= Self::NOT_A_PROC;
        *self
    }

    pub fn clear_NOT_A_PROC(&mut self) -> Self {
        self.inner &= Self::NOT_A_PROC.reverse_bits();
        *self
    }

    pub const fn is_REQUIRES_MAIN_HAND_WEAPON(&self) -> bool {
        (self.inner & Self::REQUIRES_MAIN_HAND_WEAPON) != 0
    }

    pub const fn new_REQUIRES_MAIN_HAND_WEAPON() -> Self {
        Self { inner: Self::REQUIRES_MAIN_HAND_WEAPON }
    }

    pub fn set_REQUIRES_MAIN_HAND_WEAPON(&mut self) -> Self {
        self.inner |= Self::REQUIRES_MAIN_HAND_WEAPON;
        *self
    }

    pub fn clear_REQUIRES_MAIN_HAND_WEAPON(&mut self) -> Self {
        self.inner &= Self::REQUIRES_MAIN_HAND_WEAPON.reverse_bits();
        *self
    }

    pub const fn is_ONLY_BATTLEGROUNDS(&self) -> bool {
        (self.inner & Self::ONLY_BATTLEGROUNDS) != 0
    }

    pub const fn new_ONLY_BATTLEGROUNDS() -> Self {
        Self { inner: Self::ONLY_BATTLEGROUNDS }
    }

    pub fn set_ONLY_BATTLEGROUNDS(&mut self) -> Self {
        self.inner |= Self::ONLY_BATTLEGROUNDS;
        *self
    }

    pub fn clear_ONLY_BATTLEGROUNDS(&mut self) -> Self {
        self.inner &= Self::ONLY_BATTLEGROUNDS.reverse_bits();
        *self
    }

    pub const fn is_ONLY_ON_GHOSTS(&self) -> bool {
        (self.inner & Self::ONLY_ON_GHOSTS) != 0
    }

    pub const fn new_ONLY_ON_GHOSTS() -> Self {
        Self { inner: Self::ONLY_ON_GHOSTS }
    }

    pub fn set_ONLY_ON_GHOSTS(&mut self) -> Self {
        self.inner |= Self::ONLY_ON_GHOSTS;
        *self
    }

    pub fn clear_ONLY_ON_GHOSTS(&mut self) -> Self {
        self.inner &= Self::ONLY_ON_GHOSTS.reverse_bits();
        *self
    }

    pub const fn is_HIDE_CHANNEL_BAR(&self) -> bool {
        (self.inner & Self::HIDE_CHANNEL_BAR) != 0
    }

    pub const fn new_HIDE_CHANNEL_BAR() -> Self {
        Self { inner: Self::HIDE_CHANNEL_BAR }
    }

    pub fn set_HIDE_CHANNEL_BAR(&mut self) -> Self {
        self.inner |= Self::HIDE_CHANNEL_BAR;
        *self
    }

    pub fn clear_HIDE_CHANNEL_BAR(&mut self) -> Self {
        self.inner &= Self::HIDE_CHANNEL_BAR.reverse_bits();
        *self
    }

    pub const fn is_HIDE_IN_RAID_FILTER(&self) -> bool {
        (self.inner & Self::HIDE_IN_RAID_FILTER) != 0
    }

    pub const fn new_HIDE_IN_RAID_FILTER() -> Self {
        Self { inner: Self::HIDE_IN_RAID_FILTER }
    }

    pub fn set_HIDE_IN_RAID_FILTER(&mut self) -> Self {
        self.inner |= Self::HIDE_IN_RAID_FILTER;
        *self
    }

    pub fn clear_HIDE_IN_RAID_FILTER(&mut self) -> Self {
        self.inner &= Self::HIDE_IN_RAID_FILTER.reverse_bits();
        *self
    }

    pub const fn is_NORMAL_RANGED_ATTACK(&self) -> bool {
        (self.inner & Self::NORMAL_RANGED_ATTACK) != 0
    }

    pub const fn new_NORMAL_RANGED_ATTACK() -> Self {
        Self { inner: Self::NORMAL_RANGED_ATTACK }
    }

    pub fn set_NORMAL_RANGED_ATTACK(&mut self) -> Self {
        self.inner |= Self::NORMAL_RANGED_ATTACK;
        *self
    }

    pub fn clear_NORMAL_RANGED_ATTACK(&mut self) -> Self {
        self.inner &= Self::NORMAL_RANGED_ATTACK.reverse_bits();
        *self
    }

    pub const fn is_SUPPRESS_CASTER_PROCS(&self) -> bool {
        (self.inner & Self::SUPPRESS_CASTER_PROCS) != 0
    }

    pub const fn new_SUPPRESS_CASTER_PROCS() -> Self {
        Self { inner: Self::SUPPRESS_CASTER_PROCS }
    }

    pub fn set_SUPPRESS_CASTER_PROCS(&mut self) -> Self {
        self.inner |= Self::SUPPRESS_CASTER_PROCS;
        *self
    }

    pub fn clear_SUPPRESS_CASTER_PROCS(&mut self) -> Self {
        self.inner &= Self::SUPPRESS_CASTER_PROCS.reverse_bits();
        *self
    }

    pub const fn is_SUPPRESS_TARGET_PROCS(&self) -> bool {
        (self.inner & Self::SUPPRESS_TARGET_PROCS) != 0
    }

    pub const fn new_SUPPRESS_TARGET_PROCS() -> Self {
        Self { inner: Self::SUPPRESS_TARGET_PROCS }
    }

    pub fn set_SUPPRESS_TARGET_PROCS(&mut self) -> Self {
        self.inner |= Self::SUPPRESS_TARGET_PROCS;
        *self
    }

    pub fn clear_SUPPRESS_TARGET_PROCS(&mut self) -> Self {
        self.inner &= Self::SUPPRESS_TARGET_PROCS.reverse_bits();
        *self
    }

    pub const fn is_ALWAYS_HIT(&self) -> bool {
        (self.inner & Self::ALWAYS_HIT) != 0
    }

    pub const fn new_ALWAYS_HIT() -> Self {
        Self { inner: Self::ALWAYS_HIT }
    }

    pub fn set_ALWAYS_HIT(&mut self) -> Self {
        self.inner |= Self::ALWAYS_HIT;
        *self
    }

    pub fn clear_ALWAYS_HIT(&mut self) -> Self {
        self.inner &= Self::ALWAYS_HIT.reverse_bits();
        *self
    }

    pub const fn is_INSTANT_TARGET_PROCS(&self) -> bool {
        (self.inner & Self::INSTANT_TARGET_PROCS) != 0
    }

    pub const fn new_INSTANT_TARGET_PROCS() -> Self {
        Self { inner: Self::INSTANT_TARGET_PROCS }
    }

    pub fn set_INSTANT_TARGET_PROCS(&mut self) -> Self {
        self.inner |= Self::INSTANT_TARGET_PROCS;
        *self
    }

    pub fn clear_INSTANT_TARGET_PROCS(&mut self) -> Self {
        self.inner &= Self::INSTANT_TARGET_PROCS.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_AURA_WHILE_DEAD(&self) -> bool {
        (self.inner & Self::ALLOW_AURA_WHILE_DEAD) != 0
    }

    pub const fn new_ALLOW_AURA_WHILE_DEAD() -> Self {
        Self { inner: Self::ALLOW_AURA_WHILE_DEAD }
    }

    pub fn set_ALLOW_AURA_WHILE_DEAD(&mut self) -> Self {
        self.inner |= Self::ALLOW_AURA_WHILE_DEAD;
        *self
    }

    pub fn clear_ALLOW_AURA_WHILE_DEAD(&mut self) -> Self {
        self.inner &= Self::ALLOW_AURA_WHILE_DEAD.reverse_bits();
        *self
    }

    pub const fn is_ONLY_PROC_OUTDOORS(&self) -> bool {
        (self.inner & Self::ONLY_PROC_OUTDOORS) != 0
    }

    pub const fn new_ONLY_PROC_OUTDOORS() -> Self {
        Self { inner: Self::ONLY_PROC_OUTDOORS }
    }

    pub fn set_ONLY_PROC_OUTDOORS(&mut self) -> Self {
        self.inner |= Self::ONLY_PROC_OUTDOORS;
        *self
    }

    pub fn clear_ONLY_PROC_OUTDOORS(&mut self) -> Self {
        self.inner &= Self::ONLY_PROC_OUTDOORS.reverse_bits();
        *self
    }

    pub const fn is_CASTING_CANCELS_AUTOREPEAT(&self) -> bool {
        (self.inner & Self::CASTING_CANCELS_AUTOREPEAT) != 0
    }

    pub const fn new_CASTING_CANCELS_AUTOREPEAT() -> Self {
        Self { inner: Self::CASTING_CANCELS_AUTOREPEAT }
    }

    pub fn set_CASTING_CANCELS_AUTOREPEAT(&mut self) -> Self {
        self.inner |= Self::CASTING_CANCELS_AUTOREPEAT;
        *self
    }

    pub fn clear_CASTING_CANCELS_AUTOREPEAT(&mut self) -> Self {
        self.inner &= Self::CASTING_CANCELS_AUTOREPEAT.reverse_bits();
        *self
    }

    pub const fn is_NO_DAMAGE_HISTORY(&self) -> bool {
        (self.inner & Self::NO_DAMAGE_HISTORY) != 0
    }

    pub const fn new_NO_DAMAGE_HISTORY() -> Self {
        Self { inner: Self::NO_DAMAGE_HISTORY }
    }

    pub fn set_NO_DAMAGE_HISTORY(&mut self) -> Self {
        self.inner |= Self::NO_DAMAGE_HISTORY;
        *self
    }

    pub fn clear_NO_DAMAGE_HISTORY(&mut self) -> Self {
        self.inner &= Self::NO_DAMAGE_HISTORY.reverse_bits();
        *self
    }

    pub const fn is_REQUIRES_OFFHAND_WEAPON(&self) -> bool {
        (self.inner & Self::REQUIRES_OFFHAND_WEAPON) != 0
    }

    pub const fn new_REQUIRES_OFFHAND_WEAPON() -> Self {
        Self { inner: Self::REQUIRES_OFFHAND_WEAPON }
    }

    pub fn set_REQUIRES_OFFHAND_WEAPON(&mut self) -> Self {
        self.inner |= Self::REQUIRES_OFFHAND_WEAPON;
        *self
    }

    pub fn clear_REQUIRES_OFFHAND_WEAPON(&mut self) -> Self {
        self.inner &= Self::REQUIRES_OFFHAND_WEAPON.reverse_bits();
        *self
    }

    pub const fn is_TREAT_AS_PERIODIC(&self) -> bool {
        (self.inner & Self::TREAT_AS_PERIODIC) != 0
    }

    pub const fn new_TREAT_AS_PERIODIC() -> Self {
        Self { inner: Self::TREAT_AS_PERIODIC }
    }

    pub fn set_TREAT_AS_PERIODIC(&mut self) -> Self {
        self.inner |= Self::TREAT_AS_PERIODIC;
        *self
    }

    pub fn clear_TREAT_AS_PERIODIC(&mut self) -> Self {
        self.inner &= Self::TREAT_AS_PERIODIC.reverse_bits();
        *self
    }

    pub const fn is_CAN_PROC_FROM_PROCS(&self) -> bool {
        (self.inner & Self::CAN_PROC_FROM_PROCS) != 0
    }

    pub const fn new_CAN_PROC_FROM_PROCS() -> Self {
        Self { inner: Self::CAN_PROC_FROM_PROCS }
    }

    pub fn set_CAN_PROC_FROM_PROCS(&mut self) -> Self {
        self.inner |= Self::CAN_PROC_FROM_PROCS;
        *self
    }

    pub fn clear_CAN_PROC_FROM_PROCS(&mut self) -> Self {
        self.inner &= Self::CAN_PROC_FROM_PROCS.reverse_bits();
        *self
    }

    pub const fn is_ONLY_PROC_ON_CASTER(&self) -> bool {
        (self.inner & Self::ONLY_PROC_ON_CASTER) != 0
    }

    pub const fn new_ONLY_PROC_ON_CASTER() -> Self {
        Self { inner: Self::ONLY_PROC_ON_CASTER }
    }

    pub fn set_ONLY_PROC_ON_CASTER(&mut self) -> Self {
        self.inner |= Self::ONLY_PROC_ON_CASTER;
        *self
    }

    pub fn clear_ONLY_PROC_ON_CASTER(&mut self) -> Self {
        self.inner &= Self::ONLY_PROC_ON_CASTER.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_CASTER_AND_TARGET_RESTRICTIONS(&self) -> bool {
        (self.inner & Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS) != 0
    }

    pub const fn new_IGNORE_CASTER_AND_TARGET_RESTRICTIONS() -> Self {
        Self { inner: Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS }
    }

    pub fn set_IGNORE_CASTER_AND_TARGET_RESTRICTIONS(&mut self) -> Self {
        self.inner |= Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS;
        *self
    }

    pub fn clear_IGNORE_CASTER_AND_TARGET_RESTRICTIONS(&mut self) -> Self {
        self.inner &= Self::IGNORE_CASTER_AND_TARGET_RESTRICTIONS.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_CASTER_MODIFIERS(&self) -> bool {
        (self.inner & Self::IGNORE_CASTER_MODIFIERS) != 0
    }

    pub const fn new_IGNORE_CASTER_MODIFIERS() -> Self {
        Self { inner: Self::IGNORE_CASTER_MODIFIERS }
    }

    pub fn set_IGNORE_CASTER_MODIFIERS(&mut self) -> Self {
        self.inner |= Self::IGNORE_CASTER_MODIFIERS;
        *self
    }

    pub fn clear_IGNORE_CASTER_MODIFIERS(&mut self) -> Self {
        self.inner &= Self::IGNORE_CASTER_MODIFIERS.reverse_bits();
        *self
    }

    pub const fn is_DO_NOT_DISPLAY_RANGE(&self) -> bool {
        (self.inner & Self::DO_NOT_DISPLAY_RANGE) != 0
    }

    pub const fn new_DO_NOT_DISPLAY_RANGE() -> Self {
        Self { inner: Self::DO_NOT_DISPLAY_RANGE }
    }

    pub fn set_DO_NOT_DISPLAY_RANGE(&mut self) -> Self {
        self.inner |= Self::DO_NOT_DISPLAY_RANGE;
        *self
    }

    pub fn clear_DO_NOT_DISPLAY_RANGE(&mut self) -> Self {
        self.inner &= Self::DO_NOT_DISPLAY_RANGE.reverse_bits();
        *self
    }

    pub const fn is_NOT_ON_AOE_IMMUNE(&self) -> bool {
        (self.inner & Self::NOT_ON_AOE_IMMUNE) != 0
    }

    pub const fn new_NOT_ON_AOE_IMMUNE() -> Self {
        Self { inner: Self::NOT_ON_AOE_IMMUNE }
    }

    pub fn set_NOT_ON_AOE_IMMUNE(&mut self) -> Self {
        self.inner |= Self::NOT_ON_AOE_IMMUNE;
        *self
    }

    pub fn clear_NOT_ON_AOE_IMMUNE(&mut self) -> Self {
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

