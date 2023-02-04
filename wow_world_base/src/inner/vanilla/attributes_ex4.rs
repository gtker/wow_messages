/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external_spell_1_12.wowm:157`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external_spell_1_12.wowm#L157):
/// ```text
/// flag AttributesEx4 : u32 {
///     NONE = 0x00;
///     NO_CAST_LOG = 0x00000001;
///     CLASS_TRIGGER_ONLY_ON_TARGET = 0x00000002;
///     AURA_EXPIRES_OFFLINE = 0x00000004;
///     NO_HELPFUL_THREAT = 0x00000008;
///     NO_HARMFUL_THREAT = 0x00000010;
///     ALLOW_CLIENT_TARGETING = 0x00000020;
///     CANNOT_BE_STOLEN = 0x00000040;
///     ALLOW_CAST_WHILE_CASTING = 0x00000080;
///     IGNORE_DAMAGE_TAKEN_MODIFIERS = 0x00000100;
///     COMBAT_FEEDBACK_WHEN_USABLE = 0x00000200;
///     WEAPON_SPEED_COST_SCALING = 0x00000400;
///     NO_PARTIAL_IMMUNITY = 0x00000800;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct AttributesEx4 {
    inner: u32,
}

impl AttributesEx4 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const NO_CAST_LOG: u32 = 0x01;
    pub(crate) const CLASS_TRIGGER_ONLY_ON_TARGET: u32 = 0x02;
    pub(crate) const AURA_EXPIRES_OFFLINE: u32 = 0x04;
    pub(crate) const NO_HELPFUL_THREAT: u32 = 0x08;
    pub(crate) const NO_HARMFUL_THREAT: u32 = 0x10;
    pub(crate) const ALLOW_CLIENT_TARGETING: u32 = 0x20;
    pub(crate) const CANNOT_BE_STOLEN: u32 = 0x40;
    pub(crate) const ALLOW_CAST_WHILE_CASTING: u32 = 0x80;
    pub(crate) const IGNORE_DAMAGE_TAKEN_MODIFIERS: u32 = 0x100;
    pub(crate) const COMBAT_FEEDBACK_WHEN_USABLE: u32 = 0x200;
    pub(crate) const WEAPON_SPEED_COST_SCALING: u32 = 0x400;
    pub(crate) const NO_PARTIAL_IMMUNITY: u32 = 0x800;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::NO_CAST_LOG
                | Self::CLASS_TRIGGER_ONLY_ON_TARGET
                | Self::AURA_EXPIRES_OFFLINE
                | Self::NO_HELPFUL_THREAT
                | Self::NO_HARMFUL_THREAT
                | Self::ALLOW_CLIENT_TARGETING
                | Self::CANNOT_BE_STOLEN
                | Self::ALLOW_CAST_WHILE_CASTING
                | Self::IGNORE_DAMAGE_TAKEN_MODIFIERS
                | Self::COMBAT_FEEDBACK_WHEN_USABLE
                | Self::WEAPON_SPEED_COST_SCALING
                | Self::NO_PARTIAL_IMMUNITY
        }
    }

    pub const fn is_NO_CAST_LOG(&self) -> bool {
        (self.inner & Self::NO_CAST_LOG) != 0
    }

    pub const fn new_NO_CAST_LOG() -> Self {
        Self { inner: Self::NO_CAST_LOG }
    }

    pub fn set_NO_CAST_LOG(&mut self) -> Self {
        self.inner |= Self::NO_CAST_LOG;
        *self
    }

    pub fn clear_NO_CAST_LOG(&mut self) -> Self {
        self.inner &= Self::NO_CAST_LOG.reverse_bits();
        *self
    }

    pub const fn is_CLASS_TRIGGER_ONLY_ON_TARGET(&self) -> bool {
        (self.inner & Self::CLASS_TRIGGER_ONLY_ON_TARGET) != 0
    }

    pub const fn new_CLASS_TRIGGER_ONLY_ON_TARGET() -> Self {
        Self { inner: Self::CLASS_TRIGGER_ONLY_ON_TARGET }
    }

    pub fn set_CLASS_TRIGGER_ONLY_ON_TARGET(&mut self) -> Self {
        self.inner |= Self::CLASS_TRIGGER_ONLY_ON_TARGET;
        *self
    }

    pub fn clear_CLASS_TRIGGER_ONLY_ON_TARGET(&mut self) -> Self {
        self.inner &= Self::CLASS_TRIGGER_ONLY_ON_TARGET.reverse_bits();
        *self
    }

    pub const fn is_AURA_EXPIRES_OFFLINE(&self) -> bool {
        (self.inner & Self::AURA_EXPIRES_OFFLINE) != 0
    }

    pub const fn new_AURA_EXPIRES_OFFLINE() -> Self {
        Self { inner: Self::AURA_EXPIRES_OFFLINE }
    }

    pub fn set_AURA_EXPIRES_OFFLINE(&mut self) -> Self {
        self.inner |= Self::AURA_EXPIRES_OFFLINE;
        *self
    }

    pub fn clear_AURA_EXPIRES_OFFLINE(&mut self) -> Self {
        self.inner &= Self::AURA_EXPIRES_OFFLINE.reverse_bits();
        *self
    }

    pub const fn is_NO_HELPFUL_THREAT(&self) -> bool {
        (self.inner & Self::NO_HELPFUL_THREAT) != 0
    }

    pub const fn new_NO_HELPFUL_THREAT() -> Self {
        Self { inner: Self::NO_HELPFUL_THREAT }
    }

    pub fn set_NO_HELPFUL_THREAT(&mut self) -> Self {
        self.inner |= Self::NO_HELPFUL_THREAT;
        *self
    }

    pub fn clear_NO_HELPFUL_THREAT(&mut self) -> Self {
        self.inner &= Self::NO_HELPFUL_THREAT.reverse_bits();
        *self
    }

    pub const fn is_NO_HARMFUL_THREAT(&self) -> bool {
        (self.inner & Self::NO_HARMFUL_THREAT) != 0
    }

    pub const fn new_NO_HARMFUL_THREAT() -> Self {
        Self { inner: Self::NO_HARMFUL_THREAT }
    }

    pub fn set_NO_HARMFUL_THREAT(&mut self) -> Self {
        self.inner |= Self::NO_HARMFUL_THREAT;
        *self
    }

    pub fn clear_NO_HARMFUL_THREAT(&mut self) -> Self {
        self.inner &= Self::NO_HARMFUL_THREAT.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_CLIENT_TARGETING(&self) -> bool {
        (self.inner & Self::ALLOW_CLIENT_TARGETING) != 0
    }

    pub const fn new_ALLOW_CLIENT_TARGETING() -> Self {
        Self { inner: Self::ALLOW_CLIENT_TARGETING }
    }

    pub fn set_ALLOW_CLIENT_TARGETING(&mut self) -> Self {
        self.inner |= Self::ALLOW_CLIENT_TARGETING;
        *self
    }

    pub fn clear_ALLOW_CLIENT_TARGETING(&mut self) -> Self {
        self.inner &= Self::ALLOW_CLIENT_TARGETING.reverse_bits();
        *self
    }

    pub const fn is_CANNOT_BE_STOLEN(&self) -> bool {
        (self.inner & Self::CANNOT_BE_STOLEN) != 0
    }

    pub const fn new_CANNOT_BE_STOLEN() -> Self {
        Self { inner: Self::CANNOT_BE_STOLEN }
    }

    pub fn set_CANNOT_BE_STOLEN(&mut self) -> Self {
        self.inner |= Self::CANNOT_BE_STOLEN;
        *self
    }

    pub fn clear_CANNOT_BE_STOLEN(&mut self) -> Self {
        self.inner &= Self::CANNOT_BE_STOLEN.reverse_bits();
        *self
    }

    pub const fn is_ALLOW_CAST_WHILE_CASTING(&self) -> bool {
        (self.inner & Self::ALLOW_CAST_WHILE_CASTING) != 0
    }

    pub const fn new_ALLOW_CAST_WHILE_CASTING() -> Self {
        Self { inner: Self::ALLOW_CAST_WHILE_CASTING }
    }

    pub fn set_ALLOW_CAST_WHILE_CASTING(&mut self) -> Self {
        self.inner |= Self::ALLOW_CAST_WHILE_CASTING;
        *self
    }

    pub fn clear_ALLOW_CAST_WHILE_CASTING(&mut self) -> Self {
        self.inner &= Self::ALLOW_CAST_WHILE_CASTING.reverse_bits();
        *self
    }

    pub const fn is_IGNORE_DAMAGE_TAKEN_MODIFIERS(&self) -> bool {
        (self.inner & Self::IGNORE_DAMAGE_TAKEN_MODIFIERS) != 0
    }

    pub const fn new_IGNORE_DAMAGE_TAKEN_MODIFIERS() -> Self {
        Self { inner: Self::IGNORE_DAMAGE_TAKEN_MODIFIERS }
    }

    pub fn set_IGNORE_DAMAGE_TAKEN_MODIFIERS(&mut self) -> Self {
        self.inner |= Self::IGNORE_DAMAGE_TAKEN_MODIFIERS;
        *self
    }

    pub fn clear_IGNORE_DAMAGE_TAKEN_MODIFIERS(&mut self) -> Self {
        self.inner &= Self::IGNORE_DAMAGE_TAKEN_MODIFIERS.reverse_bits();
        *self
    }

    pub const fn is_COMBAT_FEEDBACK_WHEN_USABLE(&self) -> bool {
        (self.inner & Self::COMBAT_FEEDBACK_WHEN_USABLE) != 0
    }

    pub const fn new_COMBAT_FEEDBACK_WHEN_USABLE() -> Self {
        Self { inner: Self::COMBAT_FEEDBACK_WHEN_USABLE }
    }

    pub fn set_COMBAT_FEEDBACK_WHEN_USABLE(&mut self) -> Self {
        self.inner |= Self::COMBAT_FEEDBACK_WHEN_USABLE;
        *self
    }

    pub fn clear_COMBAT_FEEDBACK_WHEN_USABLE(&mut self) -> Self {
        self.inner &= Self::COMBAT_FEEDBACK_WHEN_USABLE.reverse_bits();
        *self
    }

    pub const fn is_WEAPON_SPEED_COST_SCALING(&self) -> bool {
        (self.inner & Self::WEAPON_SPEED_COST_SCALING) != 0
    }

    pub const fn new_WEAPON_SPEED_COST_SCALING() -> Self {
        Self { inner: Self::WEAPON_SPEED_COST_SCALING }
    }

    pub fn set_WEAPON_SPEED_COST_SCALING(&mut self) -> Self {
        self.inner |= Self::WEAPON_SPEED_COST_SCALING;
        *self
    }

    pub fn clear_WEAPON_SPEED_COST_SCALING(&mut self) -> Self {
        self.inner &= Self::WEAPON_SPEED_COST_SCALING.reverse_bits();
        *self
    }

    pub const fn is_NO_PARTIAL_IMMUNITY(&self) -> bool {
        (self.inner & Self::NO_PARTIAL_IMMUNITY) != 0
    }

    pub const fn new_NO_PARTIAL_IMMUNITY() -> Self {
        Self { inner: Self::NO_PARTIAL_IMMUNITY }
    }

    pub fn set_NO_PARTIAL_IMMUNITY(&mut self) -> Self {
        self.inner |= Self::NO_PARTIAL_IMMUNITY;
        *self
    }

    pub fn clear_NO_PARTIAL_IMMUNITY(&mut self) -> Self {
        self.inner &= Self::NO_PARTIAL_IMMUNITY.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for AttributesEx4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for AttributesEx4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for AttributesEx4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for AttributesEx4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

