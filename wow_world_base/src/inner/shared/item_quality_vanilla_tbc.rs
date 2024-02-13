/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L11):
/// ```text
/// enum ItemQuality : u8 {
///     POOR = 0;
///     NORMAL = 1;
///     UNCOMMON = 2;
///     RARE = 3;
///     EPIC = 4;
///     LEGENDARY = 5;
///     ARTIFACT = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemQuality {
    Poor,
    Normal,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
}

impl ItemQuality {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Poor => 0x0,
            Self::Normal => 0x1,
            Self::Uncommon => 0x2,
            Self::Rare => 0x3,
            Self::Epic => 0x4,
            Self::Legendary => 0x5,
            Self::Artifact => 0x6,
        }
    }

    pub const fn variants() -> [Self; 7] {
        [
            Self::Poor,
            Self::Normal,
            Self::Uncommon,
            Self::Rare,
            Self::Epic,
            Self::Legendary,
            Self::Artifact,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Poor),
            1 => Ok(Self::Normal),
            2 => Ok(Self::Uncommon),
            3 => Ok(Self::Rare),
            4 => Ok(Self::Epic),
            5 => Ok(Self::Legendary),
            6 => Ok(Self::Artifact),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ItemQuality {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Poor => "POOR",
            Self::Normal => "NORMAL",
            Self::Uncommon => "UNCOMMON",
            Self::Rare => "RARE",
            Self::Epic => "EPIC",
            Self::Legendary => "LEGENDARY",
            Self::Artifact => "ARTIFACT",
        }
    }

}

const NAME: &str = "ItemQuality";

impl Default for ItemQuality {
    fn default() -> Self {
        Self::Poor
    }
}

impl std::fmt::Display for ItemQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Poor => f.write_str("Poor"),
            Self::Normal => f.write_str("Normal"),
            Self::Uncommon => f.write_str("Uncommon"),
            Self::Rare => f.write_str("Rare"),
            Self::Epic => f.write_str("Epic"),
            Self::Legendary => f.write_str("Legendary"),
            Self::Artifact => f.write_str("Artifact"),
        }
    }
}

impl TryFrom<u8> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

