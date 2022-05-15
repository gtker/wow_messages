use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketType, GmTicketTypeError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GMTICKET_CREATE {
    pub category: CMSG_GMTICKET_CREATEGmTicketType,
    pub map: Map,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub message: String,
    pub reserved_for_future_use: String,
}

impl ClientMessageWrite for CMSG_GMTICKET_CREATE {}

impl MessageBody for CMSG_GMTICKET_CREATE {
    const OPCODE: u16 = 0x0205;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GMTICKET_CREATEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // category: GmTicketType
        let category: GmTicketType = crate::util::read_u8_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        // reserved_for_future_use: CString
        let reserved_for_future_use = crate::util::read_c_string_to_vec(r)?;
        let reserved_for_future_use = String::from_utf8(reserved_for_future_use)?;

        let category_if = match category {
            GmTicketType::STUCK => CMSG_GMTICKET_CREATEGmTicketType::STUCK,
            GmTicketType::BEHAVIOR_HARASSMENT => {
                // chat_data_line_count: u32
                let chat_data_line_count = crate::util::read_u32_le(r)?;

                // chat_data_size_uncompressed: u32
                let chat_data_size_uncompressed = crate::util::read_u32_le(r)?;

                // compressed_chat_data: u8[-]
                let mut current_size = {
                    1 // category: CMSG_GMTICKET_CREATEGmTicketType
                    + 4 // map: Map
                    + 4 // position_x: f32
                    + 4 // position_y: f32
                    + 4 // position_z: f32
                    + message.len() + 1 // message: CString
                    + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
                };
                let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                while current_size < (body_size as usize) {
                    compressed_chat_data.push(crate::util::read_u8_le(r)?);
                    current_size += 1;
                }

                CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                    chat_data_line_count,
                    chat_data_size_uncompressed,
                    compressed_chat_data,
                }
            }
            GmTicketType::GUILD => CMSG_GMTICKET_CREATEGmTicketType::GUILD,
            GmTicketType::ITEM => CMSG_GMTICKET_CREATEGmTicketType::ITEM,
            GmTicketType::ENVIRONMENTAL => CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL,
            GmTicketType::NONQUEST_CREEP => CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP,
            GmTicketType::QUEST_QUESTNPC => CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC,
            GmTicketType::TECHNICAL => CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL,
            GmTicketType::ACCOUNT_BILLING => CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING,
            GmTicketType::CHARACTER => CMSG_GMTICKET_CREATEGmTicketType::CHARACTER,
        };

        Ok(Self {
            category: category_if,
            map,
            position_x,
            position_y,
            position_z,
            message,
            reserved_for_future_use,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // category: GmTicketType
        crate::util::write_u8_le(w, self.category.as_int() as u8)?;

        // map: Map
        crate::util::write_u32_le(w, self.map.as_int() as u32)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // reserved_for_future_use: CString
        w.write_all(self.reserved_for_future_use.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        match &self.category {
            CMSG_GMTICKET_CREATEGmTicketType::STUCK => {}
            CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                chat_data_line_count,
                chat_data_size_uncompressed,
                compressed_chat_data,
            } => {
                // chat_data_line_count: u32
                w.write_all(&chat_data_line_count.to_le_bytes())?;

                // chat_data_size_uncompressed: u32
                w.write_all(&chat_data_size_uncompressed.to_le_bytes())?;

                // compressed_chat_data: u8[-]
                for i in compressed_chat_data.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            CMSG_GMTICKET_CREATEGmTicketType::GUILD => {}
            CMSG_GMTICKET_CREATEGmTicketType::ITEM => {}
            CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL => {}
            CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP => {}
            CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC => {}
            CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL => {}
            CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING => {}
            CMSG_GMTICKET_CREATEGmTicketType::CHARACTER => {}
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
            // category: GmTicketType
            let category: GmTicketType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // position_x: f32
            let position_x = crate::util::tokio_read_f32_le(r).await?;
            // position_y: f32
            let position_y = crate::util::tokio_read_f32_le(r).await?;
            // position_z: f32
            let position_z = crate::util::tokio_read_f32_le(r).await?;
            // message: CString
            let message = crate::util::tokio_read_c_string_to_vec(r).await?;
            let message = String::from_utf8(message)?;

            // reserved_for_future_use: CString
            let reserved_for_future_use = crate::util::tokio_read_c_string_to_vec(r).await?;
            let reserved_for_future_use = String::from_utf8(reserved_for_future_use)?;

            let category_if = match category {
                GmTicketType::STUCK => CMSG_GMTICKET_CREATEGmTicketType::STUCK,
                GmTicketType::BEHAVIOR_HARASSMENT => {
                    // chat_data_line_count: u32
                    let chat_data_line_count = crate::util::tokio_read_u32_le(r).await?;

                    // chat_data_size_uncompressed: u32
                    let chat_data_size_uncompressed = crate::util::tokio_read_u32_le(r).await?;

                    // compressed_chat_data: u8[-]
                    let mut current_size = {
                        1 // category: CMSG_GMTICKET_CREATEGmTicketType
                        + 4 // map: Map
                        + 4 // position_x: f32
                        + 4 // position_y: f32
                        + 4 // position_z: f32
                        + message.len() + 1 // message: CString
                        + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
                    };
                    let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                    while current_size < (body_size as usize) {
                        compressed_chat_data.push(crate::util::tokio_read_u8_le(r).await?);
                        current_size += 1;
                    }

                    CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                        chat_data_line_count,
                        chat_data_size_uncompressed,
                        compressed_chat_data,
                    }
                }
                GmTicketType::GUILD => CMSG_GMTICKET_CREATEGmTicketType::GUILD,
                GmTicketType::ITEM => CMSG_GMTICKET_CREATEGmTicketType::ITEM,
                GmTicketType::ENVIRONMENTAL => CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL,
                GmTicketType::NONQUEST_CREEP => CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP,
                GmTicketType::QUEST_QUESTNPC => CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC,
                GmTicketType::TECHNICAL => CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL,
                GmTicketType::ACCOUNT_BILLING => CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING,
                GmTicketType::CHARACTER => CMSG_GMTICKET_CREATEGmTicketType::CHARACTER,
            };

            Ok(Self {
                category: category_if,
                map,
                position_x,
                position_y,
                position_z,
                message,
                reserved_for_future_use,
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
            // category: GmTicketType
            crate::util::tokio_write_u8_le(w, self.category.as_int() as u8).await?;

            // map: Map
            crate::util::tokio_write_u32_le(w, self.map.as_int() as u32).await?;

            // position_x: f32
            w.write_all(&self.position_x.to_le_bytes()).await?;

            // position_y: f32
            w.write_all(&self.position_y.to_le_bytes()).await?;

            // position_z: f32
            w.write_all(&self.position_z.to_le_bytes()).await?;

            // message: CString
            w.write_all(self.message.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // reserved_for_future_use: CString
            w.write_all(self.reserved_for_future_use.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            match &self.category {
                CMSG_GMTICKET_CREATEGmTicketType::STUCK => {}
                CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                    chat_data_line_count,
                    chat_data_size_uncompressed,
                    compressed_chat_data,
                } => {
                    // chat_data_line_count: u32
                    w.write_all(&chat_data_line_count.to_le_bytes()).await?;

                    // chat_data_size_uncompressed: u32
                    w.write_all(&chat_data_size_uncompressed.to_le_bytes()).await?;

                    // compressed_chat_data: u8[-]
                    for i in compressed_chat_data.iter() {
                        w.write_all(&i.to_le_bytes()).await?;
                    }

                }
                CMSG_GMTICKET_CREATEGmTicketType::GUILD => {}
                CMSG_GMTICKET_CREATEGmTicketType::ITEM => {}
                CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL => {}
                CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP => {}
                CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC => {}
                CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL => {}
                CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING => {}
                CMSG_GMTICKET_CREATEGmTicketType::CHARACTER => {}
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
            // category: GmTicketType
            let category: GmTicketType = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // position_x: f32
            let position_x = crate::util::astd_read_f32_le(r).await?;
            // position_y: f32
            let position_y = crate::util::astd_read_f32_le(r).await?;
            // position_z: f32
            let position_z = crate::util::astd_read_f32_le(r).await?;
            // message: CString
            let message = crate::util::astd_read_c_string_to_vec(r).await?;
            let message = String::from_utf8(message)?;

            // reserved_for_future_use: CString
            let reserved_for_future_use = crate::util::astd_read_c_string_to_vec(r).await?;
            let reserved_for_future_use = String::from_utf8(reserved_for_future_use)?;

            let category_if = match category {
                GmTicketType::STUCK => CMSG_GMTICKET_CREATEGmTicketType::STUCK,
                GmTicketType::BEHAVIOR_HARASSMENT => {
                    // chat_data_line_count: u32
                    let chat_data_line_count = crate::util::astd_read_u32_le(r).await?;

                    // chat_data_size_uncompressed: u32
                    let chat_data_size_uncompressed = crate::util::astd_read_u32_le(r).await?;

                    // compressed_chat_data: u8[-]
                    let mut current_size = {
                        1 // category: CMSG_GMTICKET_CREATEGmTicketType
                        + 4 // map: Map
                        + 4 // position_x: f32
                        + 4 // position_y: f32
                        + 4 // position_z: f32
                        + message.len() + 1 // message: CString
                        + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
                    };
                    let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                    while current_size < (body_size as usize) {
                        compressed_chat_data.push(crate::util::astd_read_u8_le(r).await?);
                        current_size += 1;
                    }

                    CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                        chat_data_line_count,
                        chat_data_size_uncompressed,
                        compressed_chat_data,
                    }
                }
                GmTicketType::GUILD => CMSG_GMTICKET_CREATEGmTicketType::GUILD,
                GmTicketType::ITEM => CMSG_GMTICKET_CREATEGmTicketType::ITEM,
                GmTicketType::ENVIRONMENTAL => CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL,
                GmTicketType::NONQUEST_CREEP => CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP,
                GmTicketType::QUEST_QUESTNPC => CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC,
                GmTicketType::TECHNICAL => CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL,
                GmTicketType::ACCOUNT_BILLING => CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING,
                GmTicketType::CHARACTER => CMSG_GMTICKET_CREATEGmTicketType::CHARACTER,
            };

            Ok(Self {
                category: category_if,
                map,
                position_x,
                position_y,
                position_z,
                message,
                reserved_for_future_use,
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
            // category: GmTicketType
            crate::util::astd_write_u8_le(w, self.category.as_int() as u8).await?;

            // map: Map
            crate::util::astd_write_u32_le(w, self.map.as_int() as u32).await?;

            // position_x: f32
            w.write_all(&self.position_x.to_le_bytes()).await?;

            // position_y: f32
            w.write_all(&self.position_y.to_le_bytes()).await?;

            // position_z: f32
            w.write_all(&self.position_z.to_le_bytes()).await?;

            // message: CString
            w.write_all(self.message.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // reserved_for_future_use: CString
            w.write_all(self.reserved_for_future_use.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            match &self.category {
                CMSG_GMTICKET_CREATEGmTicketType::STUCK => {}
                CMSG_GMTICKET_CREATEGmTicketType::BEHAVIOR_HARASSMENT {
                    chat_data_line_count,
                    chat_data_size_uncompressed,
                    compressed_chat_data,
                } => {
                    // chat_data_line_count: u32
                    w.write_all(&chat_data_line_count.to_le_bytes()).await?;

                    // chat_data_size_uncompressed: u32
                    w.write_all(&chat_data_size_uncompressed.to_le_bytes()).await?;

                    // compressed_chat_data: u8[-]
                    for i in compressed_chat_data.iter() {
                        w.write_all(&i.to_le_bytes()).await?;
                    }

                }
                CMSG_GMTICKET_CREATEGmTicketType::GUILD => {}
                CMSG_GMTICKET_CREATEGmTicketType::ITEM => {}
                CMSG_GMTICKET_CREATEGmTicketType::ENVIRONMENTAL => {}
                CMSG_GMTICKET_CREATEGmTicketType::NONQUEST_CREEP => {}
                CMSG_GMTICKET_CREATEGmTicketType::QUEST_QUESTNPC => {}
                CMSG_GMTICKET_CREATEGmTicketType::TECHNICAL => {}
                CMSG_GMTICKET_CREATEGmTicketType::ACCOUNT_BILLING => {}
                CMSG_GMTICKET_CREATEGmTicketType::CHARACTER => {}
            }

            Ok(())
        })
    }

}

impl VariableSized for CMSG_GMTICKET_CREATE {
    fn size(&self) -> usize {
        0
        + self.category.size() // category: CMSG_GMTICKET_CREATEGmTicketType
        + 4 // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + self.message.len() + 1 // message: CString
        + self.reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
    }
}

impl MaximumPossibleSized for CMSG_GMTICKET_CREATE {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

#[derive(Debug)]
pub enum CMSG_GMTICKET_CREATEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    GmTicketType(GmTicketTypeError),
    Map(MapError),
}

impl std::error::Error for CMSG_GMTICKET_CREATEError {}
impl std::fmt::Display for CMSG_GMTICKET_CREATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::GmTicketType(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GMTICKET_CREATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GMTICKET_CREATEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<GmTicketTypeError> for CMSG_GMTICKET_CREATEError {
    fn from(e: GmTicketTypeError) -> Self {
        Self::GmTicketType(e)
    }
}

impl From<MapError> for CMSG_GMTICKET_CREATEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMSG_GMTICKET_CREATEGmTicketType {
    STUCK,
    BEHAVIOR_HARASSMENT {
        chat_data_line_count: u32,
        chat_data_size_uncompressed: u32,
        compressed_chat_data: Vec<u8>,
    },
    GUILD,
    ITEM,
    ENVIRONMENTAL,
    NONQUEST_CREEP,
    QUEST_QUESTNPC,
    TECHNICAL,
    ACCOUNT_BILLING,
    CHARACTER,
}

impl Default for CMSG_GMTICKET_CREATEGmTicketType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::STUCK
    }
}

impl CMSG_GMTICKET_CREATEGmTicketType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STUCK => 1,
            Self::BEHAVIOR_HARASSMENT { .. } => 2,
            Self::GUILD => 3,
            Self::ITEM => 4,
            Self::ENVIRONMENTAL => 5,
            Self::NONQUEST_CREEP => 6,
            Self::QUEST_QUESTNPC => 7,
            Self::TECHNICAL => 8,
            Self::ACCOUNT_BILLING => 9,
            Self::CHARACTER => 10,
        }
    }

}

impl VariableSized for CMSG_GMTICKET_CREATEGmTicketType {
    fn size(&self) -> usize {
        match self {
            Self::STUCK => {
                1
            }
            Self::BEHAVIOR_HARASSMENT {
                chat_data_line_count,
                chat_data_size_uncompressed,
                compressed_chat_data,
            } => {
                1
                + 4 // chat_data_line_count: u32
                + 4 // chat_data_size_uncompressed: u32
                + compressed_chat_data.len() * core::mem::size_of::<u8>() // compressed_chat_data: u8[-]
            }
            Self::GUILD => {
                1
            }
            Self::ITEM => {
                1
            }
            Self::ENVIRONMENTAL => {
                1
            }
            Self::NONQUEST_CREEP => {
                1
            }
            Self::QUEST_QUESTNPC => {
                1
            }
            Self::TECHNICAL => {
                1
            }
            Self::ACCOUNT_BILLING => {
                1
            }
            Self::CHARACTER => {
                1
            }
        }
    }
}

impl MaximumPossibleSized for CMSG_GMTICKET_CREATEGmTicketType {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

