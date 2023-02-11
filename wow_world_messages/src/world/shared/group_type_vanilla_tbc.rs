/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L1):
/// ```text
/// enum GroupType : u8 {
///     NORMAL = 0;
///     RAID = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GroupType {
    Normal,
    Raid,
}

impl GroupType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Raid => 0x1,
        }
    }

}

impl Default for GroupType {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Raid => f.write_str("Raid"),
        }
    }
}

impl TryFrom<u8> for GroupType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Raid),
            v => Err(crate::errors::EnumError::new("GroupType", v as u64),)
        }
    }
}

