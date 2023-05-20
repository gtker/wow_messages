/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_talents_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_talents_info.wowm#L1):
/// ```text
/// enum TalentInfoType : u8 {
///     PLAYER = 0;
///     PET = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TalentInfoType {
    Player,
    Pet,
}

impl TalentInfoType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Player => 0x0,
            Self::Pet => 0x1,
        }
    }

}

impl Default for TalentInfoType {
    fn default() -> Self {
        Self::Player
    }
}

impl std::fmt::Display for TalentInfoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => f.write_str("Player"),
            Self::Pet => f.write_str("Pet"),
        }
    }
}

impl TryFrom<u8> for TalentInfoType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Player),
            1 => Ok(Self::Pet),
            v => Err(crate::errors::EnumError::new("TalentInfoType", v as u64),)
        }
    }
}

