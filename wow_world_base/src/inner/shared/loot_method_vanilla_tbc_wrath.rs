/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L1):
/// ```text
/// enum LootMethod : u8 {
///     ERROR = 0;
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LootMethod {
    ErrorX,
    Corpse,
    Pickpocketing,
    Fishing,
    Disenchanting,
    /// unsupported by client, send LOOT_PICKPOCKETING instead
    ///
    Skinning,
    /// unsupported by client, send LOOT_FISHING instead
    ///
    Fishinghole,
    /// unsupported by client, send LOOT_FISHING instead
    ///
    FishingFail,
    /// unsupported by client, send LOOT_CORPSE instead
    ///
    Insignia,
}

impl LootMethod {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::ErrorX => 0x0,
            Self::Corpse => 0x1,
            Self::Pickpocketing => 0x2,
            Self::Fishing => 0x3,
            Self::Disenchanting => 0x4,
            Self::Skinning => 0x6,
            Self::Fishinghole => 0x14,
            Self::FishingFail => 0x15,
            Self::Insignia => 0x16,
        }
    }

}

impl Default for LootMethod {
    fn default() -> Self {
        Self::ErrorX
    }
}

impl std::fmt::Display for LootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ErrorX => f.write_str("ErrorX"),
            Self::Corpse => f.write_str("Corpse"),
            Self::Pickpocketing => f.write_str("Pickpocketing"),
            Self::Fishing => f.write_str("Fishing"),
            Self::Disenchanting => f.write_str("Disenchanting"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Fishinghole => f.write_str("Fishinghole"),
            Self::FishingFail => f.write_str("FishingFail"),
            Self::Insignia => f.write_str("Insignia"),
        }
    }
}

impl TryFrom<u8> for LootMethod {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ErrorX),
            1 => Ok(Self::Corpse),
            2 => Ok(Self::Pickpocketing),
            3 => Ok(Self::Fishing),
            4 => Ok(Self::Disenchanting),
            6 => Ok(Self::Skinning),
            20 => Ok(Self::Fishinghole),
            21 => Ok(Self::FishingFail),
            22 => Ok(Self::Insignia),
            v => Err(crate::errors::EnumError::new("LootMethod", v as u64),)
        }
    }
}

