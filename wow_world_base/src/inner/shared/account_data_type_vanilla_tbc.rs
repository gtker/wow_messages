/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm#L1):
/// ```text
/// enum AccountDataType : u8 {
///     GLOBAL_CONFIG_CACHE = 0;
///     PER_CHARACTER_CONFIG_CACHE = 1;
///     GLOBAL_BINDINGS_CACHE = 2;
///     PER_CHARACTER_BINDINGS_CACHE = 3;
///     GLOBAL_MACROS_CACHE = 4;
///     PER_CHARACTER_MACROS_CACHE = 5;
///     PER_CHARACTER_LAYOUT_CACHE = 6;
///     PER_CHARACTER_CHAT_CACHE = 7;
///     NUM_ACCOUNT_DATA_TYPES = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AccountDataType {
    GlobalConfigCache,
    PerCharacterConfigCache,
    GlobalBindingsCache,
    PerCharacterBindingsCache,
    GlobalMacrosCache,
    PerCharacterMacrosCache,
    PerCharacterLayoutCache,
    PerCharacterChatCache,
    NumAccountDataTypes,
}

impl AccountDataType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::GlobalConfigCache => 0x0,
            Self::PerCharacterConfigCache => 0x1,
            Self::GlobalBindingsCache => 0x2,
            Self::PerCharacterBindingsCache => 0x3,
            Self::GlobalMacrosCache => 0x4,
            Self::PerCharacterMacrosCache => 0x5,
            Self::PerCharacterLayoutCache => 0x6,
            Self::PerCharacterChatCache => 0x7,
            Self::NumAccountDataTypes => 0x8,
        }
    }

    pub const fn variants() -> [Self; 9] {
        [
            Self::GlobalConfigCache,
            Self::PerCharacterConfigCache,
            Self::GlobalBindingsCache,
            Self::PerCharacterBindingsCache,
            Self::GlobalMacrosCache,
            Self::PerCharacterMacrosCache,
            Self::PerCharacterLayoutCache,
            Self::PerCharacterChatCache,
            Self::NumAccountDataTypes,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::GlobalConfigCache),
            1 => Ok(Self::PerCharacterConfigCache),
            2 => Ok(Self::GlobalBindingsCache),
            3 => Ok(Self::PerCharacterBindingsCache),
            4 => Ok(Self::GlobalMacrosCache),
            5 => Ok(Self::PerCharacterMacrosCache),
            6 => Ok(Self::PerCharacterLayoutCache),
            7 => Ok(Self::PerCharacterChatCache),
            8 => Ok(Self::NumAccountDataTypes),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl AccountDataType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::GlobalConfigCache => "GLOBAL_CONFIG_CACHE",
            Self::PerCharacterConfigCache => "PER_CHARACTER_CONFIG_CACHE",
            Self::GlobalBindingsCache => "GLOBAL_BINDINGS_CACHE",
            Self::PerCharacterBindingsCache => "PER_CHARACTER_BINDINGS_CACHE",
            Self::GlobalMacrosCache => "GLOBAL_MACROS_CACHE",
            Self::PerCharacterMacrosCache => "PER_CHARACTER_MACROS_CACHE",
            Self::PerCharacterLayoutCache => "PER_CHARACTER_LAYOUT_CACHE",
            Self::PerCharacterChatCache => "PER_CHARACTER_CHAT_CACHE",
            Self::NumAccountDataTypes => "NUM_ACCOUNT_DATA_TYPES",
        }
    }

}

const NAME: &str = "AccountDataType";

impl Default for AccountDataType {
    fn default() -> Self {
        Self::GlobalConfigCache
    }
}

impl std::fmt::Display for AccountDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlobalConfigCache => f.write_str("GlobalConfigCache"),
            Self::PerCharacterConfigCache => f.write_str("PerCharacterConfigCache"),
            Self::GlobalBindingsCache => f.write_str("GlobalBindingsCache"),
            Self::PerCharacterBindingsCache => f.write_str("PerCharacterBindingsCache"),
            Self::GlobalMacrosCache => f.write_str("GlobalMacrosCache"),
            Self::PerCharacterMacrosCache => f.write_str("PerCharacterMacrosCache"),
            Self::PerCharacterLayoutCache => f.write_str("PerCharacterLayoutCache"),
            Self::PerCharacterChatCache => f.write_str("PerCharacterChatCache"),
            Self::NumAccountDataTypes => f.write_str("NumAccountDataTypes"),
        }
    }
}

impl TryFrom<u8> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for AccountDataType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

