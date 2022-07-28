pub use wow_vanilla_base::Race;

pub(crate) fn race_try_from(value: u8) -> std::result::Result<Race, crate::errors::EnumError> {
    match value {
        1 => Ok(Race::HUMAN),
        2 => Ok(Race::ORC),
        3 => Ok(Race::DWARF),
        4 => Ok(Race::NIGHT_ELF),
        5 => Ok(Race::UNDEAD),
        6 => Ok(Race::TAUREN),
        7 => Ok(Race::GNOME),
        8 => Ok(Race::TROLL),
        9 => Ok(Race::GOBLIN),
        v => Err(crate::errors::EnumError::new("Race", v as u32),)
    }
}
