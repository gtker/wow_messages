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

impl Attributes {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const PROC_FAILURE_BURNS_CHARGE: u32 = 0x01;
    pub(crate) const USES_RANGED_SLOT: u32 = 0x02;
    pub(crate) const ON_NEXT_SWING_NO_DAMAGE: u32 = 0x04;
    pub(crate) const NEED_EXOTIC_AMMO: u32 = 0x08;
    pub(crate) const IS_ABILITY: u32 = 0x10;
    pub(crate) const IS_TRADESKILL: u32 = 0x20;
    pub(crate) const PASSIVE: u32 = 0x40;
    pub(crate) const DO_NOT_DISPLAY: u32 = 0x80;
    pub(crate) const DO_NOT_LOG: u32 = 0x100;
    pub(crate) const HELD_ITEM_ONLY: u32 = 0x200;
    pub(crate) const ON_NEXT_SWING: u32 = 0x400;
    pub(crate) const WEARER_CASTS_PROC_TRIGGER: u32 = 0x800;
    pub(crate) const DAYTIME_ONLY: u32 = 0x1000;
    pub(crate) const NIGHT_ONLY: u32 = 0x2000;
    pub(crate) const ONLY_INDOORS: u32 = 0x4000;
    pub(crate) const ONLY_OUTDOORS: u32 = 0x8000;
    pub(crate) const NOT_SHAPESHIFT: u32 = 0x10000;
    pub(crate) const ONLY_STEALTHED: u32 = 0x20000;
    pub(crate) const DO_NOT_SHEATH: u32 = 0x40000;
    pub(crate) const SCALES_WITH_CREATURE_LEVEL: u32 = 0x80000;
    pub(crate) const CANCELS_AUTO_ATTACK_COMBAT: u32 = 0x100000;
    pub(crate) const NO_ACTIVE_DEFENSE: u32 = 0x200000;
    pub(crate) const TRACK_TARGET_IN_CAST_PLAYER_ONLY: u32 = 0x400000;
    pub(crate) const ALLOW_CAST_WHILE_DEAD: u32 = 0x800000;
    pub(crate) const ALLOW_WHILE_MOUNTED: u32 = 0x1000000;
    pub(crate) const COOLDOWN_ON_EVENT: u32 = 0x2000000;
    pub(crate) const AURA_IS_DEBUFF: u32 = 0x4000000;
    pub(crate) const ALLOW_WHILE_SITTING: u32 = 0x8000000;
    pub(crate) const NOT_IN_COMBAT_ONLY_PEACEFUL: u32 = 0x10000000;
    pub(crate) const NO_IMMUNITIES: u32 = 0x20000000;
    pub(crate) const HEARTBEAT_RESIST: u32 = 0x40000000;
    pub(crate) const NO_AURA_CANCEL: u32 = 0x80000000;

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

    pub const fn is_PROC_FAILURE_BURNS_CHARGE(&self) -> bool {
        (self.inner & Self::PROC_FAILURE_BURNS_CHARGE) != 0
    }

    pub const fn new_PROC_FAILURE_BURNS_CHARGE() -> Self {
        Self { inner: Self::PROC_FAILURE_BURNS_CHARGE }
    }

    pub fn set_PROC_FAILURE_BURNS_CHARGE(&mut self) -> Self {
        self.inner |= Self::PROC_FAILURE_BURNS_CHARGE;
        *self
    }

    pub fn clear_PROC_FAILURE_BURNS_CHARGE(&mut self) -> Self {
        self.inner &= Self::PROC_FAILURE_BURNS_CHARGE.reverse_bits();
        *self
    }

    pub const fn is_USES_RANGED_SLOT(&self) -> bool {
        (self.inner & Self::USES_RANGED_SLOT) != 0
    }

    pub const fn new_USES_RANGED_SLOT() -> Self {
        Self { inner: Self::USES_RANGED_SLOT }
    }

