use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:571`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L571):
/// ```text
/// cmsg CMSG_GUILD_SET_OFFICER_NOTE = 0x235 {
///     CString player_name;
///     CString note;
/// }
/// ```
pub struct CMSG_GUILD_SET_OFFICER_NOTE {
    pub player_name: String,
    pub note: String,
}

impl WorldClientMessageWrite for CMSG_GUILD_SET_OFFICER_NOTE {
    const OPCODE: u32 = 0x235;

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
impl WorldMessageBody for CMSG_GUILD_SET_OFFICER_NOTE {
    type Error = CMSG_GUILD_SET_OFFICER_NOTEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        // note: CString
        let note = crate::util::read_c_string_to_vec(r)?;
        let note = String::from_utf8(note)?;

        Ok(Self {
            player_name,
            note,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // note: CString
        w.write_all(self.note.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GUILD_SET_OFFICER_NOTE {
    fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString and Null Terminator
        + self.note.len() + 1 // note: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GUILD_SET_OFFICER_NOTE {
    fn maximum_possible_size() -> usize {
        256 // player_name: CString
        + 256 // note: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_SET_OFFICER_NOTEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_SET_OFFICER_NOTEError {}
impl std::fmt::Display for CMSG_GUILD_SET_OFFICER_NOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_SET_OFFICER_NOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_SET_OFFICER_NOTEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

