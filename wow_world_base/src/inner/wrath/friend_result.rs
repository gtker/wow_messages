/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_status.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_status.wowm#L40):
/// ```text
/// enum FriendResult : u8 {
///     DB_ERROR = 0x00;
///     LIST_FULL = 0x01;
///     ONLINE = 0x02;
///     OFFLINE = 0x03;
///     NOT_FOUND = 0x04;
///     REMOVED = 0x05;
///     ADDED_ONLINE = 0x06;
///     ADDED_OFFLINE = 0x07;
///     ALREADY = 0x08;
///     SELF = 0x09;
///     ENEMY = 0x0A;
///     IGNORE_FULL = 0x0B;
///     IGNORE_SELF = 0x0C;
///     IGNORE_NOT_FOUND = 0x0D;
///     IGNORE_ALREADY = 0x0E;
///     IGNORE_ADDED = 0x0F;
///     IGNORE_REMOVED = 0x10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FriendResult {
    DbError,
    ListFull,
    Online,
    Offline,
    NotFound,
    Removed,
    AddedOnline,
    AddedOffline,
    Already,
    SelfX,
    Enemy,
    IgnoreFull,
    IgnoreSelf,
    IgnoreNotFound,
    IgnoreAlready,
    IgnoreAdded,
    IgnoreRemoved,
}

impl FriendResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::DbError => 0x0,
            Self::ListFull => 0x1,
            Self::Online => 0x2,
            Self::Offline => 0x3,
            Self::NotFound => 0x4,
            Self::Removed => 0x5,
            Self::AddedOnline => 0x6,
            Self::AddedOffline => 0x7,
            Self::Already => 0x8,
            Self::SelfX => 0x9,
            Self::Enemy => 0xa,
            Self::IgnoreFull => 0xb,
            Self::IgnoreSelf => 0xc,
            Self::IgnoreNotFound => 0xd,
            Self::IgnoreAlready => 0xe,
            Self::IgnoreAdded => 0xf,
            Self::IgnoreRemoved => 0x10,
        }
    }

}

impl Default for FriendResult {
    fn default() -> Self {
        Self::DbError
    }
}

impl std::fmt::Display for FriendResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbError => f.write_str("DbError"),
            Self::ListFull => f.write_str("ListFull"),
            Self::Online => f.write_str("Online"),
            Self::Offline => f.write_str("Offline"),
            Self::NotFound => f.write_str("NotFound"),
            Self::Removed => f.write_str("Removed"),
            Self::AddedOnline => f.write_str("AddedOnline"),
            Self::AddedOffline => f.write_str("AddedOffline"),
            Self::Already => f.write_str("Already"),
            Self::SelfX => f.write_str("SelfX"),
            Self::Enemy => f.write_str("Enemy"),
            Self::IgnoreFull => f.write_str("IgnoreFull"),
            Self::IgnoreSelf => f.write_str("IgnoreSelf"),
            Self::IgnoreNotFound => f.write_str("IgnoreNotFound"),
            Self::IgnoreAlready => f.write_str("IgnoreAlready"),
            Self::IgnoreAdded => f.write_str("IgnoreAdded"),
            Self::IgnoreRemoved => f.write_str("IgnoreRemoved"),
        }
    }
}

impl TryFrom<u8> for FriendResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DbError),
            1 => Ok(Self::ListFull),
            2 => Ok(Self::Online),
            3 => Ok(Self::Offline),
            4 => Ok(Self::NotFound),
            5 => Ok(Self::Removed),
            6 => Ok(Self::AddedOnline),
            7 => Ok(Self::AddedOffline),
            8 => Ok(Self::Already),
            9 => Ok(Self::SelfX),
            10 => Ok(Self::Enemy),
            11 => Ok(Self::IgnoreFull),
            12 => Ok(Self::IgnoreSelf),
            13 => Ok(Self::IgnoreNotFound),
            14 => Ok(Self::IgnoreAlready),
            15 => Ok(Self::IgnoreAdded),
            16 => Ok(Self::IgnoreRemoved),
            v => Err(crate::errors::EnumError::new("FriendResult", v as u64),)
        }
    }
}

