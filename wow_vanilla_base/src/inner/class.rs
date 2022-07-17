/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/class.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/class.wowm#L3):
/// ```text
/// enum Class : u8 {
///     WARRIOR = 1;
///     PALADIN = 2;
///     HUNTER = 3;
///     ROGUE = 4;
///     PRIEST = 5;
///     SHAMAN = 7;
///     MAGE = 8;
///     WARLOCK = 9;
///     DRUID = 11;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Class {
    WARRIOR,
    PALADIN,
    HUNTER,
    ROGUE,
    PRIEST,
    SHAMAN,
    MAGE,
    WARLOCK,
    DRUID,
}

impl Default for Class {
    fn default() -> Self {
        Self::WARRIOR
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WARRIOR => f.write_str("WARRIOR"),
            Self::PALADIN => f.write_str("PALADIN"),
            Self::HUNTER => f.write_str("HUNTER"),
            Self::ROGUE => f.write_str("ROGUE"),
            Self::PRIEST => f.write_str("PRIEST"),
            Self::SHAMAN => f.write_str("SHAMAN"),
            Self::MAGE => f.write_str("MAGE"),
            Self::WARLOCK => f.write_str("WARLOCK"),
            Self::DRUID => f.write_str("DRUID"),
        }
    }
}

