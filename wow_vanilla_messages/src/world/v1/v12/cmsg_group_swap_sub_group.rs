use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_swap_sub_group.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_swap_sub_group.wowm#L3):
/// ```text
/// cmsg CMSG_GROUP_SWAP_SUB_GROUP = 0x280 {
///     CString name;
///     CString swap_with_name;
/// }
/// ```
pub struct CMSG_GROUP_SWAP_SUB_GROUP {
    pub name: String,
    pub swap_with_name: String,
}

impl WorldClientMessageWrite for CMSG_GROUP_SWAP_SUB_GROUP {
    const OPCODE: u32 = 0x280;

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
impl WorldMessageBody for CMSG_GROUP_SWAP_SUB_GROUP {
    type Error = CMSG_GROUP_SWAP_SUB_GROUPError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // swap_with_name: CString
        let swap_with_name = crate::util::read_c_string_to_vec(r)?;
        let swap_with_name = String::from_utf8(swap_with_name)?;

        Ok(Self {
            name,
            swap_with_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // swap_with_name: CString
        w.write_all(self.swap_with_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GROUP_SWAP_SUB_GROUP {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
        + self.swap_with_name.len() + 1 // swap_with_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GROUP_SWAP_SUB_GROUP {
    fn maximum_possible_size() -> usize {
        256 // name: CString
        + 256 // swap_with_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GROUP_SWAP_SUB_GROUPError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GROUP_SWAP_SUB_GROUPError {}
impl std::fmt::Display for CMSG_GROUP_SWAP_SUB_GROUPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GROUP_SWAP_SUB_GROUPError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GROUP_SWAP_SUB_GROUPError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

