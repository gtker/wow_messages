pub use wow_vanilla_base::Gender;

pub(crate) fn gender_try_from(value: u8) -> std::result::Result<Gender, crate::errors::EnumError> {
    match value {
        0 => Ok(Gender::MALE),
        1 => Ok(Gender::FEMALE),
        2 => Ok(Gender::NONE),
        v => Err(crate::errors::EnumError::new("Gender", v as u32),)
    }
}
