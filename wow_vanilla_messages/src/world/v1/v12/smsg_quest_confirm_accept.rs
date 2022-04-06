use std::convert::{TryFrom, TryInto};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_confirm_accept.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_confirm_accept.wowm#L3):
/// ```text
/// smsg SMSG_QUEST_CONFIRM_ACCEPT = 0x19C {
///     u32 quest_id;
///     CString quest_title;
///     u64 guid;
/// }
/// ```
pub struct SMSG_QUEST_CONFIRM_ACCEPT {
    pub quest_id: u32,
    pub quest_title: String,
    pub guid: u64,
}

impl WorldServerMessageWrite for SMSG_QUEST_CONFIRM_ACCEPT {
    const OPCODE: u16 = 0x19c;

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
impl WorldMessageBody for SMSG_QUEST_CONFIRM_ACCEPT {
    type Error = SMSG_QUEST_CONFIRM_ACCEPTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_title: CString
        let quest_title = crate::util::read_c_string_to_vec(r)?;
        let quest_title = String::from_utf8(quest_title)?;

        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_title: CString
        w.write_all(self.quest_title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_QUEST_CONFIRM_ACCEPT {
    fn size(&self) -> usize {
        4 // quest_id: u32
        + self.quest_title.len() + 1 // quest_title: CString and Null Terminator
        + 8 // guid: u64
    }
}

impl MaximumPossibleSized for SMSG_QUEST_CONFIRM_ACCEPT {
    fn maximum_possible_size() -> usize {
        4 // quest_id: u32
        + 256 // quest_title: CString
        + 8 // guid: u64
    }
}

#[derive(Debug)]
pub enum SMSG_QUEST_CONFIRM_ACCEPTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUEST_CONFIRM_ACCEPTError {}
impl std::fmt::Display for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUEST_CONFIRM_ACCEPTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

