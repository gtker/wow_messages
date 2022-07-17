pub use wow_vanilla_base::Race;

pub(crate) const fn race_as_int(s: &Race) -> u8 {
    match s {
        Race::HUMAN => 0x1,
        Race::ORC => 0x2,
        Race::DWARF => 0x3,
        Race::NIGHTELF => 0x4,
        Race::UNDEAD => 0x5,
        Race::TAUREN => 0x6,
        Race::GNOME => 0x7,
        Race::TROLL => 0x8,
        Race::GOBLIN => 0x9,
    }
}

pub(crate) fn race_try_from(value: u8) -> std::result::Result<Race, crate::errors::EnumError> {
    match value {
        1 => Ok(Race::HUMAN),
        2 => Ok(Race::ORC),
        3 => Ok(Race::DWARF),
        4 => Ok(Race::NIGHTELF),
        5 => Ok(Race::UNDEAD),
        6 => Ok(Race::TAUREN),
        7 => Ok(Race::GNOME),
        8 => Ok(Race::TROLL),
        9 => Ok(Race::GOBLIN),
        v => Err(crate::errors::EnumError::new("Race", v as u32),)
    }
}