    pub fn set_USES_RANGED_SLOT(&mut self) -> Self {
        self.inner |= Self::USES_RANGED_SLOT;
        *self
    }

    pub fn clear_USES_RANGED_SLOT(&mut self) -> Self {
        self.inner &= Self::USES_RANGED_SLOT.reverse_bits();
        *self
    }

    pub const fn is_ON_NEXT_SWING_NO_DAMAGE(&self) -> bool {
        (self.inner & Self::ON_NEXT_SWING_NO_DAMAGE) != 0
    }

    pub const fn new_ON_NEXT_SWING_NO_DAMAGE() -> Self {
        Self { inner: Self::ON_NEXT_SWING_NO_DAMAGE }
    }

    pub fn set_ON_NEXT_SWING_NO_DAMAGE(&mut self) -> Self {
        self.inner |= Self::ON_NEXT_SWING_NO_DAMAGE;
        *self
    }

    pub fn clear_ON_NEXT_SWING_NO_DAMAGE(&mut self) -> Self {
        self.inner &= Self::ON_NEXT_SWING_NO_DAMAGE.reverse_bits();
        *self
    }

    pub const fn is_NEED_EXOTIC_AMMO(&self) -> bool {
        (self.inner & Self::NEED_EXOTIC_AMMO) != 0
    }

    pub const fn new_NEED_EXOTIC_AMMO() -> Self {
        Self { inner: Self::NEED_EXOTIC_AMMO }
    }

    pub fn set_NEED_EXOTIC_AMMO(&mut self) -> Self {
        self.inner |= Self::NEED_EXOTIC_AMMO;
        *self
    }

    pub fn clear_NEED_EXOTIC_AMMO(&mut self) -> Self {
        self.inner &= Self::NEED_EXOTIC_AMMO.reverse_bits();
        *self
    }

    pub const fn is_IS_ABILITY(&self) -> bool {
        (self.inner & Self::IS_ABILITY) != 0
    }

    pub const fn new_IS_ABILITY() -> Self {
        Self { inner: Self::IS_ABILITY }
    }

    pub fn set_IS_ABILITY(&mut self) -> Self {
        self.inner |= Self::IS_ABILITY;
        *self
    }

    pub fn clear_IS_ABILITY(&mut self) -> Self {
        self.inner &= Self::IS_ABILITY.reverse_bits();
        *self
    }

    pub const fn is_IS_TRADESKILL(&self) -> bool {
        (self.inner & Self::IS_TRADESKILL) != 0
    }

    pub const fn new_IS_TRADESKILL() -> Self {
        Self { inner: Self::IS_TRADESKILL }
    }

    pub fn set_IS_TRADESKILL(&mut self) -> Self {
        self.inner |= Self::IS_TRADESKILL;
        *self
    }

    pub fn clear_IS_TRADESKILL(&mut self) -> Self {
        self.inner &= Self::IS_TRADESKILL.reverse_bits();
        *self
    }

    pub const fn is_PASSIVE(&self) -> bool {
        (self.inner & Self::PASSIVE) != 0
    }

    pub const fn new_PASSIVE() -> Self {
        Self { inner: Self::PASSIVE }
    }

    pub fn set_PASSIVE(&mut self) -> Self {
        self.inner |= Self::PASSIVE;
        *self
    }

    pub fn clear_PASSIVE(&mut self) -> Self {
        self.inner &= Self::PASSIVE.reverse_bits();
        *self
    }

    pub const fn is_DO_NOT_DISPLAY(&self) -> bool {
        (self.inner & Self::DO_NOT_DISPLAY) != 0
    }

    pub const fn new_DO_NOT_DISPLAY() -> Self {
        Self { inner: Self::DO_NOT_DISPLAY }
    }

    pub fn set_DO_NOT_DISPLAY(&mut self) -> Self {
        self.inner |= Self::DO_NOT_DISPLAY;
        *self
    }

