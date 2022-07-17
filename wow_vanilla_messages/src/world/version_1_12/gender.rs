pub use wow_vanilla_base::Gender;

pub(crate) const fn gender_as_int(s: &Gender) -> u8 {
    match s {
        Gender::MALE => 0x0,
        Gender::FEMALE => 0x1,
        Gender::NONE => 0x2,
    }
}

pub(crate) fn gender_try_from(value: u8) -> std::result::Result<Gender, crate::errors::EnumError> {
    match value {
        0 => Ok(Gender::MALE),
        1 => Ok(Gender::FEMALE),
        2 => Ok(Gender::NONE),
        v => Err(crate::errors::EnumError::new("Gender", v as u32),)
    }
}
