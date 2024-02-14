/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:244`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L244):
/// ```text
/// enum ItemClass : u8 {
///     CONSUMABLE = 0;
///     CONTAINER = 1;
///     WEAPON = 2;
///     GEM = 3;
///     ARMOR = 4;
///     REAGENT = 5;
///     PROJECTILE = 6;
///     TRADE_GOODS = 7;
///     GENERIC = 8;
///     RECIPE = 9;
///     MONEY = 10;
///     QUIVER = 11;
///     QUEST = 12;
///     KEY = 13;
///     PERMANENT = 14;
///     MISC = 15;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemClass {
    Consumable,
    Container,
    Weapon,
    Gem,
    Armor,
    Reagent,
    Projectile,
    TradeGoods,
    Generic,
    Recipe,
    Money,
    Quiver,
    Quest,
    Key,
    Permanent,
    Misc,
}

impl ItemClass {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Consumable => 0x0,
            Self::Container => 0x1,
            Self::Weapon => 0x2,
            Self::Gem => 0x3,
            Self::Armor => 0x4,
            Self::Reagent => 0x5,
            Self::Projectile => 0x6,
            Self::TradeGoods => 0x7,
            Self::Generic => 0x8,
            Self::Recipe => 0x9,
            Self::Money => 0xa,
            Self::Quiver => 0xb,
            Self::Quest => 0xc,
            Self::Key => 0xd,
            Self::Permanent => 0xe,
            Self::Misc => 0xf,
        }
    }

    pub const fn variants() -> [Self; 16] {
        [
            Self::Consumable,
            Self::Container,
            Self::Weapon,
            Self::Gem,
            Self::Armor,
            Self::Reagent,
            Self::Projectile,
            Self::TradeGoods,
            Self::Generic,
            Self::Recipe,
            Self::Money,
            Self::Quiver,
            Self::Quest,
            Self::Key,
            Self::Permanent,
            Self::Misc,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Consumable),
            1 => Ok(Self::Container),
            2 => Ok(Self::Weapon),
            3 => Ok(Self::Gem),
            4 => Ok(Self::Armor),
            5 => Ok(Self::Reagent),
            6 => Ok(Self::Projectile),
            7 => Ok(Self::TradeGoods),
            8 => Ok(Self::Generic),
            9 => Ok(Self::Recipe),
            10 => Ok(Self::Money),
            11 => Ok(Self::Quiver),
            12 => Ok(Self::Quest),
            13 => Ok(Self::Key),
            14 => Ok(Self::Permanent),
            15 => Ok(Self::Misc),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ItemClass {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Consumable => "CONSUMABLE",
            Self::Container => "CONTAINER",
            Self::Weapon => "WEAPON",
            Self::Gem => "GEM",
            Self::Armor => "ARMOR",
            Self::Reagent => "REAGENT",
            Self::Projectile => "PROJECTILE",
            Self::TradeGoods => "TRADE_GOODS",
            Self::Generic => "GENERIC",
            Self::Recipe => "RECIPE",
            Self::Money => "MONEY",
            Self::Quiver => "QUIVER",
            Self::Quest => "QUEST",
            Self::Key => "KEY",
            Self::Permanent => "PERMANENT",
            Self::Misc => "MISC",
        }
    }

}

const NAME: &str = "ItemClass";

impl Default for ItemClass {
    fn default() -> Self {
        Self::Consumable
    }
}

impl std::fmt::Display for ItemClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Consumable => f.write_str("Consumable"),
            Self::Container => f.write_str("Container"),
            Self::Weapon => f.write_str("Weapon"),
            Self::Gem => f.write_str("Gem"),
            Self::Armor => f.write_str("Armor"),
            Self::Reagent => f.write_str("Reagent"),
            Self::Projectile => f.write_str("Projectile"),
            Self::TradeGoods => f.write_str("TradeGoods"),
            Self::Generic => f.write_str("Generic"),
            Self::Recipe => f.write_str("Recipe"),
            Self::Money => f.write_str("Money"),
            Self::Quiver => f.write_str("Quiver"),
            Self::Quest => f.write_str("Quest"),
            Self::Key => f.write_str("Key"),
            Self::Permanent => f.write_str("Permanent"),
            Self::Misc => f.write_str("Misc"),
        }
    }
}

impl TryFrom<u8> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ItemClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

