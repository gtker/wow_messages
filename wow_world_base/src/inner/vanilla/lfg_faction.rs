/// Used in `LFGDungeons.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/lfg_faction.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/lfg_faction.wowm#L2):
/// ```text
/// enum LfgFaction : i8 {
///     NEUTRAL = -1;
///     HORDE = 0;
///     ALLIANCE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LfgFaction {
    Neutral,
    Horde,
    Alliance,
}

impl LfgFaction {
    pub const fn as_int(&self) -> i8 {
        match self {
            Self::Neutral => -1,
            Self::Horde => 0x0,
            Self::Alliance => 0x1,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Neutral,
            Self::Horde,
            Self::Alliance,
        ]
    }

    pub const fn from_int(value: i8) -> Result<Self, crate::errors::EnumError> {
        match value {
            -1 => Ok(Self::Neutral),
            0 => Ok(Self::Horde),
            1 => Ok(Self::Alliance),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl LfgFaction {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Neutral => "NEUTRAL",
            Self::Horde => "HORDE",
            Self::Alliance => "ALLIANCE",
        }
    }

}

const NAME: &str = "LfgFaction";

impl Default for LfgFaction {
    fn default() -> Self {
        Self::Neutral
    }
}

impl std::fmt::Display for LfgFaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neutral => f.write_str("Neutral"),
            Self::Horde => f.write_str("Horde"),
            Self::Alliance => f.write_str("Alliance"),
        }
    }
}

impl TryFrom<i8> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = i8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<u16> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LfgFaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

