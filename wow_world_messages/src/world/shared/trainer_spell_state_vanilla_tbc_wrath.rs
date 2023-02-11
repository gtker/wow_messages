/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L1):
/// ```text
/// enum TrainerSpellState : u8 {
///     GREEN = 0;
///     RED = 1;
///     GRAY = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainerSpellState {
    Green,
    Red,
    Gray,
}

impl TrainerSpellState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Green => 0x0,
            Self::Red => 0x1,
            Self::Gray => 0x2,
        }
    }

}

impl Default for TrainerSpellState {
    fn default() -> Self {
        Self::Green
    }
}

impl std::fmt::Display for TrainerSpellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Green => f.write_str("Green"),
            Self::Red => f.write_str("Red"),
            Self::Gray => f.write_str("Gray"),
        }
    }
}

impl TryFrom<u8> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Green),
            1 => Ok(Self::Red),
            2 => Ok(Self::Gray),
            v => Err(crate::errors::EnumError::new("TrainerSpellState", v as u64),)
        }
    }
}

