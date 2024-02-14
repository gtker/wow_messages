/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm#L3):
/// ```text
/// enum EnvironmentalDamageType : u8 {
///     EXHAUSTED = 0;
///     DROWNING = 1;
///     FALL = 2;
///     LAVA = 3;
///     SLIME = 4;
///     FIRE = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum EnvironmentalDamageType {
    Exhausted,
    Drowning,
    Fall,
    Lava,
    Slime,
    Fire,
}

impl EnvironmentalDamageType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Exhausted => 0x0,
            Self::Drowning => 0x1,
            Self::Fall => 0x2,
            Self::Lava => 0x3,
            Self::Slime => 0x4,
            Self::Fire => 0x5,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::Exhausted,
            Self::Drowning,
            Self::Fall,
            Self::Lava,
            Self::Slime,
            Self::Fire,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Exhausted),
            1 => Ok(Self::Drowning),
            2 => Ok(Self::Fall),
            3 => Ok(Self::Lava),
            4 => Ok(Self::Slime),
            5 => Ok(Self::Fire),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl EnvironmentalDamageType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Exhausted => "EXHAUSTED",
            Self::Drowning => "DROWNING",
            Self::Fall => "FALL",
            Self::Lava => "LAVA",
            Self::Slime => "SLIME",
            Self::Fire => "FIRE",
        }
    }

}

const NAME: &str = "EnvironmentalDamageType";

impl Default for EnvironmentalDamageType {
    fn default() -> Self {
        Self::Exhausted
    }
}

impl std::fmt::Display for EnvironmentalDamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exhausted => f.write_str("Exhausted"),
            Self::Drowning => f.write_str("Drowning"),
            Self::Fall => f.write_str("Fall"),
            Self::Lava => f.write_str("Lava"),
            Self::Slime => f.write_str("Slime"),
            Self::Fire => f.write_str("Fire"),
        }
    }
}

impl TryFrom<u8> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

