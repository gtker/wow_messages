use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_char_rename.wowm#L3):
/// ```text
/// cmsg CMSG_CHAR_RENAME = 0x2C7 {
///     u64 guid;
///     CString name;
/// }
/// ```
pub struct CMSG_CHAR_RENAME {
    pub guid: u64,
    pub name: String,
}

impl WorldClientMessageWrite for CMSG_CHAR_RENAME {
    const OPCODE: u32 = 0x2c7;

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
impl WorldMessageBody for CMSG_CHAR_RENAME {
    type Error = CMSG_CHAR_RENAMEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            guid,
            name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_CHAR_RENAME {
    fn size(&self) -> usize {
        8 // guid: u64
        + self.name.len() + 1 // name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_CHAR_RENAME {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 256 // name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_CHAR_RENAMEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_CHAR_RENAMEError {}
impl std::fmt::Display for CMSG_CHAR_RENAMEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CHAR_RENAMEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_CHAR_RENAMEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

