/// According to cmangos: byte value (`UNIT_FIELD_BYTES_2`,0)
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm#L1):
/// ```text
/// enum SheathState : u8 {
///     UNARMED = 0;
///     MELEE = 1;
///     RANGED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SheathState {
    Unarmed,
    Melee,
    Ranged,
}

impl SheathState {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Unarmed => 0x0,
            Self::Melee => 0x1,
            Self::Ranged => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Unarmed,
            Self::Melee,
            Self::Ranged,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl SheathState {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Unarmed => "UNARMED",
            Self::Melee => "MELEE",
            Self::Ranged => "RANGED",
        }
    }

}

const NAME: &str = "SheathState";

impl Default for SheathState {
    fn default() -> Self {
        Self::Unarmed
    }
}

impl std::fmt::Display for SheathState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unarmed => f.write_str("Unarmed"),
            Self::Melee => f.write_str("Melee"),
            Self::Ranged => f.write_str("Ranged"),
        }
    }
}

impl TryFrom<u8> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unarmed),
            1 => Ok(Self::Melee),
            2 => Ok(Self::Ranged),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SheathState {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