    pub fn clear_DO_NOT_DISPLAY(&mut self) -> Self {
        self.inner &= Self::DO_NOT_DISPLAY.reverse_bits();
        *self
    }

    pub const fn is_DO_NOT_LOG(&self) -> bool {
        (self.inner & Self::DO_NOT_LOG) != 0
    }

    pub const fn new_DO_NOT_LOG() -> Self {
        Self { inner: Self::DO_NOT_LOG }
    }

    pub fn set_DO_NOT_LOG(&mut self) -> Self {
        self.inner |= Self::DO_NOT_LOG;
        *self
    }

    pub fn clear_DO_NOT_LOG(&mut self) -> Self {
        self.inner &= Self::DO_NOT_LOG.reverse_bits();
        *self
    }

    pub const fn is_HELD_ITEM_ONLY(&self) -> bool {
        (self.inner & Self::HELD_ITEM_ONLY) != 0
    }

    pub const fn new_HELD_ITEM_ONLY() -> Self {
        Self { inner: Self::HELD_ITEM_ONLY }
    }

    pub fn set_HELD_ITEM_ONLY(&mut self) -> Self {
        self.inner |= Self::HELD_ITEM_ONLY;
        *self
    }

    pub fn clear_HELD_ITEM_ONLY(&mut self) -> Self {
        self.inner &= Self::HELD_ITEM_ONLY.reverse_bits();
        *self
    }

    pub const fn is_ON_NEXT_SWING(&self) -> bool {
        (self.inner & Self::ON_NEXT_SWING) != 0
    }

    pub const fn new_ON_NEXT_SWING() -> Self {
        Self { inner: Self::ON_NEXT_SWING }
    }

    pub fn set_ON_NEXT_SWING(&mut self) -> Self {
        self.inner |= Self::ON_NEXT_SWING;
        *self
    }

    pub fn clear_ON_NEXT_SWING(&mut self) -> Self {
        self.inner &= Self::ON_NEXT_SWING.reverse_bits();
        *self
    }

    pub const fn is_WEARER_CASTS_PROC_TRIGGER(&self) -> bool {
        (self.inner & Self::WEARER_CASTS_PROC_TRIGGER) != 0
    }

    pub const fn new_WEARER_CASTS_PROC_TRIGGER() -> Self {
        Self { inner: Self::WEARER_CASTS_PROC_TRIGGER }
    }

    pub fn set_WEARER_CASTS_PROC_TRIGGER(&mut self) -> Self {
        self.inner |= Self::WEARER_CASTS_PROC_TRIGGER;
        *self
    }

    pub fn clear_WEARER_CASTS_PROC_TRIGGER(&mut self) -> Self {
        self.inner &= Self::WEARER_CASTS_PROC_TRIGGER.reverse_bits();
        *self
    }

    pub const fn is_DAYTIME_ONLY(&self) -> bool {
        (self.inner & Self::DAYTIME_ONLY) != 0
    }

    pub const fn new_DAYTIME_ONLY() -> Self {
        Self { inner: Self::DAYTIME_ONLY }
    }

    pub fn set_DAYTIME_ONLY(&mut self) -> Self {
        self.inner |= Self::DAYTIME_ONLY;
        *self
    }

    pub fn clear_DAYTIME_ONLY(&mut self) -> Self {
        self.inner &= Self::DAYTIME_ONLY.reverse_bits();
        *self
    }

    pub const fn is_NIGHT_ONLY(&self) -> bool {
        (self.inner & Self::NIGHT_ONLY) != 0
    }

    pub const fn new_NIGHT_ONLY() -> Self {
        Self { inner: Self::NIGHT_ONLY }
    }

    pub fn set_NIGHT_ONLY(&mut self) -> Self {
        self.inner |= Self::NIGHT_ONLY;
        *self
    }

