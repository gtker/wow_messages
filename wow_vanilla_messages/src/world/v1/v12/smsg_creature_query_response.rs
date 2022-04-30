use std::convert::{TryFrom, TryInto};
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
pub struct SMSG_CREATURE_QUERY_RESPONSE {
    pub creature_entry: u32,
    pub found: Option<SMSG_CREATURE_QUERY_RESPONSE_found>,
}

impl WorldServerMessageWrite for SMSG_CREATURE_QUERY_RESPONSE {
    const OPCODE: u16 = 0x61;

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
impl MessageBody for SMSG_CREATURE_QUERY_RESPONSE {
    type Error = SMSG_CREATURE_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature_entry: u32
        let creature_entry = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
            + 4 // creature_entry: u32
        };
        let found = if current_size < body_size as usize {
            // name1: CString
            let name1 = crate::util::read_c_string_to_vec(r)?;
            let name1 = String::from_utf8(name1)?;

            // name2: CString
            let name2 = crate::util::read_c_string_to_vec(r)?;
            let name2 = String::from_utf8(name2)?;

            // name3: CString
            let name3 = crate::util::read_c_string_to_vec(r)?;
            let name3 = String::from_utf8(name3)?;

            // name4: CString
            let name4 = crate::util::read_c_string_to_vec(r)?;
            let name4 = String::from_utf8(name4)?;

            // sub_name: CString
            let sub_name = crate::util::read_c_string_to_vec(r)?;
            let sub_name = String::from_utf8(sub_name)?;

            // type_flags: u32
            let type_flags = crate::util::read_u32_le(r)?;

            // creature_type: u32
            let creature_type = crate::util::read_u32_le(r)?;

            // creature_family: u32
            let creature_family = crate::util::read_u32_le(r)?;

            // creature_rank: u32
            let creature_rank = crate::util::read_u32_le(r)?;

            // unknown0: u32
            let unknown0 = crate::util::read_u32_le(r)?;

            // spell_data_id: u32
            let spell_data_id = crate::util::read_u32_le(r)?;

            // display_id: u32
            let display_id = crate::util::read_u32_le(r)?;

            // civilian: u8
            let civilian = crate::util::read_u8_le(r)?;

            // racial_leader: u8
            let racial_leader = crate::util::read_u8_le(r)?;

            Some(SMSG_CREATURE_QUERY_RESPONSE_found {
                name1,
                name2,
                name3,
                name4,
                sub_name,
                type_flags,
                creature_type,
                creature_family,
                creature_rank,
                unknown0,
                spell_data_id,
                display_id,
                civilian,
                racial_leader,
            })
        } else {
            None
        };

        Ok(Self {
            creature_entry,
            found,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature_entry: u32
        w.write_all(&self.creature_entry.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // name1: CString
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // sub_name: CString
            w.write_all(v.sub_name.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // type_flags: u32
            w.write_all(&v.type_flags.to_le_bytes())?;

            // creature_type: u32
            w.write_all(&v.creature_type.to_le_bytes())?;

            // creature_family: u32
            w.write_all(&v.creature_family.to_le_bytes())?;

            // creature_rank: u32
            w.write_all(&v.creature_rank.to_le_bytes())?;

            // unknown0: u32
            w.write_all(&v.unknown0.to_le_bytes())?;

            // spell_data_id: u32
            w.write_all(&v.spell_data_id.to_le_bytes())?;

            // display_id: u32
            w.write_all(&v.display_id.to_le_bytes())?;

            // civilian: u8
            w.write_all(&v.civilian.to_le_bytes())?;

            // racial_leader: u8
            w.write_all(&v.racial_leader.to_le_bytes())?;

        }

        Ok(())
    }
}

impl VariableSized for SMSG_CREATURE_QUERY_RESPONSE {
    fn size(&self) -> usize {
        4 // creature_entry: u32
        + {
            if let Some(v) = &self.found {
                v.size()
            } else {
                0
            }
        } // optional found
    }
}

impl MaximumPossibleSized for SMSG_CREATURE_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // creature_entry: u32
        + 65536 // optional found
    }
}

#[derive(Debug)]
pub enum SMSG_CREATURE_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_CREATURE_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_CREATURE_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CREATURE_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_CREATURE_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_CREATURE_QUERY_RESPONSE_found {
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub sub_name: String,
    pub type_flags: u32,
    pub creature_type: u32,
    pub creature_family: u32,
    pub creature_rank: u32,
    pub unknown0: u32,
    pub spell_data_id: u32,
    pub display_id: u32,
    pub civilian: u8,
    pub racial_leader: u8,
}

impl SMSG_CREATURE_QUERY_RESPONSE_found {
    pub fn size(&self) -> usize {
        self.name1.len() + 1 // name1: CString and Null Terminator
        + self.name2.len() + 1 // name2: CString and Null Terminator
        + self.name3.len() + 1 // name3: CString and Null Terminator
        + self.name4.len() + 1 // name4: CString and Null Terminator
        + self.sub_name.len() + 1 // sub_name: CString and Null Terminator
        + 4 // type_flags: u32
        + 4 // creature_type: u32
        + 4 // creature_family: u32
        + 4 // creature_rank: u32
        + 4 // unknown0: u32
        + 4 // spell_data_id: u32
        + 4 // display_id: u32
        + 1 // civilian: u8
        + 1 // racial_leader: u8
    }
}

