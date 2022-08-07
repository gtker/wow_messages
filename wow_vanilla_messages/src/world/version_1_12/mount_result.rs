use std::convert::{TryFrom, TryInto};

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
    /// You can't mount that unit!
    ///
    Invalidmountee,
    /// That mount is too far away!
    ///
    Toofaraway,
    /// You're already mounted!
    ///
    Alreadymounted,
    /// That unit can't be mounted!
    ///
    Notmountable,
    /// That mount isn't your pet!
    ///
    Notyourpet,
    /// internal
    ///
    Other,
    /// You can't mount while looting!
    ///
    Looting,
    /// You can't mount because of your race!
    ///
    Racecantmount,
    /// You can't mount while shapeshifted!
    ///
    Shapeshifted,
    /// You dismount before continuing.
    ///
    Forceddismount,
    /// no error
    ///
    Ok,
}

impl MountResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Invalidmountee => 0x0,
            Self::Toofaraway => 0x1,
            Self::Alreadymounted => 0x2,
            Self::Notmountable => 0x3,
            Self::Notyourpet => 0x4,
            Self::Other => 0x5,
            Self::Looting => 0x6,
            Self::Racecantmount => 0x7,
            Self::Shapeshifted => 0x8,
            Self::Forceddismount => 0x9,
            Self::Ok => 0xa,
        }
    }

}

impl Default for MountResult {
    fn default() -> Self {
        Self::Invalidmountee
    }
}

impl std::fmt::Display for MountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalidmountee => f.write_str("Invalidmountee"),
            Self::Toofaraway => f.write_str("Toofaraway"),
            Self::Alreadymounted => f.write_str("Alreadymounted"),
            Self::Notmountable => f.write_str("Notmountable"),
            Self::Notyourpet => f.write_str("Notyourpet"),
            Self::Other => f.write_str("Other"),
            Self::Looting => f.write_str("Looting"),
            Self::Racecantmount => f.write_str("Racecantmount"),
            Self::Shapeshifted => f.write_str("Shapeshifted"),
            Self::Forceddismount => f.write_str("Forceddismount"),
            Self::Ok => f.write_str("Ok"),
        }
    }
}

impl TryFrom<u32> for MountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Invalidmountee),
            1 => Ok(Self::Toofaraway),
            2 => Ok(Self::Alreadymounted),
            3 => Ok(Self::Notmountable),
            4 => Ok(Self::Notyourpet),
            5 => Ok(Self::Other),
            6 => Ok(Self::Looting),
            7 => Ok(Self::Racecantmount),
            8 => Ok(Self::Shapeshifted),
            9 => Ok(Self::Forceddismount),
            10 => Ok(Self::Ok),
            v => Err(crate::errors::EnumError::new("MountResult", v as u32),)
        }
    }
}

