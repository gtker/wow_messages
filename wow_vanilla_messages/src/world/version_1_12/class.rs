pub use wow_vanilla_base::Class;

pub(crate) const fn class_as_int(s: &Class) -> u8 {
    match s {
        Class::WARRIOR => 0x1,
        Class::PALADIN => 0x2,
        Class::HUNTER => 0x3,
        Class::ROGUE => 0x4,
        Class::PRIEST => 0x5,
        Class::SHAMAN => 0x7,
        Class::MAGE => 0x8,
        Class::WARLOCK => 0x9,
        Class::DRUID => 0xb,
    }
}

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
