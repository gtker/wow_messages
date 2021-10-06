use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:200`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// cmsg CMSG_WHOIS = 0x64 {
///     CString character;
/// }
/// ```
pub struct CMSG_WHOIS {
    pub character: String,
}

impl WorldClientMessageWrite for CMSG_WHOIS {
    const OPCODE: u32 = 0x64;

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
impl WorldMessageBody for CMSG_WHOIS {
    type Error = CMSG_WHOISError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // character: CString
        let character = crate::util::read_c_string_to_vec(r)?;
        let character = String::from_utf8(character)?;

        Ok(Self {
            character,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // character: CString
        w.write_all(self.character.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_WHOIS {
    fn size(&self) -> usize {
        self.character.len() + 1 // character: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_WHOIS {
    fn maximum_possible_size() -> usize {
        256 // character: CString
    }
}

#[derive(Debug)]
pub enum CMSG_WHOISError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_WHOISError {}
impl std::fmt::Display for CMSG_WHOISError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_WHOISError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_WHOISError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

