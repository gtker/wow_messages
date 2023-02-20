/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external_spell_1_12.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external_spell_1_12.wowm#L40):
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

    pub const fn is_DISMISS_PET_FIRST(&self) -> bool {
        (self.inner & Self::DISMISS_PET_FIRST) != 0
    }

    pub const fn new_DISMISS_PET_FIRST() -> Self {
        Self { inner: Self::DISMISS_PET_FIRST }
    }

    pub fn set_DISMISS_PET_FIRST(&mut self) -> Self {
        self.inner |= Self::DISMISS_PET_FIRST;
        *self
    }

    pub fn clear_DISMISS_PET_FIRST(&mut self) -> Self {
        self.inner &= Self::DISMISS_PET_FIRST.reverse_bits();
        *self
    }

    pub const fn is_USE_ALL_MANA(&self) -> bool {
        (self.inner & Self::USE_ALL_MANA) != 0
    }

    pub const fn new_USE_ALL_MANA() -> Self {
        Self { inner: Self::USE_ALL_MANA }
    }

    pub fn set_USE_ALL_MANA(&mut self) -> Self {
        self.inner |= Self::USE_ALL_MANA;
        *self
    }

    pub fn clear_USE_ALL_MANA(&mut self) -> Self {
        self.inner &= Self::USE_ALL_MANA.reverse_bits();
        *self
    }

    pub const fn is_IS_CHANNELED(&self) -> bool {
        (self.inner & Self::IS_CHANNELED) != 0
    }

    pub const fn new_IS_CHANNELED() -> Self {
        Self { inner: Self::IS_CHANNELED }
    }

    pub fn set_IS_CHANNELED(&mut self) -> Self {
        self.inner |= Self::IS_CHANNELED;
        *self
    }

    pub fn clear_IS_CHANNELED(&mut self) -> Self {
        self.inner &= Self::IS_CHANNELED.reverse_bits();
        *self
    }

    pub const fn is_NO_REDIRECTION(&self) -> bool {
        (self.inner & Self::NO_REDIRECTION) != 0
    }

    pub const fn new_NO_REDIRECTION() -> Self {
        Self { inner: Self::NO_REDIRECTION }
    }

    pub fn set_NO_REDIRECTION(&mut self) -> Self {
        self.inner |= Self::NO_REDIRECTION;
        *self
    }

    pub fn clear_NO_REDIRECTION(&mut self) -> Self {
        self.inner &= Self::NO_REDIRECTION.reverse_bits();
        *self
    }

    pub const fn is_NO_SKILL_INCREASE(&self) -> bool {
        (self.inner & Self::NO_SKILL_INCREASE) != 0
    }

    pub const fn new_NO_SKILL_INCREASE() -> Self {
        Self { inner: Self::NO_SKILL_INCREASE }
    }

    pub fn set_NO_SKILL_INCREASE(&mut self) -> Self {
        self.inner |= Self::NO_SKILL_INCREASE;
        *self
    }

    pub fn clear_NO_SKILL_INCREASE(&mut self) -> Self {
        self.inner &= Self::NO_SKILL_INCREASE.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_WHILE_STEALTHED(&self) -> bool {
        (self.inner & Self::ALLOW_WHILE_STEALTHED) != 0
    }

    pub const fn new_ALLOW_WHILE_STEALTHED() -> Self {
        Self { inner: Self::ALLOW_WHILE_STEALTHED }
    }

    pub fn set_ALLOW_WHILE_STEALTHED(&mut self) -> Self {
        self.inner |= Self::ALLOW_WHILE_STEALTHED;
        *self
    }

    pub fn clear_ALLOW_WHILE_STEALTHED(&mut self) -> Self {
        self.inner &= Self::ALLOW_WHILE_STEALTHED.reverse_bits();
        *self
    }

    pub const fn is_IS_SELF_CHANNELED(&self) -> bool {
        (self.inner & Self::IS_SELF_CHANNELED) != 0
    }

    pub const fn new_IS_SELF_CHANNELED() -> Self {
        Self { inner: Self::IS_SELF_CHANNELED }
    }

    pub fn set_IS_SELF_CHANNELED(&mut self) -> Self {
        self.inner |= Self::IS_SELF_CHANNELED;
        *self
    }

    pub fn clear_IS_SELF_CHANNELED(&mut self) -> Self {
        self.inner &= Self::IS_SELF_CHANNELED.reverse_bits();
        *self
    }

    pub const fn is_NO_REFLECTION(&self) -> bool {
        (self.inner & Self::NO_REFLECTION) != 0
    }

    pub const fn new_NO_REFLECTION() -> Self {
        Self { inner: Self::NO_REFLECTION }
    }

    pub fn set_NO_REFLECTION(&mut self) -> Self {
        self.inner |= Self::NO_REFLECTION;
        *self
    }

    pub fn clear_NO_REFLECTION(&mut self) -> Self {
        self.inner &= Self::NO_REFLECTION.reverse_bits();
        *self
    }

    pub const fn is_ONLY_PEACEFUL_TARGETS(&self) -> bool {
        (self.inner & Self::ONLY_PEACEFUL_TARGETS) != 0
    }

    pub const fn new_ONLY_PEACEFUL_TARGETS() -> Self {
        Self { inner: Self::ONLY_PEACEFUL_TARGETS }
    }

    pub fn set_ONLY_PEACEFUL_TARGETS(&mut self) -> Self {
        self.inner |= Self::ONLY_PEACEFUL_TARGETS;
        *self
    }

    pub fn clear_ONLY_PEACEFUL_TARGETS(&mut self) -> Self {
        self.inner &= Self::ONLY_PEACEFUL_TARGETS.reverse_bits();
        *self
    }

    pub const fn is_INITIATES_COMBAT_ENABLES_AUTO_ATTACK(&self) -> bool {
        (self.inner & Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK) != 0
    }

    pub const fn new_INITIATES_COMBAT_ENABLES_AUTO_ATTACK() -> Self {
        Self { inner: Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK }
    }

    pub fn set_INITIATES_COMBAT_ENABLES_AUTO_ATTACK(&mut self) -> Self {
        self.inner |= Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK;
        *self
    }

    pub fn clear_INITIATES_COMBAT_ENABLES_AUTO_ATTACK(&mut self) -> Self {
        self.inner &= Self::INITIATES_COMBAT_ENABLES_AUTO_ATTACK.reverse_bits();
        *self
    }

    pub const fn is_NO_THREAT(&self) -> bool {
        (self.inner & Self::NO_THREAT) != 0
    }

    pub const fn new_NO_THREAT() -> Self {
        Self { inner: Self::NO_THREAT }
    }

    pub fn set_NO_THREAT(&mut self) -> Self {
        self.inner |= Self::NO_THREAT;
        *self
    }

    pub fn clear_NO_THREAT(&mut self) -> Self {
        self.inner &= Self::NO_THREAT.reverse_bits();
        *self
    }

    pub const fn is_AURA_UNIQUE(&self) -> bool {
        (self.inner & Self::AURA_UNIQUE) != 0
    }

    pub const fn new_AURA_UNIQUE() -> Self {
        Self { inner: Self::AURA_UNIQUE }
    }

    pub fn set_AURA_UNIQUE(&mut self) -> Self {
        self.inner |= Self::AURA_UNIQUE;
        *self
    }

    pub fn clear_AURA_UNIQUE(&mut self) -> Self {
        self.inner &= Self::AURA_UNIQUE.reverse_bits();
        *self
    }

    pub const fn is_FAILURE_BREAKS_STEALTH(&self) -> bool {
        (self.inner & Self::FAILURE_BREAKS_STEALTH) != 0
    }

    pub const fn new_FAILURE_BREAKS_STEALTH() -> Self {
        Self { inner: Self::FAILURE_BREAKS_STEALTH }
    }

    pub fn set_FAILURE_BREAKS_STEALTH(&mut self) -> Self {
        self.inner |= Self::FAILURE_BREAKS_STEALTH;
        *self
    }

    pub fn clear_FAILURE_BREAKS_STEALTH(&mut self) -> Self {
        self.inner &= Self::FAILURE_BREAKS_STEALTH.reverse_bits();
        *self
    }

    pub const fn is_TOGGLE_FARSIGHT(&self) -> bool {
        (self.inner & Self::TOGGLE_FARSIGHT) != 0
    }

    pub const fn new_TOGGLE_FARSIGHT() -> Self {
        Self { inner: Self::TOGGLE_FARSIGHT }
    }

    pub fn set_TOGGLE_FARSIGHT(&mut self) -> Self {
        self.inner |= Self::TOGGLE_FARSIGHT;
        *self
    }

    pub fn clear_TOGGLE_FARSIGHT(&mut self) -> Self {
        self.inner &= Self::TOGGLE_FARSIGHT.reverse_bits();
        *self
    }

    pub const fn is_TRACK_TARGET_IN_CHANNEL(&self) -> bool {
        (self.inner & Self::TRACK_TARGET_IN_CHANNEL) != 0
    }

    pub const fn new_TRACK_TARGET_IN_CHANNEL() -> Self {
        Self { inner: Self::TRACK_TARGET_IN_CHANNEL }
    }

    pub fn set_TRACK_TARGET_IN_CHANNEL(&mut self) -> Self {
        self.inner |= Self::TRACK_TARGET_IN_CHANNEL;
        *self
    }

    pub fn clear_TRACK_TARGET_IN_CHANNEL(&mut self) -> Self {
        self.inner &= Self::TRACK_TARGET_IN_CHANNEL.reverse_bits();
        *self
    }

    pub const fn is_IMMUNITY_PURGES_EFFECT(&self) -> bool {
        (self.inner & Self::IMMUNITY_PURGES_EFFECT) != 0
    }

    pub const fn new_IMMUNITY_PURGES_EFFECT() -> Self {
        Self { inner: Self::IMMUNITY_PURGES_EFFECT }
    }

    pub fn set_IMMUNITY_PURGES_EFFECT(&mut self) -> Self {
        self.inner |= Self::IMMUNITY_PURGES_EFFECT;
        *self
    }

    pub fn clear_IMMUNITY_PURGES_EFFECT(&mut self) -> Self {
        self.inner &= Self::IMMUNITY_PURGES_EFFECT.reverse_bits();
        *self
    }

    pub const fn is_IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS(&self) -> bool {
        (self.inner & Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS) != 0
    }

    pub const fn new_IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS() -> Self {
        Self { inner: Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS }
    }

    pub fn set_IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS(&mut self) -> Self {
        self.inner |= Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS;
        *self
    }

    pub fn clear_IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS(&mut self) -> Self {
        self.inner &= Self::IMMUNITY_TO_HOSTILE_AND_FRIENDLY_EFFECTS.reverse_bits();
        *self
    }

    pub const fn is_NO_AUTOCAST_AI(&self) -> bool {
        (self.inner & Self::NO_AUTOCAST_AI) != 0
    }

    pub const fn new_NO_AUTOCAST_AI() -> Self {
        Self { inner: Self::NO_AUTOCAST_AI }
    }

    pub fn set_NO_AUTOCAST_AI(&mut self) -> Self {
        self.inner |= Self::NO_AUTOCAST_AI;
        *self
    }

    pub fn clear_NO_AUTOCAST_AI(&mut self) -> Self {
        self.inner &= Self::NO_AUTOCAST_AI.reverse_bits();
        *self
    }

    pub const fn is_PREVENTS_ANIM(&self) -> bool {
        (self.inner & Self::PREVENTS_ANIM) != 0
    }

    pub const fn new_PREVENTS_ANIM() -> Self {
        Self { inner: Self::PREVENTS_ANIM }
    }

    pub fn set_PREVENTS_ANIM(&mut self) -> Self {
        self.inner |= Self::PREVENTS_ANIM;
        *self
    }

    pub fn clear_PREVENTS_ANIM(&mut self) -> Self {
        self.inner &= Self::PREVENTS_ANIM.reverse_bits();
        *self
    }

    pub const fn is_EXCLUDE_CASTER(&self) -> bool {
        (self.inner & Self::EXCLUDE_CASTER) != 0
    }

    pub const fn new_EXCLUDE_CASTER() -> Self {
        Self { inner: Self::EXCLUDE_CASTER }
    }

    pub fn set_EXCLUDE_CASTER(&mut self) -> Self {
        self.inner |= Self::EXCLUDE_CASTER;
        *self
    }

    pub fn clear_EXCLUDE_CASTER(&mut self) -> Self {
        self.inner &= Self::EXCLUDE_CASTER.reverse_bits();
        *self
    }

    pub const fn is_FINISHING_MOVE_DAMAGE(&self) -> bool {
        (self.inner & Self::FINISHING_MOVE_DAMAGE) != 0
    }

    pub const fn new_FINISHING_MOVE_DAMAGE() -> Self {
        Self { inner: Self::FINISHING_MOVE_DAMAGE }
    }

    pub fn set_FINISHING_MOVE_DAMAGE(&mut self) -> Self {
        self.inner |= Self::FINISHING_MOVE_DAMAGE;
        *self
    }

    pub fn clear_FINISHING_MOVE_DAMAGE(&mut self) -> Self {
        self.inner &= Self::FINISHING_MOVE_DAMAGE.reverse_bits();
        *self
    }

    pub const fn is_THREAT_ONLY_ON_MISS(&self) -> bool {
        (self.inner & Self::THREAT_ONLY_ON_MISS) != 0
    }

    pub const fn new_THREAT_ONLY_ON_MISS() -> Self {
        Self { inner: Self::THREAT_ONLY_ON_MISS }
    }

    pub fn set_THREAT_ONLY_ON_MISS(&mut self) -> Self {
        self.inner |= Self::THREAT_ONLY_ON_MISS;
        *self
    }

    pub fn clear_THREAT_ONLY_ON_MISS(&mut self) -> Self {
        self.inner &= Self::THREAT_ONLY_ON_MISS.reverse_bits();
        *self
    }

    pub const fn is_FINISHING_MOVE_DURATION(&self) -> bool {
        (self.inner & Self::FINISHING_MOVE_DURATION) != 0
    }

    pub const fn new_FINISHING_MOVE_DURATION() -> Self {
        Self { inner: Self::FINISHING_MOVE_DURATION }
    }

    pub fn set_FINISHING_MOVE_DURATION(&mut self) -> Self {
        self.inner |= Self::FINISHING_MOVE_DURATION;
        *self
    }

    pub fn clear_FINISHING_MOVE_DURATION(&mut self) -> Self {
        self.inner &= Self::FINISHING_MOVE_DURATION.reverse_bits();
        *self
    }

    pub const fn is_UNK23(&self) -> bool {
        (self.inner & Self::UNK23) != 0
    }

    pub const fn new_UNK23() -> Self {
        Self { inner: Self::UNK23 }
    }

    pub fn set_UNK23(&mut self) -> Self {
        self.inner |= Self::UNK23;
        *self
    }

    pub fn clear_UNK23(&mut self) -> Self {
        self.inner &= Self::UNK23.reverse_bits();
        *self
    }

    pub const fn is_SPECIAL_SKILLUP(&self) -> bool {
        (self.inner & Self::SPECIAL_SKILLUP) != 0
    }

    pub const fn new_SPECIAL_SKILLUP() -> Self {
        Self { inner: Self::SPECIAL_SKILLUP }
    }

    pub fn set_SPECIAL_SKILLUP(&mut self) -> Self {
        self.inner |= Self::SPECIAL_SKILLUP;
        *self
    }

    pub fn clear_SPECIAL_SKILLUP(&mut self) -> Self {
        self.inner &= Self::SPECIAL_SKILLUP.reverse_bits();
        *self
    }

    pub const fn is_AURA_STAYS_AFTER_COMBAT(&self) -> bool {
        (self.inner & Self::AURA_STAYS_AFTER_COMBAT) != 0
    }

    pub const fn new_AURA_STAYS_AFTER_COMBAT() -> Self {
        Self { inner: Self::AURA_STAYS_AFTER_COMBAT }
    }

    pub fn set_AURA_STAYS_AFTER_COMBAT(&mut self) -> Self {
        self.inner |= Self::AURA_STAYS_AFTER_COMBAT;
        *self
    }

    pub fn clear_AURA_STAYS_AFTER_COMBAT(&mut self) -> Self {
        self.inner &= Self::AURA_STAYS_AFTER_COMBAT.reverse_bits();
        *self
    }

    pub const fn is_REQUIRE_ALL_TARGETS(&self) -> bool {
        (self.inner & Self::REQUIRE_ALL_TARGETS) != 0
    }

    pub const fn new_REQUIRE_ALL_TARGETS() -> Self {
        Self { inner: Self::REQUIRE_ALL_TARGETS }
    }

    pub fn set_REQUIRE_ALL_TARGETS(&mut self) -> Self {
        self.inner |= Self::REQUIRE_ALL_TARGETS;
        *self
    }

    pub fn clear_REQUIRE_ALL_TARGETS(&mut self) -> Self {
        self.inner &= Self::REQUIRE_ALL_TARGETS.reverse_bits();
        *self
    }

    pub const fn is_DISCOUNT_POWER_ON_MISS(&self) -> bool {
        (self.inner & Self::DISCOUNT_POWER_ON_MISS) != 0
    }

    pub const fn new_DISCOUNT_POWER_ON_MISS() -> Self {
        Self { inner: Self::DISCOUNT_POWER_ON_MISS }
    }

    pub fn set_DISCOUNT_POWER_ON_MISS(&mut self) -> Self {
        self.inner |= Self::DISCOUNT_POWER_ON_MISS;
        *self
    }

    pub fn clear_DISCOUNT_POWER_ON_MISS(&mut self) -> Self {
        self.inner &= Self::DISCOUNT_POWER_ON_MISS.reverse_bits();
        *self
    }

    pub const fn is_NO_AURA_ICON(&self) -> bool {
        (self.inner & Self::NO_AURA_ICON) != 0
    }

    pub const fn new_NO_AURA_ICON() -> Self {
        Self { inner: Self::NO_AURA_ICON }
    }

    pub fn set_NO_AURA_ICON(&mut self) -> Self {
        self.inner |= Self::NO_AURA_ICON;
        *self
    }

    pub fn clear_NO_AURA_ICON(&mut self) -> Self {
        self.inner &= Self::NO_AURA_ICON.reverse_bits();
        *self
    }

    pub const fn is_NAME_IN_CHANNEL_BAR(&self) -> bool {
        (self.inner & Self::NAME_IN_CHANNEL_BAR) != 0
    }

    pub const fn new_NAME_IN_CHANNEL_BAR() -> Self {
        Self { inner: Self::NAME_IN_CHANNEL_BAR }
    }

    pub fn set_NAME_IN_CHANNEL_BAR(&mut self) -> Self {
        self.inner |= Self::NAME_IN_CHANNEL_BAR;
        *self
    }

    pub fn clear_NAME_IN_CHANNEL_BAR(&mut self) -> Self {
        self.inner &= Self::NAME_IN_CHANNEL_BAR.reverse_bits();
        *self
    }

    pub const fn is_COMBO_ON_BLOCK(&self) -> bool {
        (self.inner & Self::COMBO_ON_BLOCK) != 0
    }

    pub const fn new_COMBO_ON_BLOCK() -> Self {
        Self { inner: Self::COMBO_ON_BLOCK }
    }

    pub fn set_COMBO_ON_BLOCK(&mut self) -> Self {
        self.inner |= Self::COMBO_ON_BLOCK;
        *self
    }

    pub fn clear_COMBO_ON_BLOCK(&mut self) -> Self {
        self.inner &= Self::COMBO_ON_BLOCK.reverse_bits();
        *self
    }

    pub const fn is_CAST_WHEN_LEARNED(&self) -> bool {
        (self.inner & Self::CAST_WHEN_LEARNED) != 0
    }

    pub const fn new_CAST_WHEN_LEARNED() -> Self {
        Self { inner: Self::CAST_WHEN_LEARNED }
    }

    pub fn set_CAST_WHEN_LEARNED(&mut self) -> Self {
        self.inner |= Self::CAST_WHEN_LEARNED;
        *self
    }

    pub fn clear_CAST_WHEN_LEARNED(&mut self) -> Self {
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

