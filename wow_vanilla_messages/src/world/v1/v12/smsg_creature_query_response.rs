use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CREATURE_QUERY_RESPONSE {
    pub creature_entry: u32,
    pub found: Option<SMSG_CREATURE_QUERY_RESPONSEfound>,
}

impl SMSG_CREATURE_QUERY_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
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

        Ok(w)
    }
}

impl ServerMessage for SMSG_CREATURE_QUERY_RESPONSE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
    const OPCODE: u16 = 0x0061;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CREATURE_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature_entry: u32
        let creature_entry = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            0
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

            Some(SMSG_CREATURE_QUERY_RESPONSEfound {
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

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // creature_entry: u32
            let creature_entry = crate::util::tokio_read_u32_le(r).await?;

            // optional found
            let current_size = {
                0
                + 4 // creature_entry: u32
            };
            let found = if current_size < body_size as usize {
                // name1: CString
                let name1 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name1 = String::from_utf8(name1)?;

                // name2: CString
                let name2 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name2 = String::from_utf8(name2)?;

                // name3: CString
                let name3 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name3 = String::from_utf8(name3)?;

                // name4: CString
                let name4 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name4 = String::from_utf8(name4)?;

                // sub_name: CString
                let sub_name = crate::util::tokio_read_c_string_to_vec(r).await?;
                let sub_name = String::from_utf8(sub_name)?;

                // type_flags: u32
                let type_flags = crate::util::tokio_read_u32_le(r).await?;

                // creature_type: u32
                let creature_type = crate::util::tokio_read_u32_le(r).await?;

                // creature_family: u32
                let creature_family = crate::util::tokio_read_u32_le(r).await?;

                // creature_rank: u32
                let creature_rank = crate::util::tokio_read_u32_le(r).await?;

                // unknown0: u32
                let unknown0 = crate::util::tokio_read_u32_le(r).await?;

                // spell_data_id: u32
                let spell_data_id = crate::util::tokio_read_u32_le(r).await?;

                // display_id: u32
                let display_id = crate::util::tokio_read_u32_le(r).await?;

                // civilian: u8
                let civilian = crate::util::tokio_read_u8_le(r).await?;

                // racial_leader: u8
                let racial_leader = crate::util::tokio_read_u8_le(r).await?;

                Some(SMSG_CREATURE_QUERY_RESPONSEfound {
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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // creature_entry: u32
            let creature_entry = crate::util::astd_read_u32_le(r).await?;

            // optional found
            let current_size = {
                0
                + 4 // creature_entry: u32
            };
            let found = if current_size < body_size as usize {
                // name1: CString
                let name1 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name1 = String::from_utf8(name1)?;

                // name2: CString
                let name2 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name2 = String::from_utf8(name2)?;

                // name3: CString
                let name3 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name3 = String::from_utf8(name3)?;

                // name4: CString
                let name4 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name4 = String::from_utf8(name4)?;

                // sub_name: CString
                let sub_name = crate::util::astd_read_c_string_to_vec(r).await?;
                let sub_name = String::from_utf8(sub_name)?;

                // type_flags: u32
                let type_flags = crate::util::astd_read_u32_le(r).await?;

                // creature_type: u32
                let creature_type = crate::util::astd_read_u32_le(r).await?;

                // creature_family: u32
                let creature_family = crate::util::astd_read_u32_le(r).await?;

                // creature_rank: u32
                let creature_rank = crate::util::astd_read_u32_le(r).await?;

                // unknown0: u32
                let unknown0 = crate::util::astd_read_u32_le(r).await?;

                // spell_data_id: u32
                let spell_data_id = crate::util::astd_read_u32_le(r).await?;

                // display_id: u32
                let display_id = crate::util::astd_read_u32_le(r).await?;

                // civilian: u8
                let civilian = crate::util::astd_read_u8_le(r).await?;

                // racial_leader: u8
                let racial_leader = crate::util::astd_read_u8_le(r).await?;

                Some(SMSG_CREATURE_QUERY_RESPONSEfound {
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
        })
    }

}

impl SMSG_CREATURE_QUERY_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + 4 // creature_entry: u32
        + if let Some(found) = &self.found {
            0
            + found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + found.sub_name.len() + 1 // sub_name: CString
            + 4 // type_flags: u32
            + 4 // creature_type: u32
            + 4 // creature_family: u32
            + 4 // creature_rank: u32
            + 4 // unknown0: u32
            + 4 // spell_data_id: u32
            + 4 // display_id: u32
            + 1 // civilian: u8
            + 1 // racial_leader: u8
        } else {
            0
        }
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
pub struct SMSG_CREATURE_QUERY_RESPONSEfound {
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

impl SMSG_CREATURE_QUERY_RESPONSEfound {
    pub(crate) fn size(&self) -> usize {
        self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + self.sub_name.len() + 1 // sub_name: CString
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

