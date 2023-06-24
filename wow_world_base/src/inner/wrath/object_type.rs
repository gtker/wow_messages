/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:76`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L76):
/// ```text
/// enum ObjectType : u8 {
///     OBJECT = 0;
///     ITEM = 1;
///     CONTAINER = 2;
///     UNIT = 3;
///     PLAYER = 4;
///     GAME_OBJECT = 5;
///     DYNAMIC_OBJECT = 6;
///     CORPSE = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ObjectType {
    Object,
    Item,
    Container,
    Unit,
    Player,
    GameObject,
    DynamicObject,
    Corpse,
}

impl ObjectType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Object => 0x0,
            Self::Item => 0x1,
            Self::Container => 0x2,
            Self::Unit => 0x3,
            Self::Player => 0x4,
            Self::GameObject => 0x5,
            Self::DynamicObject => 0x6,
            Self::Corpse => 0x7,
        }
    }

    pub const fn variants() -> [Self; 8] {
        [
            Self::Object,
            Self::Item,
            Self::Container,
            Self::Unit,
            Self::Player,
            Self::GameObject,
            Self::DynamicObject,
            Self::Corpse,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl ObjectType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Object => "OBJECT",
            Self::Item => "ITEM",
            Self::Container => "CONTAINER",
            Self::Unit => "UNIT",
            Self::Player => "PLAYER",
            Self::GameObject => "GAME_OBJECT",
            Self::DynamicObject => "DYNAMIC_OBJECT",
            Self::Corpse => "CORPSE",
        }
    }

}

const NAME: &str = "ObjectType";

impl Default for ObjectType {
    fn default() -> Self {
        Self::Object
    }
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object => f.write_str("Object"),
            Self::Item => f.write_str("Item"),
            Self::Container => f.write_str("Container"),
            Self::Unit => f.write_str("Unit"),
            Self::Player => f.write_str("Player"),
            Self::GameObject => f.write_str("GameObject"),
            Self::DynamicObject => f.write_str("DynamicObject"),
            Self::Corpse => f.write_str("Corpse"),
        }
    }
}

impl TryFrom<u8> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Object),
            1 => Ok(Self::Item),
            2 => Ok(Self::Container),
            3 => Ok(Self::Unit),
            4 => Ok(Self::Player),
            5 => Ok(Self::GameObject),
            6 => Ok(Self::DynamicObject),
            7 => Ok(Self::Corpse),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

