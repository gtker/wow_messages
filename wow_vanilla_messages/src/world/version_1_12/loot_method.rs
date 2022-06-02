use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L3):
/// ```text
/// enum LootMethod : u8 {
///     CORPSE = 1;
///     PICKPOCKETING = 2;
///     FISHING = 3;
///     DISENCHANTING = 4;
///     SKINNING = 6;
///     FISHINGHOLE = 20;
///     FISHING_FAIL = 21;
///     INSIGNIA = 22;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LootMethod {
    CORPSE,
    PICKPOCKETING,
    FISHING,
    DISENCHANTING,
    /// unsupported by client, send LOOT_PICKPOCKETING instead
    ///
    SKINNING,
    /// unsupported by client, send LOOT_FISHING instead
    ///
    FISHINGHOLE,
    /// unsupported by client, send LOOT_FISHING instead
    ///
    FISHING_FAIL,
    /// unsupported by client, send LOOT_CORPSE instead
    ///
    INSIGNIA,
}

impl LootMethod {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CORPSE => 0x1,
            Self::PICKPOCKETING => 0x2,
            Self::FISHING => 0x3,
            Self::DISENCHANTING => 0x4,
            Self::SKINNING => 0x6,
            Self::FISHINGHOLE => 0x14,
            Self::FISHING_FAIL => 0x15,
            Self::INSIGNIA => 0x16,
        }
    }

}

impl Default for LootMethod {
    fn default() -> Self {
        Self::CORPSE
    }
}

impl std::fmt::Display for LootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CORPSE => f.write_str("CORPSE"),
            Self::PICKPOCKETING => f.write_str("PICKPOCKETING"),
            Self::FISHING => f.write_str("FISHING"),
            Self::DISENCHANTING => f.write_str("DISENCHANTING"),
            Self::SKINNING => f.write_str("SKINNING"),
            Self::FISHINGHOLE => f.write_str("FISHINGHOLE"),
            Self::FISHING_FAIL => f.write_str("FISHING_FAIL"),
            Self::INSIGNIA => f.write_str("INSIGNIA"),
        }
    }
}

impl TryFrom<u8> for LootMethod {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CORPSE),
            2 => Ok(Self::PICKPOCKETING),
            3 => Ok(Self::FISHING),
            4 => Ok(Self::DISENCHANTING),
            6 => Ok(Self::SKINNING),
            20 => Ok(Self::FISHINGHOLE),
            21 => Ok(Self::FISHING_FAIL),
            22 => Ok(Self::INSIGNIA),
            v => Err(crate::errors::EnumError::new("LootMethod", v as u32),)
        }
    }
}

