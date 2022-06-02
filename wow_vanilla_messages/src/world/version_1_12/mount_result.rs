use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/mount_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/mount_common.wowm#L3):
/// ```text
/// enum MountResult : u32 {
///     INVALIDMOUNTEE = 0;
///     TOOFARAWAY = 1;
///     ALREADYMOUNTED = 2;
///     NOTMOUNTABLE = 3;
///     NOTYOURPET = 4;
///     OTHER = 5;
///     LOOTING = 6;
///     RACECANTMOUNT = 7;
///     SHAPESHIFTED = 8;
///     FORCEDDISMOUNT = 9;
///     OK = 10;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MountResult {
    /// # Comment
    /// 
    /// You can't mount that unit!
    INVALIDMOUNTEE,
    /// # Comment
    /// 
    /// That mount is too far away!
    TOOFARAWAY,
    /// # Comment
    /// 
    /// You're already mounted!
    ALREADYMOUNTED,
    /// # Comment
    /// 
    /// That unit can't be mounted!
    NOTMOUNTABLE,
    /// # Comment
    /// 
    /// That mount isn't your pet!
    NOTYOURPET,
    /// # Comment
    /// 
    /// internal
    OTHER,
    /// # Comment
    /// 
    /// You can't mount while looting!
    LOOTING,
    /// # Comment
    /// 
    /// You can't mount because of your race!
    RACECANTMOUNT,
    /// # Comment
    /// 
    /// You can't mount while shapeshifted!
    SHAPESHIFTED,
    /// # Comment
    /// 
    /// You dismount before continuing.
    FORCEDDISMOUNT,
    /// # Comment
    /// 
    /// no error
    OK,
}

impl MountResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::INVALIDMOUNTEE => 0x0,
            Self::TOOFARAWAY => 0x1,
            Self::ALREADYMOUNTED => 0x2,
            Self::NOTMOUNTABLE => 0x3,
            Self::NOTYOURPET => 0x4,
            Self::OTHER => 0x5,
            Self::LOOTING => 0x6,
            Self::RACECANTMOUNT => 0x7,
            Self::SHAPESHIFTED => 0x8,
            Self::FORCEDDISMOUNT => 0x9,
            Self::OK => 0xa,
        }
    }

}

impl Default for MountResult {
    fn default() -> Self {
        Self::INVALIDMOUNTEE
    }
}

impl std::fmt::Display for MountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALIDMOUNTEE => f.write_str("INVALIDMOUNTEE"),
            Self::TOOFARAWAY => f.write_str("TOOFARAWAY"),
            Self::ALREADYMOUNTED => f.write_str("ALREADYMOUNTED"),
            Self::NOTMOUNTABLE => f.write_str("NOTMOUNTABLE"),
            Self::NOTYOURPET => f.write_str("NOTYOURPET"),
            Self::OTHER => f.write_str("OTHER"),
            Self::LOOTING => f.write_str("LOOTING"),
            Self::RACECANTMOUNT => f.write_str("RACECANTMOUNT"),
            Self::SHAPESHIFTED => f.write_str("SHAPESHIFTED"),
            Self::FORCEDDISMOUNT => f.write_str("FORCEDDISMOUNT"),
            Self::OK => f.write_str("OK"),
        }
    }
}

impl TryFrom<u32> for MountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::INVALIDMOUNTEE),
            1 => Ok(Self::TOOFARAWAY),
            2 => Ok(Self::ALREADYMOUNTED),
            3 => Ok(Self::NOTMOUNTABLE),
            4 => Ok(Self::NOTYOURPET),
            5 => Ok(Self::OTHER),
            6 => Ok(Self::LOOTING),
            7 => Ok(Self::RACECANTMOUNT),
            8 => Ok(Self::SHAPESHIFTED),
            9 => Ok(Self::FORCEDDISMOUNT),
            10 => Ok(Self::OK),
            v => Err(crate::errors::EnumError::new("MountResult", v as u32),)
        }
    }
}

