/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm#L1):
/// ```text
/// enum AchievementNameLinkType : u8 {
///     NORMAL = 0;
///     CLICKABLE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum AchievementNameLinkType {
    Normal,
    Clickable,
}

impl AchievementNameLinkType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Clickable => 0x1,
        }
    }

}

impl Default for AchievementNameLinkType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for AchievementNameLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Clickable => f.write_str("Clickable"),
        }
    }
}

impl TryFrom<u8> for AchievementNameLinkType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Clickable),
            v => Err(crate::errors::EnumError::new("AchievementNameLinkType", v as u64),)
        }
    }
}

