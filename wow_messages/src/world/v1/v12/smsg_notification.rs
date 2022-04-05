use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_notification.wowm#L3):
/// ```text
/// smsg SMSG_NOTIFICATION = 0x1CB {
///     CString notification;
/// }
/// ```
pub struct SMSG_NOTIFICATION {
    pub notification: String,
}

impl WorldServerMessageWrite for SMSG_NOTIFICATION {
    const OPCODE: u16 = 0x1cb;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_NOTIFICATION {
    type Error = SMSG_NOTIFICATIONError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // notification: CString
        let notification = crate::util::read_c_string_to_vec(r)?;
        let notification = String::from_utf8(notification)?;

        Ok(Self {
            notification,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // notification: CString
        w.write_all(self.notification.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_NOTIFICATION {
    fn size(&self) -> usize {
        self.notification.len() + 1 // notification: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_NOTIFICATION {
    fn maximum_possible_size() -> usize {
        256 // notification: CString
    }
}

#[derive(Debug)]
pub enum SMSG_NOTIFICATIONError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_NOTIFICATIONError {}
impl std::fmt::Display for SMSG_NOTIFICATIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NOTIFICATIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_NOTIFICATIONError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

