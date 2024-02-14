/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_event.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_event.wowm#L1):
/// ```text
/// enum GuildEvent : u8 {
///     PROMOTION = 0;
///     DEMOTION = 1;
///     MOTD = 2;
///     JOINED = 3;
///     LEFT = 4;
///     REMOVED = 5;
///     LEADER_IS = 6;
///     LEADER_CHANGED = 7;
///     DISBANDED = 8;
///     TABARD_CHANGED = 9;
///     UNKNOWN10 = 10;
///     ROSTER_UPDATE = 11;
///     SIGNED_ON = 12;
///     SIGNED_OFF = 13;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GuildEvent {
    Promotion,
    Demotion,
    Motd,
    Joined,
    Left,
    Removed,
    LeaderIs,
    LeaderChanged,
    Disbanded,
    TabardChanged,
    Unknown10,
    RosterUpdate,
    SignedOn,
    SignedOff,
}

impl GuildEvent {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Promotion => 0x0,
            Self::Demotion => 0x1,
            Self::Motd => 0x2,
            Self::Joined => 0x3,
            Self::Left => 0x4,
            Self::Removed => 0x5,
            Self::LeaderIs => 0x6,
            Self::LeaderChanged => 0x7,
            Self::Disbanded => 0x8,
            Self::TabardChanged => 0x9,
            Self::Unknown10 => 0xa,
            Self::RosterUpdate => 0xb,
            Self::SignedOn => 0xc,
            Self::SignedOff => 0xd,
        }
    }

    pub const fn variants() -> [Self; 14] {
        [
            Self::Promotion,
            Self::Demotion,
            Self::Motd,
            Self::Joined,
            Self::Left,
            Self::Removed,
            Self::LeaderIs,
            Self::LeaderChanged,
            Self::Disbanded,
            Self::TabardChanged,
            Self::Unknown10,
            Self::RosterUpdate,
            Self::SignedOn,
            Self::SignedOff,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Promotion),
            1 => Ok(Self::Demotion),
            2 => Ok(Self::Motd),
            3 => Ok(Self::Joined),
            4 => Ok(Self::Left),
            5 => Ok(Self::Removed),
            6 => Ok(Self::LeaderIs),
            7 => Ok(Self::LeaderChanged),
            8 => Ok(Self::Disbanded),
            9 => Ok(Self::TabardChanged),
            10 => Ok(Self::Unknown10),
            11 => Ok(Self::RosterUpdate),
            12 => Ok(Self::SignedOn),
            13 => Ok(Self::SignedOff),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl GuildEvent {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Promotion => "PROMOTION",
            Self::Demotion => "DEMOTION",
            Self::Motd => "MOTD",
            Self::Joined => "JOINED",
            Self::Left => "LEFT",
            Self::Removed => "REMOVED",
            Self::LeaderIs => "LEADER_IS",
            Self::LeaderChanged => "LEADER_CHANGED",
            Self::Disbanded => "DISBANDED",
            Self::TabardChanged => "TABARD_CHANGED",
            Self::Unknown10 => "UNKNOWN10",
            Self::RosterUpdate => "ROSTER_UPDATE",
            Self::SignedOn => "SIGNED_ON",
            Self::SignedOff => "SIGNED_OFF",
        }
    }

}

const NAME: &str = "GuildEvent";

impl Default for GuildEvent {
    fn default() -> Self {
        Self::Promotion
    }
}

impl std::fmt::Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Promotion => f.write_str("Promotion"),
            Self::Demotion => f.write_str("Demotion"),
            Self::Motd => f.write_str("Motd"),
            Self::Joined => f.write_str("Joined"),
            Self::Left => f.write_str("Left"),
            Self::Removed => f.write_str("Removed"),
            Self::LeaderIs => f.write_str("LeaderIs"),
            Self::LeaderChanged => f.write_str("LeaderChanged"),
            Self::Disbanded => f.write_str("Disbanded"),
            Self::TabardChanged => f.write_str("TabardChanged"),
            Self::Unknown10 => f.write_str("Unknown10"),
            Self::RosterUpdate => f.write_str("RosterUpdate"),
            Self::SignedOn => f.write_str("SignedOn"),
            Self::SignedOff => f.write_str("SignedOff"),
        }
    }
}

impl TryFrom<u8> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

