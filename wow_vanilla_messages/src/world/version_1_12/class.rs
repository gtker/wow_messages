pub use wow_vanilla_base::Class;

pub(crate) fn class_try_from(value: u8) -> std::result::Result<Class, crate::errors::EnumError> {
    match value {
        1 => Ok(Class::WARRIOR),
        2 => Ok(Class::PALADIN),
        3 => Ok(Class::HUNTER),
        4 => Ok(Class::ROGUE),
        5 => Ok(Class::PRIEST),
        7 => Ok(Class::SHAMAN),
        8 => Ok(Class::MAGE),
        9 => Ok(Class::WARLOCK),
        11 => Ok(Class::DRUID),
        v => Err(crate::errors::EnumError::new("Class", v as u32),)
    }
}
