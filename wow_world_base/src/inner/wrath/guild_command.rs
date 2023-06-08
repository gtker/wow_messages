/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L16):
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
        match value {
            0 => Ok(Self::Create),
            1 => Ok(Self::Invite),
            2 => Ok(Self::Quit),
            3 => Ok(Self::Promote),
            12 => Ok(Self::Founder),
            13 => Ok(Self::Member),
            19 => Ok(Self::PublicNoteChanged),
            20 => Ok(Self::OfficerNoteChanged),
            v => Err(crate::errors::EnumError::new("GuildCommand", v as u64),)
        }
    }
}

