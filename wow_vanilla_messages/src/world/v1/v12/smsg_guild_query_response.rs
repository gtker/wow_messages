use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_QUERY_RESPONSE {
    pub id: u32,
    pub name: String,
    pub rank_names: [String; 10],
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl ServerMessageWrite for SMSG_GUILD_QUERY_RESPONSE {}

impl MessageBody for SMSG_GUILD_QUERY_RESPONSE {
    const OPCODE: u16 = 0x0055;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GUILD_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // rank_names: CString[10]
        let mut rank_names = Vec::with_capacity(10 as usize);
        for i in 0..10 {
            let s = crate::util::read_c_string_to_vec(r)?;
            rank_names[i] = String::from_utf8(s)?;
        }
        let rank_names = rank_names.try_into().unwrap();

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(r)?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            name,
            rank_names,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
            background_color,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // rank_names: CString[10]
        for i in self.rank_names.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // emblem_style: u32
        w.write_all(&self.emblem_style.to_le_bytes())?;

        // emblem_color: u32
        w.write_all(&self.emblem_color.to_le_bytes())?;

        // border_style: u32
        w.write_all(&self.border_style.to_le_bytes())?;

        // border_color: u32
        w.write_all(&self.border_color.to_le_bytes())?;

        // background_color: u32
        w.write_all(&self.background_color.to_le_bytes())?;

        Ok(())
    }

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
            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // rank_names: CString[10]
            let mut rank_names = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                let s = crate::util::tokio_read_c_string_to_vec(r).await?;
                rank_names[i] = String::from_utf8(s)?;
            }
            let rank_names = rank_names.try_into().unwrap();

            // emblem_style: u32
            let emblem_style = crate::util::tokio_read_u32_le(r).await?;

            // emblem_color: u32
            let emblem_color = crate::util::tokio_read_u32_le(r).await?;

            // border_style: u32
            let border_style = crate::util::tokio_read_u32_le(r).await?;

            // border_color: u32
            let border_color = crate::util::tokio_read_u32_le(r).await?;

            // background_color: u32
            let background_color = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                id,
                name,
                rank_names,
                emblem_style,
                emblem_color,
                border_style,
                border_color,
                background_color,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // rank_names: CString[10]
            for i in self.rank_names.iter() {
                w.write_all(&i.as_bytes()).await?;
                w.write_all(&[0]).await?;
            }

            // emblem_style: u32
            w.write_all(&self.emblem_style.to_le_bytes()).await?;

            // emblem_color: u32
            w.write_all(&self.emblem_color.to_le_bytes()).await?;

            // border_style: u32
            w.write_all(&self.border_style.to_le_bytes()).await?;

            // border_color: u32
            w.write_all(&self.border_color.to_le_bytes()).await?;

            // background_color: u32
            w.write_all(&self.background_color.to_le_bytes()).await?;

            Ok(())
        })
    }

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
            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // rank_names: CString[10]
            let mut rank_names = Vec::with_capacity(10 as usize);
            for i in 0..10 {
                let s = crate::util::astd_read_c_string_to_vec(r).await?;
                rank_names[i] = String::from_utf8(s)?;
            }
            let rank_names = rank_names.try_into().unwrap();

            // emblem_style: u32
            let emblem_style = crate::util::astd_read_u32_le(r).await?;

            // emblem_color: u32
            let emblem_color = crate::util::astd_read_u32_le(r).await?;

            // border_style: u32
            let border_style = crate::util::astd_read_u32_le(r).await?;

            // border_color: u32
            let border_color = crate::util::astd_read_u32_le(r).await?;

            // background_color: u32
            let background_color = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                id,
                name,
                rank_names,
                emblem_style,
                emblem_color,
                border_style,
                border_color,
                background_color,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // rank_names: CString[10]
            for i in self.rank_names.iter() {
                w.write_all(&i.as_bytes()).await?;
                w.write_all(&[0]).await?;
            }

            // emblem_style: u32
            w.write_all(&self.emblem_style.to_le_bytes()).await?;

            // emblem_color: u32
            w.write_all(&self.emblem_color.to_le_bytes()).await?;

            // border_style: u32
            w.write_all(&self.border_style.to_le_bytes()).await?;

            // border_color: u32
            w.write_all(&self.border_color.to_le_bytes()).await?;

            // background_color: u32
            w.write_all(&self.background_color.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_GUILD_QUERY_RESPONSE {
    fn size(&self) -> usize {
        0
        + 4 // id: u32
        + self.name.len() + 1 // name: CString
        + self.rank_names.iter().fold(0, |acc, x| acc + x.len() + 1) // rank_names: CString[10]
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

impl MaximumPossibleSized for SMSG_GUILD_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // id: u32
        + 256 // name: CString
        + 2560 // rank_names: CString[10]
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GUILD_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_GUILD_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

