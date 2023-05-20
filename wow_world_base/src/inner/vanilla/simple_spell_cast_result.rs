/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cast_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_result.wowm#L3):
/// ```text
/// enum SimpleSpellCastResult : u8 {
///     SUCCESS = 0;
///     FAILURE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SimpleSpellCastResult {
    Success,
    Failure,
}

impl SimpleSpellCastResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0x0,
            Self::Failure => 0x2,
        }
    }

}

impl Default for SimpleSpellCastResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for SimpleSpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::Failure => f.write_str("Failure"),
        }
    }
}

impl TryFrom<u8> for SimpleSpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            2 => Ok(Self::Failure),
            v => Err(crate::errors::EnumError::new("SimpleSpellCastResult", v as u64),)
        }
    }
}

