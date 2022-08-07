use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm#L3):
/// ```text
/// enum ActivateTaxiReply : u32 {
///     OK = 0;
///     UNSPECIFIEDSERVERERROR = 1;
///     NOSUCHPATH = 2;
///     NOTENOUGHMONEY = 3;
///     TOOFARAWAY = 4;
///     NOVENDORNEARBY = 5;
///     NOTVISITED = 6;
///     PLAYERBUSY = 7;
///     PLAYERALREADYMOUNTED = 8;
///     PLAYERSHAPESHIFTED = 9;
///     PLAYERMOVING = 10;
///     SAMENODE = 11;
///     NOTSTANDING = 12;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ActivateTaxiReply {
    Ok,
    Unspecifiedservererror,
    Nosuchpath,
    Notenoughmoney,
    Toofaraway,
    Novendornearby,
    Notvisited,
    Playerbusy,
    Playeralreadymounted,
    Playershapeshifted,
    Playermoving,
    Samenode,
    Notstanding,
}

impl ActivateTaxiReply {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0x0,
            Self::Unspecifiedservererror => 0x1,
            Self::Nosuchpath => 0x2,
            Self::Notenoughmoney => 0x3,
            Self::Toofaraway => 0x4,
            Self::Novendornearby => 0x5,
            Self::Notvisited => 0x6,
            Self::Playerbusy => 0x7,
            Self::Playeralreadymounted => 0x8,
            Self::Playershapeshifted => 0x9,
            Self::Playermoving => 0xa,
            Self::Samenode => 0xb,
            Self::Notstanding => 0xc,
        }
    }

}

impl Default for ActivateTaxiReply {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for ActivateTaxiReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::Unspecifiedservererror => f.write_str("Unspecifiedservererror"),
            Self::Nosuchpath => f.write_str("Nosuchpath"),
            Self::Notenoughmoney => f.write_str("Notenoughmoney"),
            Self::Toofaraway => f.write_str("Toofaraway"),
            Self::Novendornearby => f.write_str("Novendornearby"),
            Self::Notvisited => f.write_str("Notvisited"),
            Self::Playerbusy => f.write_str("Playerbusy"),
            Self::Playeralreadymounted => f.write_str("Playeralreadymounted"),
            Self::Playershapeshifted => f.write_str("Playershapeshifted"),
            Self::Playermoving => f.write_str("Playermoving"),
            Self::Samenode => f.write_str("Samenode"),
            Self::Notstanding => f.write_str("Notstanding"),
        }
    }
}

impl TryFrom<u32> for ActivateTaxiReply {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::Unspecifiedservererror),
            2 => Ok(Self::Nosuchpath),
            3 => Ok(Self::Notenoughmoney),
            4 => Ok(Self::Toofaraway),
            5 => Ok(Self::Novendornearby),
            6 => Ok(Self::Notvisited),
            7 => Ok(Self::Playerbusy),
            8 => Ok(Self::Playeralreadymounted),
            9 => Ok(Self::Playershapeshifted),
            10 => Ok(Self::Playermoving),
            11 => Ok(Self::Samenode),
            12 => Ok(Self::Notstanding),
            v => Err(crate::errors::EnumError::new("ActivateTaxiReply", v as u32),)
        }
    }
}

