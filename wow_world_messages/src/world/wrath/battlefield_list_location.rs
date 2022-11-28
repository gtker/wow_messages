use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm#L7):
/// ```text
/// enum BattlefieldListLocation : u8 {
///     BATTLEMASTER = 0;
///     UI = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlefieldListLocation {
    /// lua: ShowBattlefieldList
    ///
    Battlemaster,
    /// lua: RequestBattlegroundInstanceInfo
    ///
    Ui,
}

impl BattlefieldListLocation {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Battlemaster => 0x0,
            Self::Ui => 0x1,
        }
    }

}

impl Default for BattlefieldListLocation {
    fn default() -> Self {
        Self::Battlemaster
    }
}

impl std::fmt::Display for BattlefieldListLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Battlemaster => f.write_str("Battlemaster"),
            Self::Ui => f.write_str("Ui"),
        }
    }
}

impl TryFrom<u8> for BattlefieldListLocation {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Battlemaster),
            1 => Ok(Self::Ui),
            v => Err(crate::errors::EnumError::new("BattlefieldListLocation", v as u32),)
        }
    }
}

