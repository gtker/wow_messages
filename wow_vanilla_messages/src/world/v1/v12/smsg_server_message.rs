use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ServerMessageType, ServerMessageTypeError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_server_message.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_server_message.wowm#L11):
/// ```text
/// smsg SMSG_SERVER_MESSAGE = 0x291 {
///     ServerMessageType message_type;
///     CString message;
/// }
/// ```
pub struct SMSG_SERVER_MESSAGE {
    pub message_type: ServerMessageType,
    pub message: String,
}

impl WorldServerMessageWrite for SMSG_SERVER_MESSAGE {
    const OPCODE: u16 = 0x291;

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
impl WorldMessageBody for SMSG_SERVER_MESSAGE {
    type Error = SMSG_SERVER_MESSAGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: ServerMessageType
        let message_type = ServerMessageType::read(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            message_type,
            message,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: ServerMessageType
        self.message_type.write(w)?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_SERVER_MESSAGE {
    fn size(&self) -> usize {
        ServerMessageType::size() // message_type: ServerMessageType
        + self.message.len() + 1 // message: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_SERVER_MESSAGE {
    fn maximum_possible_size() -> usize {
        ServerMessageType::maximum_possible_size() // message_type: ServerMessageType
        + 256 // message: CString
    }
}

#[derive(Debug)]
pub enum SMSG_SERVER_MESSAGEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    ServerMessageType(ServerMessageTypeError),
}

impl std::error::Error for SMSG_SERVER_MESSAGEError {}
impl std::fmt::Display for SMSG_SERVER_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::ServerMessageType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SERVER_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_SERVER_MESSAGEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ServerMessageTypeError> for SMSG_SERVER_MESSAGEError {
    fn from(e: ServerMessageTypeError) -> Self {
        Self::ServerMessageType(e)
    }
}

