/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L14):
/// ```text
/// enum GuildCommand : u8 {
///     CREATE = 0x00;
///     INVITE = 0x01;
///     QUIT = 0x02;
///     PROMOTE = 0x03;
///     FOUNDER = 0x0C;
///     MEMBER = 0x0D;
///     PUBLIC_NOTE_CHANGED = 0x13;
///     OFFICER_NOTE_CHANGED = 0x14;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GuildCommand {
    Create,
    Invite,
    Quit,
    Promote,
    Founder,
    Member,
    PublicNoteChanged,
    OfficerNoteChanged,
}

impl GuildCommand {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Create => 0x0,
            Self::Invite => 0x1,
            Self::Quit => 0x2,
            Self::Promote => 0x3,
            Self::Founder => 0xc,
            Self::Member => 0xd,
            Self::PublicNoteChanged => 0x13,
            Self::OfficerNoteChanged => 0x14,
        }
    }

    pub const fn variants() -> [Self; 8] {
        [
            Self::Create,
            Self::Invite,
            Self::Quit,
            Self::Promote,
            Self::Founder,
            Self::Member,
            Self::PublicNoteChanged,
            Self::OfficerNoteChanged,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Create),
            1 => Ok(Self::Invite),
            2 => Ok(Self::Quit),
            3 => Ok(Self::Promote),
            12 => Ok(Self::Founder),
            13 => Ok(Self::Member),
            19 => Ok(Self::PublicNoteChanged),
            20 => Ok(Self::OfficerNoteChanged),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl GuildCommand {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Create => "CREATE",
            Self::Invite => "INVITE",
            Self::Quit => "QUIT",
            Self::Promote => "PROMOTE",
            Self::Founder => "FOUNDER",
            Self::Member => "MEMBER",
            Self::PublicNoteChanged => "PUBLIC_NOTE_CHANGED",
            Self::OfficerNoteChanged => "OFFICER_NOTE_CHANGED",
        }
    }

}

const NAME: &str = "GuildCommand";

impl Default for GuildCommand {
    fn default() -> Self {
        Self::Create
    }
}

impl std::fmt::Display for GuildCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create => f.write_str("Create"),
            Self::Invite => f.write_str("Invite"),
            Self::Quit => f.write_str("Quit"),
            Self::Promote => f.write_str("Promote"),
            Self::Founder => f.write_str("Founder"),
            Self::Member => f.write_str("Member"),
            Self::PublicNoteChanged => f.write_str("PublicNoteChanged"),
            Self::OfficerNoteChanged => f.write_str("OfficerNoteChanged"),
        }
    }
}

impl TryFrom<u8> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

