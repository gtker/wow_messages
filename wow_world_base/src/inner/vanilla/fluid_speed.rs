/// Used in `SoundWaterType.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/fluid_speed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/fluid_speed.wowm#L1):
/// ```text
/// enum FluidSpeed : u8 {
///     STILL = 0;
///     SLOW = 4;
///     RAPID = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FluidSpeed {
    Still,
    Slow,
    Rapid,
}

impl FluidSpeed {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Still => 0x0,
            Self::Slow => 0x4,
            Self::Rapid => 0x8,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Still,
            Self::Slow,
            Self::Rapid,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl FluidSpeed {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Still => "STILL",
            Self::Slow => "SLOW",
            Self::Rapid => "RAPID",
        }
    }

}

const NAME: &str = "FluidSpeed";

impl Default for FluidSpeed {
    fn default() -> Self {
        Self::Still
    }
}

impl std::fmt::Display for FluidSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Still => f.write_str("Still"),
            Self::Slow => f.write_str("Slow"),
            Self::Rapid => f.write_str("Rapid"),
        }
    }
}

impl TryFrom<u8> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Still),
            4 => Ok(Self::Slow),
            8 => Ok(Self::Rapid),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for FluidSpeed {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