    pub fn clear_NIGHT_ONLY(&mut self) -> Self {
        self.inner &= Self::NIGHT_ONLY.reverse_bits();
        *self
    }

    pub const fn is_ONLY_INDOORS(&self) -> bool {
        (self.inner & Self::ONLY_INDOORS) != 0
    }

    pub const fn new_ONLY_INDOORS() -> Self {
        Self { inner: Self::ONLY_INDOORS }
    }

    pub fn set_ONLY_INDOORS(&mut self) -> Self {
        self.inner |= Self::ONLY_INDOORS;
        *self
    }

    pub fn clear_ONLY_INDOORS(&mut self) -> Self {
        self.inner &= Self::ONLY_INDOORS.reverse_bits();
        *self
    }

    pub const fn is_ONLY_OUTDOORS(&self) -> bool {
        (self.inner & Self::ONLY_OUTDOORS) != 0
    }

    pub const fn new_ONLY_OUTDOORS() -> Self {
        Self { inner: Self::ONLY_OUTDOORS }
    }

    pub fn set_ONLY_OUTDOORS(&mut self) -> Self {
        self.inner |= Self::ONLY_OUTDOORS;
        *self
    }

    pub fn clear_ONLY_OUTDOORS(&mut self) -> Self {
        self.inner &= Self::ONLY_OUTDOORS.reverse_bits();
        *self
    }

    pub const fn is_NOT_SHAPESHIFT(&self) -> bool {
        (self.inner & Self::NOT_SHAPESHIFT) != 0
    }

    pub const fn new_NOT_SHAPESHIFT() -> Self {
        Self { inner: Self::NOT_SHAPESHIFT }
    }

    pub fn set_NOT_SHAPESHIFT(&mut self) -> Self {
        self.inner |= Self::NOT_SHAPESHIFT;
        *self
    }

    pub fn clear_NOT_SHAPESHIFT(&mut self) -> Self {
        self.inner &= Self::NOT_SHAPESHIFT.reverse_bits();
        *self
    }

    pub const fn is_ONLY_STEALTHED(&self) -> bool {
        (self.inner & Self::ONLY_STEALTHED) != 0
    }

    pub const fn new_ONLY_STEALTHED() -> Self {
        Self { inner: Self::ONLY_STEALTHED }
    }

    pub fn set_ONLY_STEALTHED(&mut self) -> Self {
        self.inner |= Self::ONLY_STEALTHED;
        *self
    }

    pub fn clear_ONLY_STEALTHED(&mut self) -> Self {
        self.inner &= Self::ONLY_STEALTHED.reverse_bits();
        *self
    }

    pub const fn is_DO_NOT_SHEATH(&self) -> bool {
        (self.inner & Self::DO_NOT_SHEATH) != 0
    }

    pub const fn new_DO_NOT_SHEATH() -> Self {
        Self { inner: Self::DO_NOT_SHEATH }
    }

    pub fn set_DO_NOT_SHEATH(&mut self) -> Self {
        self.inner |= Self::DO_NOT_SHEATH;
        *self
    }

    pub fn clear_DO_NOT_SHEATH(&mut self) -> Self {
        self.inner &= Self::DO_NOT_SHEATH.reverse_bits();
        *self
    }

    pub const fn is_SCALES_WITH_CREATURE_LEVEL(&self) -> bool {
        (self.inner & Self::SCALES_WITH_CREATURE_LEVEL) != 0
    }

    pub const fn new_SCALES_WITH_CREATURE_LEVEL() -> Self {
        Self { inner: Self::SCALES_WITH_CREATURE_LEVEL }
    }

    pub fn set_SCALES_WITH_CREATURE_LEVEL(&mut self) -> Self {
        self.inner |= Self::SCALES_WITH_CREATURE_LEVEL;
        *self
    }

    pub fn clear_SCALES_WITH_CREATURE_LEVEL(&mut self) -> Self {
        self.inner &= Self::SCALES_WITH_CREATURE_LEVEL.reverse_bits();
        *self
    }

