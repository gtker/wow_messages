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

#[cfg(feature = "print-testcase")]
impl AttributesEx4 {
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
        if self.is_no_cast_log() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_CAST_LOG").unwrap();
            first = false;
        }
        if self.is_class_trigger_only_on_target() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CLASS_TRIGGER_ONLY_ON_TARGET").unwrap();
            first = false;
        }
        if self.is_aura_expires_offline() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AURA_EXPIRES_OFFLINE").unwrap();
            first = false;
        }
        if self.is_no_helpful_threat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_HELPFUL_THREAT").unwrap();
            first = false;
        }
        if self.is_no_harmful_threat() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_HARMFUL_THREAT").unwrap();
            first = false;
        }
        if self.is_allow_client_targeting() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_CLIENT_TARGETING").unwrap();
            first = false;
        }
        if self.is_cannot_be_stolen() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CANNOT_BE_STOLEN").unwrap();
            first = false;
        }
        if self.is_allow_cast_while_casting() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALLOW_CAST_WHILE_CASTING").unwrap();
            first = false;
        }
        if self.is_ignore_damage_taken_modifiers() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "IGNORE_DAMAGE_TAKEN_MODIFIERS").unwrap();
            first = false;
        }
        if self.is_combat_feedback_when_usable() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "COMBAT_FEEDBACK_WHEN_USABLE").unwrap();
            first = false;
        }
        if self.is_weapon_speed_cost_scaling() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "WEAPON_SPEED_COST_SCALING").unwrap();
            first = false;
        }
        if self.is_no_partial_immunity() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NO_PARTIAL_IMMUNITY").unwrap();
            first = false;
        }
        s
    }

}

impl AttributesEx4 {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const NO_CAST_LOG: u32 = 0x01;
    pub const CLASS_TRIGGER_ONLY_ON_TARGET: u32 = 0x02;
    pub const AURA_EXPIRES_OFFLINE: u32 = 0x04;
    pub const NO_HELPFUL_THREAT: u32 = 0x08;
    pub const NO_HARMFUL_THREAT: u32 = 0x10;
    pub const ALLOW_CLIENT_TARGETING: u32 = 0x20;
    pub const CANNOT_BE_STOLEN: u32 = 0x40;
    pub const ALLOW_CAST_WHILE_CASTING: u32 = 0x80;
    pub const IGNORE_DAMAGE_TAKEN_MODIFIERS: u32 = 0x100;
    pub const COMBAT_FEEDBACK_WHEN_USABLE: u32 = 0x200;
    pub const WEAPON_SPEED_COST_SCALING: u32 = 0x400;
    pub const NO_PARTIAL_IMMUNITY: u32 = 0x800;

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

    pub const fn is_no_cast_log(&self) -> bool {
        (self.inner & Self::NO_CAST_LOG) != 0
    }

    pub const fn new_no_cast_log() -> Self {
        Self { inner: Self::NO_CAST_LOG }
    }

    pub fn set_no_cast_log(&mut self) -> Self {
        self.inner |= Self::NO_CAST_LOG;
        *self
    }

    pub fn clear_no_cast_log(&mut self) -> Self {
        self.inner &= Self::NO_CAST_LOG.reverse_bits();
        *self
    }

    pub const fn is_class_trigger_only_on_target(&self) -> bool {
        (self.inner & Self::CLASS_TRIGGER_ONLY_ON_TARGET) != 0
    }

    pub const fn new_class_trigger_only_on_target() -> Self {
        Self { inner: Self::CLASS_TRIGGER_ONLY_ON_TARGET }
    }

    pub fn set_class_trigger_only_on_target(&mut self) -> Self {
        self.inner |= Self::CLASS_TRIGGER_ONLY_ON_TARGET;
        *self
    }

    pub fn clear_class_trigger_only_on_target(&mut self) -> Self {
        self.inner &= Self::CLASS_TRIGGER_ONLY_ON_TARGET.reverse_bits();
        *self
    }

    pub const fn is_aura_expires_offline(&self) -> bool {
        (self.inner & Self::AURA_EXPIRES_OFFLINE) != 0
    }

    pub const fn new_aura_expires_offline() -> Self {
        Self { inner: Self::AURA_EXPIRES_OFFLINE }
    }

    pub fn set_aura_expires_offline(&mut self) -> Self {
        self.inner |= Self::AURA_EXPIRES_OFFLINE;
        *self
    }

    pub fn clear_aura_expires_offline(&mut self) -> Self {
        self.inner &= Self::AURA_EXPIRES_OFFLINE.reverse_bits();
        *self
    }

    pub const fn is_no_helpful_threat(&self) -> bool {
        (self.inner & Self::NO_HELPFUL_THREAT) != 0
    }

    pub const fn new_no_helpful_threat() -> Self {
        Self { inner: Self::NO_HELPFUL_THREAT }
    }

    pub fn set_no_helpful_threat(&mut self) -> Self {
        self.inner |= Self::NO_HELPFUL_THREAT;
        *self
    }

    pub fn clear_no_helpful_threat(&mut self) -> Self {
        self.inner &= Self::NO_HELPFUL_THREAT.reverse_bits();
        *self
    }

    pub const fn is_no_harmful_threat(&self) -> bool {
        (self.inner & Self::NO_HARMFUL_THREAT) != 0
    }

