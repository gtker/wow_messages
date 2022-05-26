use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_INFO {
    pub guild_name: String,
    pub created_day: u32,
    pub created_month: u32,
    pub created_year: u32,
    pub amount_of_characters_in_guild: u32,
    pub amount_of_accounts_in_guild: u32,
}

impl SMSG_GUILD_INFO {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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

impl ServerMessage for SMSG_GUILD_INFO {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
    const OPCODE: u16 = 0x0088;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

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

}

impl SMSG_GUILD_INFO {
    pub fn size(&self) -> usize {
        0
        + self.guild_name.len() + 1 // guild_name: CString
        + 4 // created_day: u32
        + 4 // created_month: u32
        + 4 // created_year: u32
        + 4 // amount_of_characters_in_guild: u32
        + 4 // amount_of_accounts_in_guild: u32
    }
}

