/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_push_result.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_push_result.wowm#L8):
/// ```text
/// enum NewItemCreationType : u32 {
///     RECEIVED = 0;
///     CREATED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum NewItemCreationType {
    Received,
    Created,
}

impl NewItemCreationType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Received => 0x0,
            Self::Created => 0x1,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::Received,
            Self::Created,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Received),
            1 => Ok(Self::Created),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl NewItemCreationType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Received => "RECEIVED",
            Self::Created => "CREATED",
        }
    }

}

const NAME: &str = "NewItemCreationType";

impl Default for NewItemCreationType {
    fn default() -> Self {
        Self::Received
    }
}

impl std::fmt::Display for NewItemCreationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Received => f.write_str("Received"),
            Self::Created => f.write_str("Created"),
        }
    }
}

impl TryFrom<u32> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for NewItemCreationType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

