/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_action_sound.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_action_sound.wowm#L3):
/// ```text
/// enum PetTalkReason : u32 {
///     SPECIAL_SPELL = 0;
///     ATTACK = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetTalkReason {
    SpecialSpell,
    Attack,
}

impl PetTalkReason {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::SpecialSpell => 0x0,
            Self::Attack => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl PetTalkReason {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::SpecialSpell => "SPECIAL_SPELL",
            Self::Attack => "ATTACK",
        }
    }

}

impl Default for PetTalkReason {
    fn default() -> Self {
        Self::SpecialSpell
    }
}

impl std::fmt::Display for PetTalkReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpecialSpell => f.write_str("SpecialSpell"),
            Self::Attack => f.write_str("Attack"),
        }
    }
}

impl TryFrom<u32> for PetTalkReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SpecialSpell),
            1 => Ok(Self::Attack),
            v => Err(crate::errors::EnumError::new("PetTalkReason", v as u64),)
        }
    }
}

