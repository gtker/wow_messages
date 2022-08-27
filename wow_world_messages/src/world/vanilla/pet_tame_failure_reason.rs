use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L3):
/// ```text
/// enum PetTameFailureReason : u8 {
///     INVALIDCREATURE = 1;
///     TOOMANY = 2;
///     CREATUREALREADYOWNED = 3;
///     NOTTAMEABLE = 4;
///     ANOTHERSUMMONACTIVE = 5;
///     UNITSCANTTAME = 6;
///     NOPETAVAILABLE = 7;
///     INTERNALERROR = 8;
///     TOOHIGHLEVEL = 9;
///     DEAD = 10;
///     NOTDEAD = 11;
///     UNKNOWNERROR = 12;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTameFailureReason {
    Invalidcreature,
    Toomany,
    Creaturealreadyowned,
    Nottameable,
    Anothersummonactive,
    Unitscanttame,
    /// not used in taming
    ///
    Nopetavailable,
    Internalerror,
    Toohighlevel,
    /// not used in taming
    ///
    Dead,
    /// not used in taming
    ///
    Notdead,
    Unknownerror,
}

impl PetTameFailureReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Invalidcreature => 0x1,
            Self::Toomany => 0x2,
            Self::Creaturealreadyowned => 0x3,
            Self::Nottameable => 0x4,
            Self::Anothersummonactive => 0x5,
            Self::Unitscanttame => 0x6,
            Self::Nopetavailable => 0x7,
            Self::Internalerror => 0x8,
            Self::Toohighlevel => 0x9,
            Self::Dead => 0xa,
            Self::Notdead => 0xb,
            Self::Unknownerror => 0xc,
        }
    }

}

impl Default for PetTameFailureReason {
    fn default() -> Self {
        Self::Invalidcreature
    }
}

impl std::fmt::Display for PetTameFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalidcreature => f.write_str("Invalidcreature"),
            Self::Toomany => f.write_str("Toomany"),
            Self::Creaturealreadyowned => f.write_str("Creaturealreadyowned"),
            Self::Nottameable => f.write_str("Nottameable"),
            Self::Anothersummonactive => f.write_str("Anothersummonactive"),
            Self::Unitscanttame => f.write_str("Unitscanttame"),
            Self::Nopetavailable => f.write_str("Nopetavailable"),
            Self::Internalerror => f.write_str("Internalerror"),
            Self::Toohighlevel => f.write_str("Toohighlevel"),
            Self::Dead => f.write_str("Dead"),
            Self::Notdead => f.write_str("Notdead"),
            Self::Unknownerror => f.write_str("Unknownerror"),
        }
    }
}

impl TryFrom<u8> for PetTameFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Invalidcreature),
            2 => Ok(Self::Toomany),
            3 => Ok(Self::Creaturealreadyowned),
            4 => Ok(Self::Nottameable),
            5 => Ok(Self::Anothersummonactive),
            6 => Ok(Self::Unitscanttame),
            7 => Ok(Self::Nopetavailable),
            8 => Ok(Self::Internalerror),
            9 => Ok(Self::Toohighlevel),
            10 => Ok(Self::Dead),
            11 => Ok(Self::Notdead),
            12 => Ok(Self::Unknownerror),
            v => Err(crate::errors::EnumError::new("PetTameFailureReason", v as u32),)
        }
    }
}

