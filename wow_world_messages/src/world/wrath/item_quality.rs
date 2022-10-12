use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common_3_3_5.wowm:236`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common_3_3_5.wowm#L236):
/// ```text
/// enum ItemQuality : u8 {
///     POOR = 0;
///     NORMAL = 1;
///     UNCOMMON = 2;
///     RARE = 3;
///     EPIC = 4;
///     LEGENDARY = 5;
///     ARTIFACT = 6;
///     HEIRLOOM = 7;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ItemQuality {
    Poor,
    Normal,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Artifact,
    Heirloom,
}

impl ItemQuality {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Poor => 0x0,
            Self::Normal => 0x1,
            Self::Uncommon => 0x2,
            Self::Rare => 0x3,
            Self::Epic => 0x4,
            Self::Legendary => 0x5,
            Self::Artifact => 0x6,
            Self::Heirloom => 0x7,
        }
    }

}

impl Default for ItemQuality {
    fn default() -> Self {
        Self::Poor
    }
}

impl std::fmt::Display for ItemQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Poor => f.write_str("Poor"),
            Self::Normal => f.write_str("Normal"),
            Self::Uncommon => f.write_str("Uncommon"),
            Self::Rare => f.write_str("Rare"),
            Self::Epic => f.write_str("Epic"),
            Self::Legendary => f.write_str("Legendary"),
            Self::Artifact => f.write_str("Artifact"),
            Self::Heirloom => f.write_str("Heirloom"),
        }
    }
}

impl TryFrom<u8> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Poor),
            1 => Ok(Self::Normal),
            2 => Ok(Self::Uncommon),
            3 => Ok(Self::Rare),
            4 => Ok(Self::Epic),
            5 => Ok(Self::Legendary),
            6 => Ok(Self::Artifact),
            7 => Ok(Self::Heirloom),
            v => Err(crate::errors::EnumError::new("ItemQuality", v as u32),)
        }
    }
}