    pub const fn is_CANCELS_AUTO_ATTACK_COMBAT(&self) -> bool {
        (self.inner & Self::CANCELS_AUTO_ATTACK_COMBAT) != 0
    }

    pub const fn new_CANCELS_AUTO_ATTACK_COMBAT() -> Self {
        Self { inner: Self::CANCELS_AUTO_ATTACK_COMBAT }
    }

    pub fn set_CANCELS_AUTO_ATTACK_COMBAT(&mut self) -> Self {
        self.inner |= Self::CANCELS_AUTO_ATTACK_COMBAT;
        *self
    }

    pub fn clear_CANCELS_AUTO_ATTACK_COMBAT(&mut self) -> Self {
        self.inner &= Self::CANCELS_AUTO_ATTACK_COMBAT.reverse_bits();
        *self
    }

    pub const fn is_NO_ACTIVE_DEFENSE(&self) -> bool {
        (self.inner & Self::NO_ACTIVE_DEFENSE) != 0
    }

    pub const fn new_NO_ACTIVE_DEFENSE() -> Self {
        Self { inner: Self::NO_ACTIVE_DEFENSE }
    }

    pub fn set_NO_ACTIVE_DEFENSE(&mut self) -> Self {
        self.inner |= Self::NO_ACTIVE_DEFENSE;
        *self
    }

    pub fn clear_NO_ACTIVE_DEFENSE(&mut self) -> Self {
        self.inner &= Self::NO_ACTIVE_DEFENSE.reverse_bits();
        *self
    }

    pub const fn is_TRACK_TARGET_IN_CAST_PLAYER_ONLY(&self) -> bool {
        (self.inner & Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY) != 0
    }

    pub const fn new_TRACK_TARGET_IN_CAST_PLAYER_ONLY() -> Self {
        Self { inner: Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY }
    }

    pub fn set_TRACK_TARGET_IN_CAST_PLAYER_ONLY(&mut self) -> Self {
        self.inner |= Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY;
        *self
    }

