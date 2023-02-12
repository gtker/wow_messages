/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm#L3):
/// ```text
/// enum EnvironmentalDamageType : u32 {
///     EXHAUSTED = 0;
///     DROWNING = 1;
///     FALL = 2;
///     LAVA = 3;
///     SLIME = 4;
///     FIRE = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum EnvironmentalDamageType {
    Exhausted,
    Drowning,
    Fall,
    Lava,
    Slime,
    Fire,
}

impl EnvironmentalDamageType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Exhausted => 0x0,
            Self::Drowning => 0x1,
            Self::Fall => 0x2,
            Self::Lava => 0x3,
            Self::Slime => 0x4,
            Self::Fire => 0x5,
        }
    }

}

impl Default for EnvironmentalDamageType {
    fn default() -> Self {
        Self::Exhausted
    }
}

impl std::fmt::Display for EnvironmentalDamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exhausted => f.write_str("Exhausted"),
            Self::Drowning => f.write_str("Drowning"),
            Self::Fall => f.write_str("Fall"),
            Self::Lava => f.write_str("Lava"),
            Self::Slime => f.write_str("Slime"),
            Self::Fire => f.write_str("Fire"),
        }
    }
}

impl TryFrom<u32> for EnvironmentalDamageType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Exhausted),
            1 => Ok(Self::Drowning),
            2 => Ok(Self::Fall),
            3 => Ok(Self::Lava),
            4 => Ok(Self::Slime),
            5 => Ok(Self::Fire),
            v => Err(crate::errors::EnumError::new("EnvironmentalDamageType", v as u64),)
        }
    }
}

