/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm#L23):
/// ```text
/// enum PetQueryDisabledNames : u8 {
///     PRESENT = 1;
///     NOT_PRESENT = 0;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PetQueryDisabledNames {
    Present,
    NotPresent,
}

impl PetQueryDisabledNames {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Present => 0x1,
            Self::NotPresent => 0x0,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::Present,
            Self::NotPresent,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl PetQueryDisabledNames {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Present => "PRESENT",
            Self::NotPresent => "NOT_PRESENT",
        }
    }

}

const NAME: &str = "PetQueryDisabledNames";

impl Default for PetQueryDisabledNames {
    fn default() -> Self {
        Self::Present
    }
}

impl std::fmt::Display for PetQueryDisabledNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Present => f.write_str("Present"),
            Self::NotPresent => f.write_str("NotPresent"),
        }
    }
}

impl TryFrom<u8> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Present),
            0 => Ok(Self::NotPresent),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for PetQueryDisabledNames {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

