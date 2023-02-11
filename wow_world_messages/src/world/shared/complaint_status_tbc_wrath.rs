/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm#L1):
/// ```text
/// enum ComplaintStatus : u8 {
///     DISABLED = 0;
///     ENABLED_WITHOUT_AUTO_IGNORE = 1;
///     ENABLED_WITH_AUTO_IGNORE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ComplaintStatus {
    Disabled,
    EnabledWithoutAutoIgnore,
    EnabledWithAutoIgnore,
}

impl ComplaintStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Disabled => 0x0,
            Self::EnabledWithoutAutoIgnore => 0x1,
            Self::EnabledWithAutoIgnore => 0x2,
        }
    }

}

impl Default for ComplaintStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

impl std::fmt::Display for ComplaintStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => f.write_str("Disabled"),
            Self::EnabledWithoutAutoIgnore => f.write_str("EnabledWithoutAutoIgnore"),
            Self::EnabledWithAutoIgnore => f.write_str("EnabledWithAutoIgnore"),
        }
    }
}

impl TryFrom<u8> for ComplaintStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::EnabledWithoutAutoIgnore),
            2 => Ok(Self::EnabledWithAutoIgnore),
            v => Err(crate::errors::EnumError::new("ComplaintStatus", v as u64),)
        }
    }
}

