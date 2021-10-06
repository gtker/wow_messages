use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ChatNotify, ChatNotifyError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:668`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// smsg SMSG_CHANNEL_NOTIFY = 0x99 {
///     ChatNotify notify_type;
///     CString channel_name;
/// }
/// ```
pub struct SMSG_CHANNEL_NOTIFY {
    pub notify_type: ChatNotify,
    pub channel_name: String,
}

impl WorldServerMessageWrite for SMSG_CHANNEL_NOTIFY {
    const OPCODE: u16 = 0x99;

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
impl WorldMessageBody for SMSG_CHANNEL_NOTIFY {
    type Error = SMSG_CHANNEL_NOTIFYError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // notify_type: ChatNotify
        let notify_type = ChatNotify::read(r)?;

        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            notify_type,
            channel_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // notify_type: ChatNotify
        self.notify_type.write(w)?;

        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_CHANNEL_NOTIFY {
    fn size(&self) -> usize {
        ChatNotify::size() // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_CHANNEL_NOTIFY {
    fn maximum_possible_size() -> usize {
        ChatNotify::maximum_possible_size() // notify_type: ChatNotify
        + 256 // channel_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_CHANNEL_NOTIFYError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    ChatNotify(ChatNotifyError),
}

impl std::error::Error for SMSG_CHANNEL_NOTIFYError {}
impl std::fmt::Display for SMSG_CHANNEL_NOTIFYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::ChatNotify(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHANNEL_NOTIFYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_CHANNEL_NOTIFYError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ChatNotifyError> for SMSG_CHANNEL_NOTIFYError {
    fn from(e: ChatNotifyError) -> Self {
        Self::ChatNotify(e)
    }
}