    pub fn clear_TRACK_TARGET_IN_CAST_PLAYER_ONLY(&mut self) -> Self {
        self.inner &= Self::TRACK_TARGET_IN_CAST_PLAYER_ONLY.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_CAST_WHILE_DEAD(&self) -> bool {
        (self.inner & Self::ALLOW_CAST_WHILE_DEAD) != 0
    }

    pub const fn new_ALLOW_CAST_WHILE_DEAD() -> Self {
        Self { inner: Self::ALLOW_CAST_WHILE_DEAD }
    }

    pub fn set_ALLOW_CAST_WHILE_DEAD(&mut self) -> Self {
        self.inner |= Self::ALLOW_CAST_WHILE_DEAD;
        *self
    }

    pub fn clear_ALLOW_CAST_WHILE_DEAD(&mut self) -> Self {
        self.inner &= Self::ALLOW_CAST_WHILE_DEAD.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_WHILE_MOUNTED(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_MOUNTED) != 0
    }

    pub const fn new_ALLOW_WHILE_MOUNTED() -> Self {
        Self { inner: Self::ALLOW_WHILE_MOUNTED }
    }

    pub fn set_ALLOW_WHILE_MOUNTED(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_MOUNTED;
        *self
    }

    pub fn clear_ALLOW_WHILE_MOUNTED(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_MOUNTED.reverse_bits();
        *self
    }

    pub const fn is_COOLDOWN_ON_EVENT(&self) -> bool {
        (self.inner & Self::COOLDOWN_ON_EVENT) != 0
    }

    pub const fn new_COOLDOWN_ON_EVENT() -> Self {
        Self { inner: Self::COOLDOWN_ON_EVENT }
    }

    pub fn set_COOLDOWN_ON_EVENT(&mut self) -> Self {
        self.inner |= Self::COOLDOWN_ON_EVENT;
        *self
    }

    pub fn clear_COOLDOWN_ON_EVENT(&mut self) -> Self {
        self.inner &= Self::COOLDOWN_ON_EVENT.reverse_bits();
        *self
    }

    pub const fn is_AURA_IS_DEBUFF(&self) -> bool {
        (self.inner & Self::AURA_IS_DEBUFF) != 0
    }

    pub const fn new_AURA_IS_DEBUFF() -> Self {
        Self { inner: Self::AURA_IS_DEBUFF }
    }

    pub fn set_AURA_IS_DEBUFF(&mut self) -> Self {
        self.inner |= Self::AURA_IS_DEBUFF;
        *self
    }

    pub fn clear_AURA_IS_DEBUFF(&mut self) -> Self {
        self.inner &= Self::AURA_IS_DEBUFF.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_WHILE_SITTING(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_SITTING) != 0
    }

    pub const fn new_ALLOW_WHILE_SITTING() -> Self {
        Self { inner: Self::ALLOW_WHILE_SITTING }
    }

    pub fn set_ALLOW_WHILE_SITTING(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_SITTING;
        *self
    }

    pub fn clear_ALLOW_WHILE_SITTING(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_SITTING.reverse_bits();
        *self
    }

    pub const fn is_NOT_IN_COMBAT_ONLY_PEACEFUL(&self) -> bool {
        (self.inner & Self::NOT_IN_COMBAT_ONLY_PEACEFUL) != 0
    }

    pub const fn new_NOT_IN_COMBAT_ONLY_PEACEFUL() -> Self {
        Self { inner: Self::NOT_IN_COMBAT_ONLY_PEACEFUL }
    }

    pub fn set_NOT_IN_COMBAT_ONLY_PEACEFUL(&mut self) -> Self {
        self.inner |= Self::NOT_IN_COMBAT_ONLY_PEACEFUL;
        *self
    }

    pub fn clear_NOT_IN_COMBAT_ONLY_PEACEFUL(&mut self) -> Self {
        self.inner &= Self::NOT_IN_COMBAT_ONLY_PEACEFUL.reverse_bits();
        *self
    }

    pub const fn is_NO_IMMUNITIES(&self) -> bool {
        (self.inner & Self::NO_IMMUNITIES) != 0
    }

    pub const fn new_NO_IMMUNITIES() -> Self {
        Self { inner: Self::NO_IMMUNITIES }
    }

    pub fn set_NO_IMMUNITIES(&mut self) -> Self {
        self.inner |= Self::NO_IMMUNITIES;
        *self
    }

    pub fn clear_NO_IMMUNITIES(&mut self) -> Self {
        self.inner &= Self::NO_IMMUNITIES.reverse_bits();
        *self
    }

    pub const fn is_HEARTBEAT_RESIST(&self) -> bool {
        (self.inner & Self::HEARTBEAT_RESIST) != 0
    }

    pub const fn new_HEARTBEAT_RESIST() -> Self {
        Self { inner: Self::HEARTBEAT_RESIST }
    }

    pub fn set_HEARTBEAT_RESIST(&mut self) -> Self {
        self.inner |= Self::HEARTBEAT_RESIST;
        *self
    }

    pub fn clear_HEARTBEAT_RESIST(&mut self) -> Self {
        self.inner &= Self::HEARTBEAT_RESIST.reverse_bits();
        *self
    }

    pub const fn is_NO_AURA_CANCEL(&self) -> bool {
        (self.inner & Self::NO_AURA_CANCEL) != 0
    }

    pub const fn new_NO_AURA_CANCEL() -> Self {
        Self { inner: Self::NO_AURA_CANCEL }
    }

    pub fn set_NO_AURA_CANCEL(&mut self) -> Self {
        self.inner |= Self::NO_AURA_CANCEL;
        *self
    }

    pub fn clear_NO_AURA_CANCEL(&mut self) -> Self {
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

