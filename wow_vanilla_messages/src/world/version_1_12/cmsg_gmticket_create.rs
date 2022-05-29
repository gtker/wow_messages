use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GmTicketType;
use crate::world::version_1_12::Map;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl ClientMessage for CMSG_GMTICKET_CREATE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // category: GmTicketType
        w.write_all(&(self.category.as_int() as u8).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

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
    const OPCODE: u16 = 0x0205;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

impl CMSG_GMTICKET_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.category.size() // category: CMSG_GMTICKET_CREATEGmTicketType
        + 4 // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + self.message.len() + 1 // message: CString
        + self.reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
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

impl CMSG_GMTICKET_CREATEGmTicketType {
    pub(crate) fn size(&self) -> usize {
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

