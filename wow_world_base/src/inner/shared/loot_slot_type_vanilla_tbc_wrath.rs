/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L50):
/// ```text
/// enum LootSlotType : u8 {
///     TYPE_ALLOW_LOOT = 0;
///     TYPE_ROLL_ONGOING = 1;
///     TYPE_MASTER = 2;
///     TYPE_LOCKED = 3;
///     TYPE_OWNER = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LootSlotType {
    /// player can loot the item.
    TypeAllowLoot,
    /// roll is ongoing. player cannot loot.
    TypeRollOngoing,
    /// item can only be distributed by group loot master.
    TypeMaster,
    /// item is shown in red. player cannot loot.
    TypeLocked,
    /// ignore binding confirmation and etc, for single player looting
    TypeOwner,
}

impl LootSlotType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::TypeAllowLoot => 0x0,
            Self::TypeRollOngoing => 0x1,
            Self::TypeMaster => 0x2,
            Self::TypeLocked => 0x3,
            Self::TypeOwner => 0x4,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::TypeAllowLoot,
            Self::TypeRollOngoing,
            Self::TypeMaster,
            Self::TypeLocked,
            Self::TypeOwner,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::TypeAllowLoot),
            1 => Ok(Self::TypeRollOngoing),
            2 => Ok(Self::TypeMaster),
            3 => Ok(Self::TypeLocked),
            4 => Ok(Self::TypeOwner),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl LootSlotType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::TypeAllowLoot => "TYPE_ALLOW_LOOT",
            Self::TypeRollOngoing => "TYPE_ROLL_ONGOING",
            Self::TypeMaster => "TYPE_MASTER",
            Self::TypeLocked => "TYPE_LOCKED",
            Self::TypeOwner => "TYPE_OWNER",
        }
    }

}

const NAME: &str = "LootSlotType";

impl Default for LootSlotType {
    fn default() -> Self {
        Self::TypeAllowLoot
    }
}

impl std::fmt::Display for LootSlotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeAllowLoot => f.write_str("TypeAllowLoot"),
            Self::TypeRollOngoing => f.write_str("TypeRollOngoing"),
            Self::TypeMaster => f.write_str("TypeMaster"),
            Self::TypeLocked => f.write_str("TypeLocked"),
            Self::TypeOwner => f.write_str("TypeOwner"),
        }
    }
}

impl TryFrom<u8> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