    pub const fn new_no_harmful_threat() -> Self {
        Self { inner: Self::NO_HARMFUL_THREAT }
    }

    pub fn set_no_harmful_threat(&mut self) -> Self {
        self.inner |= Self::NO_HARMFUL_THREAT;
        *self
    }

    pub fn clear_no_harmful_threat(&mut self) -> Self {
        self.inner &= Self::NO_HARMFUL_THREAT.reverse_bits();
        *self
    }

    pub const fn is_allow_client_targeting(&self) -> bool {
        (self.inner & Self::ALLOW_CLIENT_TARGETING) != 0
    }

    pub const fn new_allow_client_targeting() -> Self {
        Self { inner: Self::ALLOW_CLIENT_TARGETING }
    }

    pub fn set_allow_client_targeting(&mut self) -> Self {
        self.inner |= Self::ALLOW_CLIENT_TARGETING;
        *self
    }

    pub fn clear_allow_client_targeting(&mut self) -> Self {
        self.inner &= Self::ALLOW_CLIENT_TARGETING.reverse_bits();
        *self
    }

    pub const fn is_cannot_be_stolen(&self) -> bool {
        (self.inner & Self::CANNOT_BE_STOLEN) != 0
    }

    pub const fn new_cannot_be_stolen() -> Self {
        Self { inner: Self::CANNOT_BE_STOLEN }
    }

    pub fn set_cannot_be_stolen(&mut self) -> Self {
        self.inner |= Self::CANNOT_BE_STOLEN;
        *self
    }

    pub fn clear_cannot_be_stolen(&mut self) -> Self {
        self.inner &= Self::CANNOT_BE_STOLEN.reverse_bits();
        *self
    }

    pub const fn is_allow_cast_while_casting(&self) -> bool {
        (self.inner & Self::ALLOW_CAST_WHILE_CASTING) != 0
    }

    pub const fn new_allow_cast_while_casting() -> Self {
        Self { inner: Self::ALLOW_CAST_WHILE_CASTING }
    }

    pub fn set_allow_cast_while_casting(&mut self) -> Self {
        self.inner |= Self::ALLOW_CAST_WHILE_CASTING;
        *self
    }

    pub fn clear_allow_cast_while_casting(&mut self) -> Self {
        self.inner &= Self::ALLOW_CAST_WHILE_CASTING.reverse_bits();
        *self
    }

    pub const fn is_ignore_damage_taken_modifiers(&self) -> bool {
        (self.inner & Self::IGNORE_DAMAGE_TAKEN_MODIFIERS) != 0
    }

    pub const fn new_ignore_damage_taken_modifiers() -> Self {
        Self { inner: Self::IGNORE_DAMAGE_TAKEN_MODIFIERS }
    }

    pub fn set_ignore_damage_taken_modifiers(&mut self) -> Self {
        self.inner |= Self::IGNORE_DAMAGE_TAKEN_MODIFIERS;
        *self
    }

    pub fn clear_ignore_damage_taken_modifiers(&mut self) -> Self {
        self.inner &= Self::IGNORE_DAMAGE_TAKEN_MODIFIERS.reverse_bits();
        *self
    }

    pub const fn is_combat_feedback_when_usable(&self) -> bool {
        (self.inner & Self::COMBAT_FEEDBACK_WHEN_USABLE) != 0
    }

    pub const fn new_combat_feedback_when_usable() -> Self {
        Self { inner: Self::COMBAT_FEEDBACK_WHEN_USABLE }
    }

    pub fn set_combat_feedback_when_usable(&mut self) -> Self {
        self.inner |= Self::COMBAT_FEEDBACK_WHEN_USABLE;
        *self
    }

    pub fn clear_combat_feedback_when_usable(&mut self) -> Self {
        self.inner &= Self::COMBAT_FEEDBACK_WHEN_USABLE.reverse_bits();
        *self
    }

    pub const fn is_weapon_speed_cost_scaling(&self) -> bool {
        (self.inner & Self::WEAPON_SPEED_COST_SCALING) != 0
    }

    pub const fn new_weapon_speed_cost_scaling() -> Self {
        Self { inner: Self::WEAPON_SPEED_COST_SCALING }
    }

    pub fn set_weapon_speed_cost_scaling(&mut self) -> Self {
        self.inner |= Self::WEAPON_SPEED_COST_SCALING;
        *self
    }

    pub fn clear_weapon_speed_cost_scaling(&mut self) -> Self {
        self.inner &= Self::WEAPON_SPEED_COST_SCALING.reverse_bits();
        *self
    }

    pub const fn is_no_partial_immunity(&self) -> bool {
        (self.inner & Self::NO_PARTIAL_IMMUNITY) != 0
    }

    pub const fn new_no_partial_immunity() -> Self {
        Self { inner: Self::NO_PARTIAL_IMMUNITY }
    }

    pub fn set_no_partial_immunity(&mut self) -> Self {
        self.inner |= Self::NO_PARTIAL_IMMUNITY;
        *self
    }

    pub fn clear_no_partial_immunity(&mut self) -> Self {
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

impl std::ops::BitAnd for AttributesEx4 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for AttributesEx4 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for AttributesEx4 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for AttributesEx4 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for AttributesEx4 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for AttributesEx4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

