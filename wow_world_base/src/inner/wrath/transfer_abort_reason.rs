/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:57`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L57):
/// ```text
/// enum TransferAbortReason : u8 {
///     NONE = 0x00;
///     ERROR = 0x01;
///     MAX_PLAYERS = 0x02;
///     NOT_FOUND = 0x03;
///     TOO_MANY_INSTANCES = 0x04;
///     ZONE_IN_COMBAT = 0x06;
///     INSUFFICIENT_EXPANSION_LEVEL = 0x07;
///     DIFFICULTY_NOT_AVAILABLE = 0x08;
///     UNIQUE_MESSAGE = 0x09;
///     TOO_MANY_REALM_INSTANCES = 0x0A;
///     NEED_GROUP = 0x0B;
///     NOT_FOUND1 = 0x0C;
///     NOT_FOUND2 = 0x0D;
///     NOT_FOUND3 = 0x0E;
///     REALM_ONLY = 0x0F;
///     MAP_NOT_ALLOWED = 0x10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TransferAbortReason {
    None,
    ErrorX,
    /// Transfer Aborted: instance is full
    ///
    MaxPlayers,
    /// Transfer Aborted: instance not found
    ///
    NotFound,
    /// You have entered too many instances recently.
    ///
    TooManyInstances,
    /// Unable to zone in while an encounter is in progress.
    ///
    ZoneInCombat,
    /// You must have TBC/WotLK expansion installed to access this area.
    ///
    InsufficientExpansionLevel,
    /// Normal/Heroic/Epic difficulty mode is not available for %s.
    ///
    DifficultyNotAvailable,
    /// Until you've escaped The Lich Kings grasp, you cannot leave this place!
    ///
    UniqueMessage,
    /// Additional instances cannot be launched, please try again later.
    ///
    TooManyRealmInstances,
    /// 3.1
    ///
    NeedGroup,
    /// 3.1
    ///
    NotFound1,
    /// 3.1
    ///
    NotFound2,
    /// 3.2
    ///
    NotFound3,
    /// All players on party must be from the same realm.
    ///
    RealmOnly,
    /// Map can't be entered at this time.
    ///
    MapNotAllowed,
}

impl TransferAbortReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::ErrorX => 0x1,
            Self::MaxPlayers => 0x2,
            Self::NotFound => 0x3,
            Self::TooManyInstances => 0x4,
            Self::ZoneInCombat => 0x6,
            Self::InsufficientExpansionLevel => 0x7,
            Self::DifficultyNotAvailable => 0x8,
            Self::UniqueMessage => 0x9,
            Self::TooManyRealmInstances => 0xa,
            Self::NeedGroup => 0xb,
            Self::NotFound1 => 0xc,
            Self::NotFound2 => 0xd,
            Self::NotFound3 => 0xe,
            Self::RealmOnly => 0xf,
            Self::MapNotAllowed => 0x10,
        }
    }

}

impl Default for TransferAbortReason {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for TransferAbortReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::ErrorX => f.write_str("ErrorX"),
            Self::MaxPlayers => f.write_str("MaxPlayers"),
            Self::NotFound => f.write_str("NotFound"),
            Self::TooManyInstances => f.write_str("TooManyInstances"),
            Self::ZoneInCombat => f.write_str("ZoneInCombat"),
            Self::InsufficientExpansionLevel => f.write_str("InsufficientExpansionLevel"),
            Self::DifficultyNotAvailable => f.write_str("DifficultyNotAvailable"),
            Self::UniqueMessage => f.write_str("UniqueMessage"),
            Self::TooManyRealmInstances => f.write_str("TooManyRealmInstances"),
            Self::NeedGroup => f.write_str("NeedGroup"),
            Self::NotFound1 => f.write_str("NotFound1"),
            Self::NotFound2 => f.write_str("NotFound2"),
            Self::NotFound3 => f.write_str("NotFound3"),
            Self::RealmOnly => f.write_str("RealmOnly"),
            Self::MapNotAllowed => f.write_str("MapNotAllowed"),
        }
    }
}

impl TryFrom<u8> for TransferAbortReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::ErrorX),
            2 => Ok(Self::MaxPlayers),
            3 => Ok(Self::NotFound),
            4 => Ok(Self::TooManyInstances),
            6 => Ok(Self::ZoneInCombat),
            7 => Ok(Self::InsufficientExpansionLevel),
            8 => Ok(Self::DifficultyNotAvailable),
            9 => Ok(Self::UniqueMessage),
            10 => Ok(Self::TooManyRealmInstances),
            11 => Ok(Self::NeedGroup),
            12 => Ok(Self::NotFound1),
            13 => Ok(Self::NotFound2),
            14 => Ok(Self::NotFound3),
            15 => Ok(Self::RealmOnly),
            16 => Ok(Self::MapNotAllowed),
            v => Err(crate::errors::EnumError::new("TransferAbortReason", v as u64),)
        }
    }
}

