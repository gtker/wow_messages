use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PETITION_QUERY_RESPONSE {
    pub petition_guid: Guid,
    pub charter_owner: Guid,
    pub guild_name: String,
    pub body_text: String,
    pub unknown_flags: u32,
    pub minimum_signatures: u32,
    pub maximum_signatures: u32,
    pub deadline: u32,
    pub issue_date: u32,
    pub allowed_guild_id: u32,
    pub allowed_classes: u32,
    pub allowed_races: u32,
    pub allowed_genders: u16,
    pub allowed_minimum_level: u32,
    pub allowed_maximum_level: u32,
    pub todo_amount_of_signers: u32,
    pub number_of_choices: u32,
}

impl WorldServerMessageWrite for SMSG_PETITION_QUERY_RESPONSE {
    const OPCODE: u16 = 0x1c7;

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
impl MessageBody for SMSG_PETITION_QUERY_RESPONSE {
    type Error = SMSG_PETITION_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // charter_owner: Guid
        let charter_owner = Guid::read(r)?;

        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        // body_text: CString
        let body_text = crate::util::read_c_string_to_vec(r)?;
        let body_text = String::from_utf8(body_text)?;

        // unknown_flags: u32
        let unknown_flags = crate::util::read_u32_le(r)?;

        // minimum_signatures: u32
        let minimum_signatures = crate::util::read_u32_le(r)?;

        // maximum_signatures: u32
        let maximum_signatures = crate::util::read_u32_le(r)?;

        // deadline: u32
        let deadline = crate::util::read_u32_le(r)?;

        // issue_date: u32
        let issue_date = crate::util::read_u32_le(r)?;

        // allowed_guild_id: u32
        let allowed_guild_id = crate::util::read_u32_le(r)?;

        // allowed_classes: u32
        let allowed_classes = crate::util::read_u32_le(r)?;

        // allowed_races: u32
        let allowed_races = crate::util::read_u32_le(r)?;

        // allowed_genders: u16
        let allowed_genders = crate::util::read_u16_le(r)?;

        // allowed_minimum_level: u32
        let allowed_minimum_level = crate::util::read_u32_le(r)?;

        // allowed_maximum_level: u32
        let allowed_maximum_level = crate::util::read_u32_le(r)?;

        // todo_amount_of_signers: u32
        let todo_amount_of_signers = crate::util::read_u32_le(r)?;

        // number_of_choices: u32
        let number_of_choices = crate::util::read_u32_le(r)?;

        Ok(Self {
            petition_guid,
            charter_owner,
            guild_name,
            body_text,
            unknown_flags,
            minimum_signatures,
            maximum_signatures,
            deadline,
            issue_date,
            allowed_guild_id,
            allowed_classes,
            allowed_races,
            allowed_genders,
            allowed_minimum_level,
            allowed_maximum_level,
            todo_amount_of_signers,
            number_of_choices,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // charter_owner: Guid
        self.charter_owner.write(w)?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body_text: CString
        w.write_all(self.body_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown_flags: u32
        w.write_all(&self.unknown_flags.to_le_bytes())?;

        // minimum_signatures: u32
        w.write_all(&self.minimum_signatures.to_le_bytes())?;

        // maximum_signatures: u32
        w.write_all(&self.maximum_signatures.to_le_bytes())?;

        // deadline: u32
        w.write_all(&self.deadline.to_le_bytes())?;

        // issue_date: u32
        w.write_all(&self.issue_date.to_le_bytes())?;

        // allowed_guild_id: u32
        w.write_all(&self.allowed_guild_id.to_le_bytes())?;

        // allowed_classes: u32
        w.write_all(&self.allowed_classes.to_le_bytes())?;

        // allowed_races: u32
        w.write_all(&self.allowed_races.to_le_bytes())?;

        // allowed_genders: u16
        w.write_all(&self.allowed_genders.to_le_bytes())?;

        // allowed_minimum_level: u32
        w.write_all(&self.allowed_minimum_level.to_le_bytes())?;

        // allowed_maximum_level: u32
        w.write_all(&self.allowed_maximum_level.to_le_bytes())?;

        // todo_amount_of_signers: u32
        w.write_all(&self.todo_amount_of_signers.to_le_bytes())?;

        // number_of_choices: u32
        w.write_all(&self.number_of_choices.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_PETITION_QUERY_RESPONSE {
    fn size(&self) -> usize {
        8 // petition_guid: Guid
        + 8 // charter_owner: Guid
        + self.guild_name.len() + 1 // guild_name: CString and Null Terminator
        + self.body_text.len() + 1 // body_text: CString and Null Terminator
        + 4 // unknown_flags: u32
        + 4 // minimum_signatures: u32
        + 4 // maximum_signatures: u32
        + 4 // deadline: u32
        + 4 // issue_date: u32
        + 4 // allowed_guild_id: u32
        + 4 // allowed_classes: u32
        + 4 // allowed_races: u32
        + 2 // allowed_genders: u16
        + 4 // allowed_minimum_level: u32
        + 4 // allowed_maximum_level: u32
        + 4 // todo_amount_of_signers: u32
        + 4 // number_of_choices: u32
    }
}

impl MaximumPossibleSized for SMSG_PETITION_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        8 // petition_guid: Guid
        + 8 // charter_owner: Guid
        + 256 // guild_name: CString
        + 256 // body_text: CString
        + 4 // unknown_flags: u32
        + 4 // minimum_signatures: u32
        + 4 // maximum_signatures: u32
        + 4 // deadline: u32
        + 4 // issue_date: u32
        + 4 // allowed_guild_id: u32
        + 4 // allowed_classes: u32
        + 4 // allowed_races: u32
        + 2 // allowed_genders: u16
        + 4 // allowed_minimum_level: u32
        + 4 // allowed_maximum_level: u32
        + 4 // todo_amount_of_signers: u32
        + 4 // number_of_choices: u32
    }
}

#[derive(Debug)]
pub enum SMSG_PETITION_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_PETITION_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_PETITION_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PETITION_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PETITION_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

