use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_guild_rank.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_guild_rank.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_RANK = 0x231 {
///     u32 rank_id;
///     u32 rights;
///     CString rank_name;
/// }
/// ```
pub struct CMSG_GUILD_RANK {
    pub rank_id: u32,
    pub rights: u32,
    pub rank_name: String,
}

impl WorldClientMessageWrite for CMSG_GUILD_RANK {
    const OPCODE: u32 = 0x231;

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
impl WorldMessageBody for CMSG_GUILD_RANK {
    type Error = CMSG_GUILD_RANKError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // rank_id: u32
        let rank_id = crate::util::read_u32_le(r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(r)?;

        // rank_name: CString
        let rank_name = crate::util::read_c_string_to_vec(r)?;
        let rank_name = String::from_utf8(rank_name)?;

        Ok(Self {
            rank_id,
            rights,
            rank_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // rank_name: CString
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for CMSG_GUILD_RANK {
    fn size(&self) -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + self.rank_name.len() + 1 // rank_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_GUILD_RANK {
    fn maximum_possible_size() -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + 256 // rank_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_RANKError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_RANKError {}
impl std::fmt::Display for CMSG_GUILD_RANKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_RANKError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_RANKError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

