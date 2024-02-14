/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:91`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L91):
/// ```text
/// enum ItemStatType : u8 {
///     MANA = 0;
///     HEALTH = 1;
///     AGILITY = 3;
///     STRENGTH = 4;
///     INTELLECT = 5;
///     SPIRIT = 6;
///     STAMINA = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemStatType {
    Mana,
    Health,
    Agility,
    Strength,
    Intellect,
    Spirit,
    Stamina,
}

impl ItemStatType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Mana => 0x0,
            Self::Health => 0x1,
            Self::Agility => 0x3,
            Self::Strength => 0x4,
            Self::Intellect => 0x5,
            Self::Spirit => 0x6,
            Self::Stamina => 0x7,
        }
    }

    pub const fn variants() -> [Self; 7] {
        [
            Self::Mana,
            Self::Health,
            Self::Agility,
            Self::Strength,
            Self::Intellect,
            Self::Spirit,
            Self::Stamina,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Mana),
            1 => Ok(Self::Health),
            3 => Ok(Self::Agility),
            4 => Ok(Self::Strength),
            5 => Ok(Self::Intellect),
            6 => Ok(Self::Spirit),
            7 => Ok(Self::Stamina),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ItemStatType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Mana => "MANA",
            Self::Health => "HEALTH",
            Self::Agility => "AGILITY",
            Self::Strength => "STRENGTH",
            Self::Intellect => "INTELLECT",
            Self::Spirit => "SPIRIT",
            Self::Stamina => "STAMINA",
        }
    }

}

const NAME: &str = "ItemStatType";

impl Default for ItemStatType {
    fn default() -> Self {
        Self::Mana
    }
}

impl std::fmt::Display for ItemStatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mana => f.write_str("Mana"),
            Self::Health => f.write_str("Health"),
            Self::Agility => f.write_str("Agility"),
            Self::Strength => f.write_str("Strength"),
            Self::Intellect => f.write_str("Intellect"),
            Self::Spirit => f.write_str("Spirit"),
            Self::Stamina => f.write_str("Stamina"),
        }
    }
}

impl TryFrom<u8> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

