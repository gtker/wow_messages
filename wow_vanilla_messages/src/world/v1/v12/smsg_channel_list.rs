use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::ChannelMember;
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
pub struct SMSG_CHANNEL_LIST {
    pub channel_name: String,
    pub channel_flags: u8,
    pub members: Vec<ChannelMember>,
}

impl ServerMessageWrite for SMSG_CHANNEL_LIST {}

impl MessageBody for SMSG_CHANNEL_LIST {
    const OPCODE: u16 = 0x009b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CHANNEL_LISTError;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_CHANNEL_LIST {
    fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString and Null Terminator
        + 1 // channel_flags: u8
        + 4 // amount_of_members: u32
        + self.members.iter().fold(0, |acc, x| acc + ChannelMember::size()) // members: ChannelMember[amount_of_members]
    }
}

impl MaximumPossibleSized for SMSG_CHANNEL_LIST {
    fn maximum_possible_size() -> usize {
        256 // channel_name: CString
        + 1 // channel_flags: u8
        + 4 // amount_of_members: u32
        + 4294967295 * ChannelMember::maximum_possible_size() // members: ChannelMember[amount_of_members]
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

