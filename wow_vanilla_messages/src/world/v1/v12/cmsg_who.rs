use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_WHO {
    pub minimum_level: u32,
    pub maximum_level: u32,
    pub player_name: String,
    pub guild_name: String,
    pub race_mask: u32,
    pub class_mask: u32,
    pub zones: Vec<u32>,
    pub search_strings: Vec<String>,
}

impl ClientMessageWrite for CMSG_WHO {}

impl CMSG_WHO {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // minimum_level: u32
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u32
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race_mask: u32
        w.write_all(&self.race_mask.to_le_bytes())?;

        // class_mask: u32
        w.write_all(&self.class_mask.to_le_bytes())?;

        // amount_of_zones: u32
        w.write_all(&(self.zones.len() as u32).to_le_bytes())?;

        // zones: u32[amount_of_zones]
        for i in self.zones.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_strings: u32
        w.write_all(&(self.search_strings.len() as u32).to_le_bytes())?;

        // search_strings: CString[amount_of_strings]
        for i in self.search_strings.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(w)
    }
}

impl MessageBody for CMSG_WHO {
    const OPCODE: u16 = 0x0062;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_WHOError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum_level: u32
        let minimum_level = crate::util::read_u32_le(r)?;

        // maximum_level: u32
        let maximum_level = crate::util::read_u32_le(r)?;

        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        // race_mask: u32
        let race_mask = crate::util::read_u32_le(r)?;

        // class_mask: u32
        let class_mask = crate::util::read_u32_le(r)?;

        // amount_of_zones: u32
        let amount_of_zones = crate::util::read_u32_le(r)?;

        // zones: u32[amount_of_zones]
        let mut zones = Vec::with_capacity(amount_of_zones as usize);
        for i in 0..amount_of_zones {
            zones.push(crate::util::read_u32_le(r)?);
        }

        // amount_of_strings: u32
        let amount_of_strings = crate::util::read_u32_le(r)?;

        // search_strings: CString[amount_of_strings]
        let mut search_strings = Vec::with_capacity(amount_of_strings as usize);
        for i in 0..amount_of_strings {
            let s = crate::util::read_c_string_to_vec(r)?;
            search_strings.push(String::from_utf8(s)?);
        }

        Ok(Self {
            minimum_level,
            maximum_level,
            player_name,
            guild_name,
            race_mask,
            class_mask,
            zones,
            search_strings,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // minimum_level: u32
            let minimum_level = crate::util::tokio_read_u32_le(r).await?;

            // maximum_level: u32
            let maximum_level = crate::util::tokio_read_u32_le(r).await?;

            // player_name: CString
            let player_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let player_name = String::from_utf8(player_name)?;

            // guild_name: CString
            let guild_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let guild_name = String::from_utf8(guild_name)?;

            // race_mask: u32
            let race_mask = crate::util::tokio_read_u32_le(r).await?;

            // class_mask: u32
            let class_mask = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_zones: u32
            let amount_of_zones = crate::util::tokio_read_u32_le(r).await?;

            // zones: u32[amount_of_zones]
            let mut zones = Vec::with_capacity(amount_of_zones as usize);
            for i in 0..amount_of_zones {
                zones.push(crate::util::tokio_read_u32_le(r).await?);
            }

            // amount_of_strings: u32
            let amount_of_strings = crate::util::tokio_read_u32_le(r).await?;

            // search_strings: CString[amount_of_strings]
            let mut search_strings = Vec::with_capacity(amount_of_strings as usize);
            for i in 0..amount_of_strings {
                let s = crate::util::tokio_read_c_string_to_vec(r).await?;
                search_strings.push(String::from_utf8(s)?);
            }

            Ok(Self {
                minimum_level,
                maximum_level,
                player_name,
                guild_name,
                race_mask,
                class_mask,
                zones,
                search_strings,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // minimum_level: u32
            let minimum_level = crate::util::astd_read_u32_le(r).await?;

            // maximum_level: u32
            let maximum_level = crate::util::astd_read_u32_le(r).await?;

            // player_name: CString
            let player_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let player_name = String::from_utf8(player_name)?;

            // guild_name: CString
            let guild_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let guild_name = String::from_utf8(guild_name)?;

            // race_mask: u32
            let race_mask = crate::util::astd_read_u32_le(r).await?;

            // class_mask: u32
            let class_mask = crate::util::astd_read_u32_le(r).await?;

            // amount_of_zones: u32
            let amount_of_zones = crate::util::astd_read_u32_le(r).await?;

            // zones: u32[amount_of_zones]
            let mut zones = Vec::with_capacity(amount_of_zones as usize);
            for i in 0..amount_of_zones {
                zones.push(crate::util::astd_read_u32_le(r).await?);
            }

            // amount_of_strings: u32
            let amount_of_strings = crate::util::astd_read_u32_le(r).await?;

            // search_strings: CString[amount_of_strings]
            let mut search_strings = Vec::with_capacity(amount_of_strings as usize);
            for i in 0..amount_of_strings {
                let s = crate::util::astd_read_c_string_to_vec(r).await?;
                search_strings.push(String::from_utf8(s)?);
            }

            Ok(Self {
                minimum_level,
                maximum_level,
                player_name,
                guild_name,
                race_mask,
                class_mask,
                zones,
                search_strings,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl CMSG_WHO {
    pub fn size(&self) -> usize {
        0
        + 4 // minimum_level: u32
        + 4 // maximum_level: u32
        + self.player_name.len() + 1 // player_name: CString
        + self.guild_name.len() + 1 // guild_name: CString
        + 4 // race_mask: u32
        + 4 // class_mask: u32
        + 4 // amount_of_zones: u32
        + self.zones.len() * core::mem::size_of::<u32>() // zones: u32[amount_of_zones]
        + 4 // amount_of_strings: u32
        + self.search_strings.iter().fold(0, |acc, x| acc + x.len() + 1) // search_strings: CString[amount_of_strings]
    }
}

#[derive(Debug)]
pub enum CMSG_WHOError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_WHOError {}
impl std::fmt::Display for CMSG_WHOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_WHOError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_WHOError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

