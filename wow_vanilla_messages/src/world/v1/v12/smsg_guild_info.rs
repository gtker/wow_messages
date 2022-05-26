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
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
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

        Ok(w)
    }
}

impl ServerMessage for SMSG_GUILD_INFO {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
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

        Ok(w)
    }
    const OPCODE: u16 = 0x0088;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GUILD_INFOError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guild_name: CString
            let guild_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let guild_name = String::from_utf8(guild_name)?;

            // created_day: u32
            let created_day = crate::util::tokio_read_u32_le(r).await?;

            // created_month: u32
            let created_month = crate::util::tokio_read_u32_le(r).await?;

            // created_year: u32
            let created_year = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_characters_in_guild: u32
            let amount_of_characters_in_guild = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_accounts_in_guild: u32
            let amount_of_accounts_in_guild = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                guild_name,
                created_day,
                created_month,
                created_year,
                amount_of_characters_in_guild,
                amount_of_accounts_in_guild,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // guild_name: CString
            let guild_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let guild_name = String::from_utf8(guild_name)?;

            // created_day: u32
            let created_day = crate::util::astd_read_u32_le(r).await?;

            // created_month: u32
            let created_month = crate::util::astd_read_u32_le(r).await?;

            // created_year: u32
            let created_year = crate::util::astd_read_u32_le(r).await?;

            // amount_of_characters_in_guild: u32
            let amount_of_characters_in_guild = crate::util::astd_read_u32_le(r).await?;

            // amount_of_accounts_in_guild: u32
            let amount_of_accounts_in_guild = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                guild_name,
                created_day,
                created_month,
                created_year,
                amount_of_characters_in_guild,
                amount_of_accounts_in_guild,
            })
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

