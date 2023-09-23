/// Used in `Emotes.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/emote_spec_proc.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/emote_spec_proc.wowm#L2):
/// ```text
/// enum EmoteSpecProc : u8 {
///     NO_LOOP = 0;
///     LOOP = 1;
///     LOOP_WITH_SOUND = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum EmoteSpecProc {
    NoLoop,
    Loop,
    LoopWithSound,
}

impl EmoteSpecProc {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NoLoop => 0x0,
            Self::Loop => 0x1,
            Self::LoopWithSound => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::NoLoop,
            Self::Loop,
            Self::LoopWithSound,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl EmoteSpecProc {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NoLoop => "NO_LOOP",
            Self::Loop => "LOOP",
            Self::LoopWithSound => "LOOP_WITH_SOUND",
        }
    }

}

const NAME: &str = "EmoteSpecProc";

impl Default for EmoteSpecProc {
    fn default() -> Self {
        Self::NoLoop
    }
}

impl std::fmt::Display for EmoteSpecProc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoLoop => f.write_str("NoLoop"),
            Self::Loop => f.write_str("Loop"),
            Self::LoopWithSound => f.write_str("LoopWithSound"),
        }
    }
}

impl TryFrom<u8> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoLoop),
            1 => Ok(Self::Loop),
            2 => Ok(Self::LoopWithSound),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for EmoteSpecProc {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

