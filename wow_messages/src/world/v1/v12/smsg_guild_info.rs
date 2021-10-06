use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:401`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// smsg SMSG_GUILD_INFO = 0x88 {
///     CString guild_name;
///     u32 created_day;
///     u32 created_month;
///     u32 created_year;
///     u32 amount_of_characters_in_guild;
///     u32 amount_of_accounts_in_guild;
/// }
/// ```
pub struct SMSG_GUILD_INFO {
    pub guild_name: String,
    pub created_day: u32,
    pub created_month: u32,
    pub created_year: u32,
    pub amount_of_characters_in_guild: u32,
    pub amount_of_accounts_in_guild: u32,
}

impl WorldServerMessageWrite for SMSG_GUILD_INFO {
    const OPCODE: u16 = 0x88;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_GUILD_INFO {
    type Error = SMSG_GUILD_INFOError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        // created_day: u32
        let created_day = crate::util::read_u32_le(r)?;

        // created_month: u32
        let created_month = crate::util::read_u32_le(r)?;

        // created_year: u32
        let created_year = crate::util::read_u32_le(r)?;

        // amount_of_characters_in_guild: u32
        let amount_of_characters_in_guild = crate::util::read_u32_le(r)?;

        // amount_of_accounts_in_guild: u32
        let amount_of_accounts_in_guild = crate::util::read_u32_le(r)?;

        Ok(Self {
            guild_name,
            created_day,
            created_month,
            created_year,
            amount_of_characters_in_guild,
            amount_of_accounts_in_guild,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // created_day: u32
        w.write_all(&self.created_day.to_le_bytes())?;

        // created_month: u32
        w.write_all(&self.created_month.to_le_bytes())?;

        // created_year: u32
        w.write_all(&self.created_year.to_le_bytes())?;

        // amount_of_characters_in_guild: u32
        w.write_all(&self.amount_of_characters_in_guild.to_le_bytes())?;

        // amount_of_accounts_in_guild: u32
        w.write_all(&self.amount_of_accounts_in_guild.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_GUILD_INFO {
    fn size(&self) -> usize {
        self.guild_name.len() + 1 // guild_name: CString and Null Terminator
        + 4 // created_day: u32
        + 4 // created_month: u32
        + 4 // created_year: u32
        + 4 // amount_of_characters_in_guild: u32
        + 4 // amount_of_accounts_in_guild: u32
    }
}

impl MaximumPossibleSized for SMSG_GUILD_INFO {
    fn maximum_possible_size() -> usize {
        256 // guild_name: CString
        + 4 // created_day: u32
        + 4 // created_month: u32
        + 4 // created_year: u32
        + 4 // amount_of_characters_in_guild: u32
        + 4 // amount_of_accounts_in_guild: u32
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_INFOError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GUILD_INFOError {}
impl std::fmt::Display for SMSG_GUILD_INFOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_INFOError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_INFOError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

