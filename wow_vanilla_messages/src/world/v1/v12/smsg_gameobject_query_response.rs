use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE {
    pub entry_id: u32,
    pub found: Option<SMSG_GAMEOBJECT_QUERY_RESPONSE_found>,
}

impl ServerMessageWrite for SMSG_GAMEOBJECT_QUERY_RESPONSE {}

impl MessageBody for SMSG_GAMEOBJECT_QUERY_RESPONSE {
    const OPCODE: u16 = 0x005f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GAMEOBJECT_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // entry_id: u32
        let entry_id = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            0
            + 4 // entry_id: u32
        };
        let found = if current_size < body_size as usize {
            // info_type: u32
            let info_type = crate::util::read_u32_le(r)?;

            // display_id: u32
            let display_id = crate::util::read_u32_le(r)?;

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

            // name5: CString
            let name5 = crate::util::read_c_string_to_vec(r)?;
            let name5 = String::from_utf8(name5)?;

            // raw_data: u32[6]
            let mut raw_data = Vec::with_capacity(6 as usize);
            for i in 0..6 {
                raw_data.push(crate::util::read_u32_le(r)?);
            }
            let raw_data = raw_data.try_into().unwrap();

            Some(SMSG_GAMEOBJECT_QUERY_RESPONSEfound {
                info_type,
                display_id,
                name1,
                name2,
                name3,
                name4,
                name5,
                raw_data,
            })
        } else {
            None
        };

        Ok(Self {
            entry_id,
            found,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // entry_id: u32
        w.write_all(&self.entry_id.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // info_type: u32
            w.write_all(&v.info_type.to_le_bytes())?;

            // display_id: u32
            w.write_all(&v.display_id.to_le_bytes())?;

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

            // name5: CString
            w.write_all(v.name5.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // raw_data: u32[6]
            for i in v.raw_data.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // entry_id: u32
            let entry_id = crate::util::tokio_read_u32_le(r).await?;

            // optional found
            let current_size = {
                0
                + 4 // entry_id: u32
            };
            let found = if current_size < body_size as usize {
                // info_type: u32
                let info_type = crate::util::tokio_read_u32_le(r).await?;

                // display_id: u32
                let display_id = crate::util::tokio_read_u32_le(r).await?;

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

                // name5: CString
                let name5 = crate::util::tokio_read_c_string_to_vec(r).await?;
                let name5 = String::from_utf8(name5)?;

                // raw_data: u32[6]
                let mut raw_data = Vec::with_capacity(6 as usize);
                for i in 0..6 {
                    raw_data.push(crate::util::tokio_read_u32_le(r).await?);
                }
                let raw_data = raw_data.try_into().unwrap();

                Some(SMSG_GAMEOBJECT_QUERY_RESPONSEfound {
                    info_type,
                    display_id,
                    name1,
                    name2,
                    name3,
                    name4,
                    name5,
                    raw_data,
                })
            } else {
                None
            };

            Ok(Self {
                entry_id,
                found,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // entry_id: u32
            w.write_all(&self.entry_id.to_le_bytes()).await?;

            // optional found
            if let Some(v) = &self.found {
                // info_type: u32
                w.write_all(&v.info_type.to_le_bytes()).await?;

                // display_id: u32
                w.write_all(&v.display_id.to_le_bytes()).await?;

                // name1: CString
                w.write_all(v.name1.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name2: CString
                w.write_all(v.name2.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name3: CString
                w.write_all(v.name3.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name4: CString
                w.write_all(v.name4.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name5: CString
                w.write_all(v.name5.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // raw_data: u32[6]
                for i in v.raw_data.iter() {
                    w.write_all(&i.to_le_bytes()).await?;
                }

            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // entry_id: u32
            let entry_id = crate::util::astd_read_u32_le(r).await?;

            // optional found
            let current_size = {
                0
                + 4 // entry_id: u32
            };
            let found = if current_size < body_size as usize {
                // info_type: u32
                let info_type = crate::util::astd_read_u32_le(r).await?;

                // display_id: u32
                let display_id = crate::util::astd_read_u32_le(r).await?;

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

                // name5: CString
                let name5 = crate::util::astd_read_c_string_to_vec(r).await?;
                let name5 = String::from_utf8(name5)?;

                // raw_data: u32[6]
                let mut raw_data = Vec::with_capacity(6 as usize);
                for i in 0..6 {
                    raw_data.push(crate::util::astd_read_u32_le(r).await?);
                }
                let raw_data = raw_data.try_into().unwrap();

                Some(SMSG_GAMEOBJECT_QUERY_RESPONSEfound {
                    info_type,
                    display_id,
                    name1,
                    name2,
                    name3,
                    name4,
                    name5,
                    raw_data,
                })
            } else {
                None
            };

            Ok(Self {
                entry_id,
                found,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // entry_id: u32
            w.write_all(&self.entry_id.to_le_bytes()).await?;

            // optional found
            if let Some(v) = &self.found {
                // info_type: u32
                w.write_all(&v.info_type.to_le_bytes()).await?;

                // display_id: u32
                w.write_all(&v.display_id.to_le_bytes()).await?;

                // name1: CString
                w.write_all(v.name1.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name2: CString
                w.write_all(v.name2.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name3: CString
                w.write_all(v.name3.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name4: CString
                w.write_all(v.name4.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // name5: CString
                w.write_all(v.name5.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // raw_data: u32[6]
                for i in v.raw_data.iter() {
                    w.write_all(&i.to_le_bytes()).await?;
                }

            }

            Ok(())
        })
    }

}

impl VariableSized for SMSG_GAMEOBJECT_QUERY_RESPONSE {
    fn size(&self) -> usize {
        0
        + 4 // entry_id: u32
        + if let Some(found) = &self.found {
            0
            + 4 // info_type: u32
            + 4 // display_id: u32
            + found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + found.name5.len() + 1 // name5: CString
            + 6 * core::mem::size_of::<u32>() // raw_data: u32[6]
        } else {
            0
        }
    }
}

impl MaximumPossibleSized for SMSG_GAMEOBJECT_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // entry_id: u32
    }
}

#[derive(Debug)]
pub enum SMSG_GAMEOBJECT_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GAMEOBJECT_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_GAMEOBJECT_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GAMEOBJECT_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GAMEOBJECT_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
    pub info_type: u32,
    pub display_id: u32,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub name5: String,
    pub raw_data: [u32; 6],
}

impl SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
    pub(crate) fn size(&self) -> usize {
        4 // info_type: u32
        + 4 // display_id: u32
        + self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + self.name5.len() + 1 // name5: CString
        + 6 * core::mem::size_of::<u32>() // raw_data: u32[6]
    }

}

