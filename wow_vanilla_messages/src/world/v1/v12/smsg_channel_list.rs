use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ChannelMember;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CHANNEL_LIST {
    pub channel_name: String,
    pub channel_flags: u8,
    pub members: Vec<ChannelMember>,
}

impl SMSG_CHANNEL_LIST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_flags: u8
        w.write_all(&self.channel_flags.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: ChannelMember[amount_of_members]
        for i in self.members.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_CHANNEL_LIST {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_flags: u8
        w.write_all(&self.channel_flags.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&(self.members.len() as u32).to_le_bytes())?;

        // members: ChannelMember[amount_of_members]
        for i in self.members.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x009b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CHANNEL_LISTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        // channel_flags: u8
        let channel_flags = crate::util::read_u8_le(r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(r)?;

        // members: ChannelMember[amount_of_members]
        let mut members = Vec::with_capacity(amount_of_members as usize);
        for i in 0..amount_of_members {
            members.push(ChannelMember::read(r)?);
        }

        Ok(Self {
            channel_name,
            channel_flags,
            members,
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
            // channel_name: CString
            let channel_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let channel_name = String::from_utf8(channel_name)?;

            // channel_flags: u8
            let channel_flags = crate::util::tokio_read_u8_le(r).await?;

            // amount_of_members: u32
            let amount_of_members = crate::util::tokio_read_u32_le(r).await?;

            // members: ChannelMember[amount_of_members]
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(ChannelMember::tokio_read(r).await?);
            }

            Ok(Self {
                channel_name,
                channel_flags,
                members,
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
            // channel_name: CString
            let channel_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let channel_name = String::from_utf8(channel_name)?;

            // channel_flags: u8
            let channel_flags = crate::util::astd_read_u8_le(r).await?;

            // amount_of_members: u32
            let amount_of_members = crate::util::astd_read_u32_le(r).await?;

            // members: ChannelMember[amount_of_members]
            let mut members = Vec::with_capacity(amount_of_members as usize);
            for i in 0..amount_of_members {
                members.push(ChannelMember::astd_read(r).await?);
            }

            Ok(Self {
                channel_name,
                channel_flags,
                members,
            })
        })
    }

}

impl SMSG_CHANNEL_LIST {
    pub fn size(&self) -> usize {
        0
        + self.channel_name.len() + 1 // channel_name: CString
        + 1 // channel_flags: u8
        + 4 // amount_of_members: u32
        + self.members.len() * 9 // members: ChannelMember[amount_of_members]
    }
}

#[derive(Debug)]
pub enum SMSG_CHANNEL_LISTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_CHANNEL_LISTError {}
impl std::fmt::Display for SMSG_CHANNEL_LISTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHANNEL_LISTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_CHANNEL_LISTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

