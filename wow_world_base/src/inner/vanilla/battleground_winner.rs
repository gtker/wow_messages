/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L8):
/// ```text
/// enum BattlegroundWinner : u8 {
///     HORDE = 0;
///     ALLIANCE = 1;
///     NONE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlegroundWinner {
    Horde,
    Alliance,
    None,
}

impl BattlegroundWinner {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Horde => 0x0,
            Self::Alliance => 0x1,
            Self::None => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Horde,
            Self::Alliance,
            Self::None,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Horde),
            1 => Ok(Self::Alliance),
            2 => Ok(Self::None),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BattlegroundWinner {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Horde => "HORDE",
            Self::Alliance => "ALLIANCE",
            Self::None => "NONE",
        }
    }

}

const NAME: &str = "BattlegroundWinner";

impl Default for BattlegroundWinner {
    fn default() -> Self {
        Self::Horde
    }
}

impl std::fmt::Display for BattlegroundWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horde => f.write_str("Horde"),
            Self::Alliance => f.write_str("Alliance"),
            Self::None => f.write_str("None"),
        }
    }
}

impl TryFrom<u8> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

