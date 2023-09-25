/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L1):
/// ```text
/// enum SheatheType : u8 {
///     NONE = 0;
///     MAIN_HAND = 1;
///     OFF_HAND = 2;
///     LARGE_WEAPON_LEFT = 3;
///     LARGE_WEAPON_RIGHT = 4;
///     HIP_WEAPON_LEFT = 5;
///     HIP_WEAPON_RIGHT = 6;
///     SHIELD = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SheatheType {
    None,
    MainHand,
    OffHand,
    LargeWeaponLeft,
    LargeWeaponRight,
    HipWeaponLeft,
    HipWeaponRight,
    Shield,
}

impl SheatheType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::MainHand => 0x1,
            Self::OffHand => 0x2,
            Self::LargeWeaponLeft => 0x3,
            Self::LargeWeaponRight => 0x4,
            Self::HipWeaponLeft => 0x5,
            Self::HipWeaponRight => 0x6,
            Self::Shield => 0x7,
        }
    }

    pub const fn variants() -> [Self; 8] {
        [
            Self::None,
            Self::MainHand,
            Self::OffHand,
            Self::LargeWeaponLeft,
            Self::LargeWeaponRight,
            Self::HipWeaponLeft,
            Self::HipWeaponRight,
            Self::Shield,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::MainHand),
            2 => Ok(Self::OffHand),
            3 => Ok(Self::LargeWeaponLeft),
            4 => Ok(Self::LargeWeaponRight),
            5 => Ok(Self::HipWeaponLeft),
            6 => Ok(Self::HipWeaponRight),
            7 => Ok(Self::Shield),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SheatheType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::MainHand => "MAIN_HAND",
            Self::OffHand => "OFF_HAND",
            Self::LargeWeaponLeft => "LARGE_WEAPON_LEFT",
            Self::LargeWeaponRight => "LARGE_WEAPON_RIGHT",
            Self::HipWeaponLeft => "HIP_WEAPON_LEFT",
            Self::HipWeaponRight => "HIP_WEAPON_RIGHT",
            Self::Shield => "SHIELD",
        }
    }

}

const NAME: &str = "SheatheType";

impl Default for SheatheType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for SheatheType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::MainHand => f.write_str("MainHand"),
            Self::OffHand => f.write_str("OffHand"),
            Self::LargeWeaponLeft => f.write_str("LargeWeaponLeft"),
            Self::LargeWeaponRight => f.write_str("LargeWeaponRight"),
            Self::HipWeaponLeft => f.write_str("HipWeaponLeft"),
            Self::HipWeaponRight => f.write_str("HipWeaponRight"),
            Self::Shield => f.write_str("Shield"),
        }
    }
}

impl TryFrom<u8> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SheatheType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

