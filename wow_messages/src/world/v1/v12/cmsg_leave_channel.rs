use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:235`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L235):
/// ```text
/// cmsg CMSG_LEAVE_CHANNEL = 0x98 {
///     CString channel_name;
/// }
/// ```
pub struct CMSG_LEAVE_CHANNEL {
    pub channel_name: String,
}

impl WorldClientMessageWrite for CMSG_LEAVE_CHANNEL {
    const OPCODE: u32 = 0x98;

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
impl WorldMessageBody for CMSG_LEAVE_CHANNEL {
    type Error = CMSG_LEAVE_CHANNELError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            channel_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_LEAVE_CHANNEL {
    fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_LEAVE_CHANNEL {
    fn maximum_possible_size() -> usize {
        256 // channel_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_LEAVE_CHANNELError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_LEAVE_CHANNELError {}
impl std::fmt::Display for CMSG_LEAVE_CHANNELError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_LEAVE_CHANNELError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_LEAVE_CHANNELError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

