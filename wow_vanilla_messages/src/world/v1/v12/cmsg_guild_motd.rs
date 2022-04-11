use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_MOTD {
    pub message_of_the_day: String,
}

impl WorldClientMessageWrite for CMSG_GUILD_MOTD {
    const OPCODE: u32 = 0x91;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_GUILD_MOTD {
    type Error = CMSG_GUILD_MOTDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_of_the_day: CString
        let message_of_the_day = crate::util::read_c_string_to_vec(r)?;
        let message_of_the_day = String::from_utf8(message_of_the_day)?;

        Ok(Self {
            message_of_the_day,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_of_the_day: CString
        w.write_all(self.message_of_the_day.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GUILD_MOTD {
    fn size(&self) -> usize {
        self.message_of_the_day.len() + 1 // message_of_the_day: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GUILD_MOTD {
    fn maximum_possible_size() -> usize {
        256 // message_of_the_day: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_MOTDError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_MOTDError {}
impl std::fmt::Display for CMSG_GUILD_MOTDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_MOTDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_MOTDError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

