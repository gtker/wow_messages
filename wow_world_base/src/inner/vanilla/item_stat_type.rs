use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:64`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L64):
/// ```text
/// enum ItemStatType : u8 {
///     MANA = 0;
///     HEALTH = 1;
///     AGILITY = 3;
///     STRENGTH = 4;
///     INTELLECT = 5;
///     SPIRIT = 6;
///     STAMINA = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ItemStatType {
    Mana,
    Health,
    Agility,
    Strength,
    Intellect,
    Spirit,
    Stamina,
}

impl ItemStatType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Mana => 0x0,
            Self::Health => 0x1,
            Self::Agility => 0x3,
            Self::Strength => 0x4,
            Self::Intellect => 0x5,
            Self::Spirit => 0x6,
            Self::Stamina => 0x7,
        }
    }

}

impl Default for ItemStatType {
    fn default() -> Self {
        Self::Mana
    }
}

impl std::fmt::Display for ItemStatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mana => f.write_str("Mana"),
            Self::Health => f.write_str("Health"),
            Self::Agility => f.write_str("Agility"),
            Self::Strength => f.write_str("Strength"),
            Self::Intellect => f.write_str("Intellect"),
            Self::Spirit => f.write_str("Spirit"),
            Self::Stamina => f.write_str("Stamina"),
        }
    }
}

impl TryFrom<u8> for ItemStatType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Mana),
            1 => Ok(Self::Health),
            3 => Ok(Self::Agility),
            4 => Ok(Self::Strength),
            5 => Ok(Self::Intellect),
            6 => Ok(Self::Spirit),
            7 => Ok(Self::Stamina),
            v => Err(crate::errors::EnumError::new("ItemStatType", v as u64),)
        }
    }
}

