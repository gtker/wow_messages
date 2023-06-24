/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm#L3):
/// ```text
/// enum DismountResult : u32 {
///     NOT_MOUNTED = 1;
///     OK = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DismountResult {
    NotMounted,
    Ok,
}

impl DismountResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::NotMounted => 0x1,
            Self::Ok => 0x3,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::NotMounted,
            Self::Ok,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl DismountResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotMounted => "NOT_MOUNTED",
            Self::Ok => "OK",
        }
    }

}

const NAME: &str = "DismountResult";

impl Default for DismountResult {
    fn default() -> Self {
        Self::NotMounted
    }
}

impl std::fmt::Display for DismountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotMounted => f.write_str("NotMounted"),
            Self::Ok => f.write_str("Ok"),
        }
    }
}

impl TryFrom<u32> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::NotMounted),
            3 => Ok(Self::Ok),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

