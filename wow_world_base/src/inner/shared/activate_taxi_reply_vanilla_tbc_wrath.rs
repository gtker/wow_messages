/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm#L3):
/// ```text
/// enum ActivateTaxiReply : u32 {
///     OK = 0;
///     UNSPECIFIED_SERVER_ERROR = 1;
///     NO_SUCH_PATH = 2;
///     NOT_ENOUGH_MONEY = 3;
///     TOO_FAR_AWAY = 4;
///     NO_VENDOR_NEARBY = 5;
///     NOT_VISITED = 6;
///     PLAYER_BUSY = 7;
///     PLAYER_ALREADY_MOUNTED = 8;
///     PLAYER_SHAPE_SHIFTED = 9;
///     PLAYER_MOVING = 10;
///     SAME_NODE = 11;
///     NOT_STANDING = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ActivateTaxiReply {
    Ok,
    UnspecifiedServerError,
    NoSuchPath,
    NotEnoughMoney,
    TooFarAway,
    NoVendorNearby,
    NotVisited,
    PlayerBusy,
    PlayerAlreadyMounted,
    PlayerShapeShifted,
    PlayerMoving,
    SameNode,
    NotStanding,
}

impl ActivateTaxiReply {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0x0,
            Self::UnspecifiedServerError => 0x1,
            Self::NoSuchPath => 0x2,
            Self::NotEnoughMoney => 0x3,
            Self::TooFarAway => 0x4,
            Self::NoVendorNearby => 0x5,
            Self::NotVisited => 0x6,
            Self::PlayerBusy => 0x7,
            Self::PlayerAlreadyMounted => 0x8,
            Self::PlayerShapeShifted => 0x9,
            Self::PlayerMoving => 0xa,
            Self::SameNode => 0xb,
            Self::NotStanding => 0xc,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ActivateTaxiReply {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::UnspecifiedServerError => "UNSPECIFIED_SERVER_ERROR",
            Self::NoSuchPath => "NO_SUCH_PATH",
            Self::NotEnoughMoney => "NOT_ENOUGH_MONEY",
            Self::TooFarAway => "TOO_FAR_AWAY",
            Self::NoVendorNearby => "NO_VENDOR_NEARBY",
            Self::NotVisited => "NOT_VISITED",
            Self::PlayerBusy => "PLAYER_BUSY",
            Self::PlayerAlreadyMounted => "PLAYER_ALREADY_MOUNTED",
            Self::PlayerShapeShifted => "PLAYER_SHAPE_SHIFTED",
            Self::PlayerMoving => "PLAYER_MOVING",
            Self::SameNode => "SAME_NODE",
            Self::NotStanding => "NOT_STANDING",
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
            Self::UnspecifiedServerError => f.write_str("UnspecifiedServerError"),
            Self::NoSuchPath => f.write_str("NoSuchPath"),
            Self::NotEnoughMoney => f.write_str("NotEnoughMoney"),
            Self::TooFarAway => f.write_str("TooFarAway"),
            Self::NoVendorNearby => f.write_str("NoVendorNearby"),
            Self::NotVisited => f.write_str("NotVisited"),
            Self::PlayerBusy => f.write_str("PlayerBusy"),
            Self::PlayerAlreadyMounted => f.write_str("PlayerAlreadyMounted"),
            Self::PlayerShapeShifted => f.write_str("PlayerShapeShifted"),
            Self::PlayerMoving => f.write_str("PlayerMoving"),
            Self::SameNode => f.write_str("SameNode"),
            Self::NotStanding => f.write_str("NotStanding"),
        }
    }
}

impl TryFrom<u32> for ActivateTaxiReply {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::UnspecifiedServerError),
            2 => Ok(Self::NoSuchPath),
            3 => Ok(Self::NotEnoughMoney),
            4 => Ok(Self::TooFarAway),
            5 => Ok(Self::NoVendorNearby),
            6 => Ok(Self::NotVisited),
            7 => Ok(Self::PlayerBusy),
            8 => Ok(Self::PlayerAlreadyMounted),
            9 => Ok(Self::PlayerShapeShifted),
            10 => Ok(Self::PlayerMoving),
            11 => Ok(Self::SameNode),
            12 => Ok(Self::NotStanding),
            v => Err(crate::errors::EnumError::new("ActivateTaxiReply", v.into()),)
        }
    }
}

