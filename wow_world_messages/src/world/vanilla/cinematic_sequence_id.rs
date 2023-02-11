/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm#L1):
/// ```text
/// enum CinematicSequenceId : u32 {
///     GOBLIN = 0;
///     UNDEAD = 2;
///     ORC = 21;
///     DWARF = 41;
///     NIGHT_ELF = 61;
///     HUMAN = 81;
///     GNOME = 101;
///     TROLL = 121;
///     TAUREN = 141;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum CinematicSequenceId {
    Goblin,
    Undead,
    Orc,
    Dwarf,
    NightElf,
    Human,
    Gnome,
    Troll,
    Tauren,
}

impl CinematicSequenceId {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Goblin => 0x0,
            Self::Undead => 0x2,
            Self::Orc => 0x15,
            Self::Dwarf => 0x29,
            Self::NightElf => 0x3d,
            Self::Human => 0x51,
            Self::Gnome => 0x65,
            Self::Troll => 0x79,
            Self::Tauren => 0x8d,
        }
    }

}

impl Default for CinematicSequenceId {
    fn default() -> Self {
        Self::Goblin
    }
}

impl std::fmt::Display for CinematicSequenceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Goblin => f.write_str("Goblin"),
            Self::Undead => f.write_str("Undead"),
            Self::Orc => f.write_str("Orc"),
            Self::Dwarf => f.write_str("Dwarf"),
            Self::NightElf => f.write_str("NightElf"),
            Self::Human => f.write_str("Human"),
            Self::Gnome => f.write_str("Gnome"),
            Self::Troll => f.write_str("Troll"),
            Self::Tauren => f.write_str("Tauren"),
        }
    }
}

impl TryFrom<u32> for CinematicSequenceId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Goblin),
            2 => Ok(Self::Undead),
            21 => Ok(Self::Orc),
            41 => Ok(Self::Dwarf),
            61 => Ok(Self::NightElf),
            81 => Ok(Self::Human),
            101 => Ok(Self::Gnome),
            121 => Ok(Self::Troll),
            141 => Ok(Self::Tauren),
            v => Err(crate::errors::EnumError::new("CinematicSequenceId", v as u64),)
        }
    }
}

