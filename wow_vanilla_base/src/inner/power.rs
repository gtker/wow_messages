/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L3):
/// ```text
/// enum Power : u8 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     HEALTH = 0xFE;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Power {
    /// mangoszero: The most common one, mobs usually have this or rage
    ///
    MANA,
    /// mangoszero: This is what warriors use to cast their spells
    ///
    RAGE,
    /// mangoszero: Used by hunters after Cataclysm (4.x)
    ///
    FOCUS,
    /// mangoszero: Used by rouges to do their spells
    ///
    ENERGY,
    /// mangoszero: Hunter's pet's happiness affect their damage
    ///
    HAPPINESS,
    /// mangoszero: Health, everyone has this (-2 as signed value)
    /// This might not actually be sent to the client.
    ///
    HEALTH,
}

impl Default for Power {
    fn default() -> Self {
        Self::MANA
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MANA => f.write_str("MANA"),
            Self::RAGE => f.write_str("RAGE"),
            Self::FOCUS => f.write_str("FOCUS"),
            Self::ENERGY => f.write_str("ENERGY"),
            Self::HAPPINESS => f.write_str("HAPPINESS"),
            Self::HEALTH => f.write_str("HEALTH"),
        }
    }
}

impl Power {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::MANA => 0x0,
            Self::RAGE => 0x1,
            Self::FOCUS => 0x2,
            Self::ENERGY => 0x3,
            Self::HAPPINESS => 0x4,
            Self::HEALTH => 0xfe,
        }
    }

}

