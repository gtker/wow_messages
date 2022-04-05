use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:302`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L302):
/// ```text
/// smsg SMSG_ITEM_NAME_QUERY_RESPONSE = 0x2C5 {
///     u32 item_id;
///     CString item_name;
/// }
/// ```
pub struct SMSG_ITEM_NAME_QUERY_RESPONSE {
    pub item_id: u32,
    pub item_name: String,
}

impl WorldServerMessageWrite for SMSG_ITEM_NAME_QUERY_RESPONSE {
    const OPCODE: u16 = 0x2c5;

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
impl WorldMessageBody for SMSG_ITEM_NAME_QUERY_RESPONSE {
    type Error = SMSG_ITEM_NAME_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_name: CString
        let item_name = crate::util::read_c_string_to_vec(r)?;
        let item_name = String::from_utf8(item_name)?;

        Ok(Self {
            item_id,
            item_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_name: CString
        w.write_all(self.item_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_ITEM_NAME_QUERY_RESPONSE {
    fn size(&self) -> usize {
        4 // item_id: u32
        + self.item_name.len() + 1 // item_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_ITEM_NAME_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // item_id: u32
        + 256 // item_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_NAME_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_ITEM_NAME_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_ITEM_NAME_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_NAME_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_ITEM_NAME_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

