/// Used in `SheatheSoundLookups.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/item_env_types.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/item_env_types.wowm#L2):
/// ```text
/// enum ItemEnvTypes : u8 {
///     SHIELD = 0;
///     METAL_WEAPON = 1;
///     WOOD_WEAPON = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemEnvTypes {
    Shield,
    MetalWeapon,
    WoodWeapon,
}

impl ItemEnvTypes {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Shield => 0x0,
            Self::MetalWeapon => 0x1,
            Self::WoodWeapon => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Shield,
            Self::MetalWeapon,
            Self::WoodWeapon,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Shield),
            1 => Ok(Self::MetalWeapon),
            2 => Ok(Self::WoodWeapon),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ItemEnvTypes {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Shield => "SHIELD",
            Self::MetalWeapon => "METAL_WEAPON",
            Self::WoodWeapon => "WOOD_WEAPON",
        }
    }

}

const NAME: &str = "ItemEnvTypes";

impl Default for ItemEnvTypes {
    fn default() -> Self {
        Self::Shield
    }
}

impl std::fmt::Display for ItemEnvTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shield => f.write_str("Shield"),
            Self::MetalWeapon => f.write_str("MetalWeapon"),
            Self::WoodWeapon => f.write_str("WoodWeapon"),
        }
    }
}

impl TryFrom<u8> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ItemEnvTypes {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

