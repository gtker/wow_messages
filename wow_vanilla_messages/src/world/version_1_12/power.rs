pub use wow_vanilla_base::Power;

pub(crate) fn power_try_from(value: u8) -> std::result::Result<Power, crate::errors::EnumError> {
    match value {
        0 => Ok(Power::MANA),
        1 => Ok(Power::RAGE),
        2 => Ok(Power::FOCUS),
        3 => Ok(Power::ENERGY),
        4 => Ok(Power::HAPPINESS),
        254 => Ok(Power::HEALTH),
        v => Err(crate::errors::EnumError::new("Power", v as u32),)
    }
}
