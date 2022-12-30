use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_mountresult.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_mountresult.wowm#L3):
/// ```text
/// enum MountResult : u32 {
///     INVALID_MOUNTEE = 0;
///     TOO_FAR_AWAY = 1;
///     ALREADY_MOUNTED = 2;
///     NOT_MOUNTABLE = 3;
///     NOT_YOUR_PET = 4;
///     OTHER = 5;
///     LOOTING = 6;
///     RACE_CANT_MOUNT = 7;
///     SHAPESHIFTED = 8;
///     FORCED_DISMOUNT = 9;
///     OK = 10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MountResult {
    /// You can't mount that unit!
    ///
    InvalidMountee,
    /// That mount is too far away!
    ///
    TooFarAway,
    /// You're already mounted!
    ///
    AlreadyMounted,
    /// That unit can't be mounted!
    ///
    NotMountable,
    /// That mount isn't your pet!
    ///
    NotYourPet,
    /// internal
    ///
    Other,
    /// You can't mount while looting!
    ///
    Looting,
    /// You can't mount because of your race!
    ///
    RaceCantMount,
    /// You can't mount while shapeshifted!
    ///
    Shapeshifted,
    /// You dismount before continuing.
    ///
    ForcedDismount,
    /// no error
    ///
    Ok,
}

impl MountResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::InvalidMountee => 0x0,
            Self::TooFarAway => 0x1,
            Self::AlreadyMounted => 0x2,
            Self::NotMountable => 0x3,
            Self::NotYourPet => 0x4,
            Self::Other => 0x5,
            Self::Looting => 0x6,
            Self::RaceCantMount => 0x7,
            Self::Shapeshifted => 0x8,
            Self::ForcedDismount => 0x9,
            Self::Ok => 0xa,
        }
    }

}

impl Default for MountResult {
    fn default() -> Self {
        Self::InvalidMountee
    }
}

impl std::fmt::Display for MountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidMountee => f.write_str("InvalidMountee"),
            Self::TooFarAway => f.write_str("TooFarAway"),
            Self::AlreadyMounted => f.write_str("AlreadyMounted"),
            Self::NotMountable => f.write_str("NotMountable"),
            Self::NotYourPet => f.write_str("NotYourPet"),
            Self::Other => f.write_str("Other"),
            Self::Looting => f.write_str("Looting"),
            Self::RaceCantMount => f.write_str("RaceCantMount"),
            Self::Shapeshifted => f.write_str("Shapeshifted"),
            Self::ForcedDismount => f.write_str("ForcedDismount"),
            Self::Ok => f.write_str("Ok"),
        }
    }
}

impl TryFrom<u32> for MountResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::InvalidMountee),
            1 => Ok(Self::TooFarAway),
            2 => Ok(Self::AlreadyMounted),
            3 => Ok(Self::NotMountable),
            4 => Ok(Self::NotYourPet),
            5 => Ok(Self::Other),
            6 => Ok(Self::Looting),
            7 => Ok(Self::RaceCantMount),
            8 => Ok(Self::Shapeshifted),
            9 => Ok(Self::ForcedDismount),
            10 => Ok(Self::Ok),
            v => Err(crate::errors::EnumError::new("MountResult", v as u64),)
        }
    }
}

