/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:171`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L171):
/// ```text
/// enum PageTextMaterial : u8 {
///     NONE = 0;
///     PARCHMENT = 1;
///     STONE = 2;
///     MARBLE = 3;
///     SILVER = 4;
///     BRONZE = 5;
///     VALENTINE = 6;
///     ILLIDAN = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PageTextMaterial {
    None,
    Parchment,
    Stone,
    Marble,
    Silver,
    Bronze,
    Valentine,
    Illidan,
}

impl PageTextMaterial {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Parchment => 0x1,
            Self::Stone => 0x2,
            Self::Marble => 0x3,
            Self::Silver => 0x4,
            Self::Bronze => 0x5,
            Self::Valentine => 0x6,
            Self::Illidan => 0x7,
        }
    }

}

impl Default for PageTextMaterial {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for PageTextMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Parchment => f.write_str("Parchment"),
            Self::Stone => f.write_str("Stone"),
            Self::Marble => f.write_str("Marble"),
            Self::Silver => f.write_str("Silver"),
            Self::Bronze => f.write_str("Bronze"),
            Self::Valentine => f.write_str("Valentine"),
            Self::Illidan => f.write_str("Illidan"),
        }
    }
}

impl TryFrom<u8> for PageTextMaterial {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Parchment),
            2 => Ok(Self::Stone),
            3 => Ok(Self::Marble),
            4 => Ok(Self::Silver),
            5 => Ok(Self::Bronze),
            6 => Ok(Self::Valentine),
            7 => Ok(Self::Illidan),
            v => Err(crate::errors::EnumError::new("PageTextMaterial", v as u64),)
        }
    }
}

