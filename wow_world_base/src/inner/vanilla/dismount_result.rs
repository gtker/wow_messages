/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_dismountresult.wowm#L3):
/// ```text
/// enum DismountResult : u32 {
///     NOT_MOUNTED = 1;
///     OK = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DismountResult {
    NotMounted,
    Ok,
}

impl DismountResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::NotMounted => 0x1,
            Self::Ok => 0x3,
        }
    }

}

impl Default for DismountResult {
    fn default() -> Self {
        Self::NotMounted
    }
}

impl std::fmt::Display for DismountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotMounted => f.write_str("NotMounted"),
            Self::Ok => f.write_str("Ok"),
        }
    }
}

impl TryFrom<u32> for DismountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::NotMounted),
            3 => Ok(Self::Ok),
            v => Err(crate::errors::EnumError::new("DismountResult", v as u64),)
        }
    }
}

