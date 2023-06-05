/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L1):
/// ```text
/// enum HitInfo : u32 {
///     NORMAL_SWING = 0x00000000;
///     UNK1 = 0x00000001;
///     AFFECTS_VICTIM = 0x00000002;
///     LEFT_SWING = 0x00000004;
///     EARLY_CRITICAL_HIT = 0x00000008;
///     MISS = 0x00000010;
///     ABSORB = 0x00000020;
///     RESIST = 0x00000040;
///     CRITICAL_HIT = 0x00000080;
///     UNK9 = 0x00000100;
///     UNK10 = 0x00002000;
///     GLANCING = 0x00004000;
///     CRUSHING = 0x00008000;
///     NO_ACTION = 0x00010000;
///     SWING_NO_HIT_SOUND = 0x00080000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum HitInfo {
    NormalSwing,
    /// req correct packet structure
    ///
    Unk1,
    /// no being hit animation on victim without it
    ///
    AffectsVictim,
    LeftSwing,
    /// According to vmangos used as crit prior to 1.9.
    ///
    EarlyCriticalHit,
    Miss,
    /// plays absorb sound
    ///
    Absorb,
    /// resisted atleast some damage
    ///
    Resist,
    CriticalHit,
    /// wotlk?
    ///
    Unk9,
    /// wotlk?
    ///
    Unk10,
    Glancing,
    Crushing,
    NoAction,
    SwingNoHitSound,
}

impl HitInfo {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::NormalSwing => 0x0,
            Self::Unk1 => 0x1,
            Self::AffectsVictim => 0x2,
            Self::LeftSwing => 0x4,
            Self::EarlyCriticalHit => 0x8,
            Self::Miss => 0x10,
            Self::Absorb => 0x20,
            Self::Resist => 0x40,
            Self::CriticalHit => 0x80,
            Self::Unk9 => 0x100,
            Self::Unk10 => 0x2000,
            Self::Glancing => 0x4000,
            Self::Crushing => 0x8000,
            Self::NoAction => 0x10000,
            Self::SwingNoHitSound => 0x80000,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl HitInfo {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NormalSwing => "NORMAL_SWING",
            Self::Unk1 => "UNK1",
            Self::AffectsVictim => "AFFECTS_VICTIM",
            Self::LeftSwing => "LEFT_SWING",
            Self::EarlyCriticalHit => "EARLY_CRITICAL_HIT",
            Self::Miss => "MISS",
            Self::Absorb => "ABSORB",
            Self::Resist => "RESIST",
            Self::CriticalHit => "CRITICAL_HIT",
            Self::Unk9 => "UNK9",
            Self::Unk10 => "UNK10",
            Self::Glancing => "GLANCING",
            Self::Crushing => "CRUSHING",
            Self::NoAction => "NO_ACTION",
            Self::SwingNoHitSound => "SWING_NO_HIT_SOUND",
        }
    }

}

impl Default for HitInfo {
    fn default() -> Self {
        Self::NormalSwing
    }
}

impl std::fmt::Display for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NormalSwing => f.write_str("NormalSwing"),
            Self::Unk1 => f.write_str("Unk1"),
            Self::AffectsVictim => f.write_str("AffectsVictim"),
            Self::LeftSwing => f.write_str("LeftSwing"),
            Self::EarlyCriticalHit => f.write_str("EarlyCriticalHit"),
            Self::Miss => f.write_str("Miss"),
            Self::Absorb => f.write_str("Absorb"),
            Self::Resist => f.write_str("Resist"),
            Self::CriticalHit => f.write_str("CriticalHit"),
            Self::Unk9 => f.write_str("Unk9"),
            Self::Unk10 => f.write_str("Unk10"),
            Self::Glancing => f.write_str("Glancing"),
            Self::Crushing => f.write_str("Crushing"),
            Self::NoAction => f.write_str("NoAction"),
            Self::SwingNoHitSound => f.write_str("SwingNoHitSound"),
        }
    }
}

impl TryFrom<u32> for HitInfo {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NormalSwing),
            1 => Ok(Self::Unk1),
            2 => Ok(Self::AffectsVictim),
            4 => Ok(Self::LeftSwing),
            8 => Ok(Self::EarlyCriticalHit),
            16 => Ok(Self::Miss),
            32 => Ok(Self::Absorb),
            64 => Ok(Self::Resist),
            128 => Ok(Self::CriticalHit),
            256 => Ok(Self::Unk9),
            8192 => Ok(Self::Unk10),
            16384 => Ok(Self::Glancing),
            32768 => Ok(Self::Crushing),
            65536 => Ok(Self::NoAction),
            524288 => Ok(Self::SwingNoHitSound),
            v => Err(crate::errors::EnumError::new("HitInfo", v as u64),)
        }
    }
}

