/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/bonding.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/bonding.wowm#L1):
/// ```text
/// enum Bonding : u8 {
///     NO_BIND = 0;
///     PICK_UP = 1;
///     EQUIP = 2;
///     USE = 3;
///     QUEST_ITEM = 4;
///     QUEST_ITEM1 = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Bonding {
    NoBind,
    PickUp,
    Equip,
    Use,
    QuestItem,
    /// Not used in game according to all emulators.
    QuestItem1,
}

impl Bonding {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NoBind => 0x0,
            Self::PickUp => 0x1,
            Self::Equip => 0x2,
            Self::Use => 0x3,
            Self::QuestItem => 0x4,
            Self::QuestItem1 => 0x5,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::NoBind,
            Self::PickUp,
            Self::Equip,
            Self::Use,
            Self::QuestItem,
            Self::QuestItem1,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::NoBind),
            1 => Ok(Self::PickUp),
            2 => Ok(Self::Equip),
            3 => Ok(Self::Use),
            4 => Ok(Self::QuestItem),
            5 => Ok(Self::QuestItem1),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl Bonding {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NoBind => "NO_BIND",
            Self::PickUp => "PICK_UP",
            Self::Equip => "EQUIP",
            Self::Use => "USE",
            Self::QuestItem => "QUEST_ITEM",
            Self::QuestItem1 => "QUEST_ITEM1",
        }
    }

}

const NAME: &str = "Bonding";

impl Default for Bonding {
    fn default() -> Self {
        Self::NoBind
    }
}

impl std::fmt::Display for Bonding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoBind => f.write_str("NoBind"),
            Self::PickUp => f.write_str("Binds when picked up"),
            Self::Equip => f.write_str("Binds when equipped"),
            Self::Use => f.write_str("Binds on use"),
            Self::QuestItem => f.write_str("Quest Item"),
            Self::QuestItem1 => f.write_str("Quest Item"),
        }
    }
}

impl TryFrom<u8> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for Bonding {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

