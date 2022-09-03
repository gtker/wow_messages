use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::GmTicketType;
use crate::world::vanilla::Map;
use crate::world::vanilla::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm#L3):
/// ```text
/// cmsg CMSG_GMTICKET_CREATE = 0x0205 {
///     GmTicketType category;
///     Map map;
///     Vector3d position;
///     CString message;
///     CString reserved_for_future_use;
///     if (category == BEHAVIOR_HARASSMENT) {
///         u32 chat_data_line_count;
///         u32 chat_data_size_uncompressed;
///         u8[-] compressed_chat_data;
///     }
/// }
/// ```
pub struct CMSG_GMTICKET_CREATE {
    pub category: CMSG_GMTICKET_CREATE_GmTicketType,
    pub map: Map,
    pub position: Vector3d,
    pub message: String,
    /// cmangos/vmangos/mangoszero: Pre-TBC: 'Reserved for future use'
    /// cmangos/vmangos/mangoszero: Unused
    ///
    pub reserved_for_future_use: String,
}

impl crate::Message for CMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0205;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // category: GmTicketType
        w.write_all(&(self.category.as_int() as u8).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // reserved_for_future_use: CString
        w.write_all(self.reserved_for_future_use.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        match &self.category {
            CMSG_GMTICKET_CREATE_GmTicketType::Stuck => {}
            CMSG_GMTICKET_CREATE_GmTicketType::BehaviorHarassment {
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
            CMSG_GMTICKET_CREATE_GmTicketType::Guild => {}
            CMSG_GMTICKET_CREATE_GmTicketType::Item => {}
            CMSG_GMTICKET_CREATE_GmTicketType::Environmental => {}
            CMSG_GMTICKET_CREATE_GmTicketType::NonquestCreep => {}
            CMSG_GMTICKET_CREATE_GmTicketType::QuestQuestnpc => {}
            CMSG_GMTICKET_CREATE_GmTicketType::Technical => {}
            CMSG_GMTICKET_CREATE_GmTicketType::AccountBilling => {}
            CMSG_GMTICKET_CREATE_GmTicketType::Character => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // category: GmTicketType
        let category: GmTicketType = crate::util::read_u8_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        // reserved_for_future_use: CString
        let reserved_for_future_use = crate::util::read_c_string_to_vec(r)?;
        let reserved_for_future_use = String::from_utf8(reserved_for_future_use)?;

        let category_if = match category {
            GmTicketType::Stuck => CMSG_GMTICKET_CREATE_GmTicketType::Stuck,
            GmTicketType::BehaviorHarassment => {
                // chat_data_line_count: u32
                let chat_data_line_count = crate::util::read_u32_le(r)?;

                // chat_data_size_uncompressed: u32
                let chat_data_size_uncompressed = crate::util::read_u32_le(r)?;

                // compressed_chat_data: u8[-]
                let mut current_size = {
                    1 // category: CMSG_GMTICKET_CREATE_GmTicketType
                    + 4 // map: Map
                    + 12 // position: Vector3d
                    + message.len() + 1 // message: CString
                    + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
                };
                let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                while current_size < (body_size as usize) {
                    compressed_chat_data.push(crate::util::read_u8_le(r)?);
                    current_size += 1;
                }

                CMSG_GMTICKET_CREATE_GmTicketType::BehaviorHarassment {
                    chat_data_line_count,
                    chat_data_size_uncompressed,
                    compressed_chat_data,
                }
            }
            GmTicketType::Guild => CMSG_GMTICKET_CREATE_GmTicketType::Guild,
            GmTicketType::Item => CMSG_GMTICKET_CREATE_GmTicketType::Item,
            GmTicketType::Environmental => CMSG_GMTICKET_CREATE_GmTicketType::Environmental,
            GmTicketType::NonquestCreep => CMSG_GMTICKET_CREATE_GmTicketType::NonquestCreep,
            GmTicketType::QuestQuestnpc => CMSG_GMTICKET_CREATE_GmTicketType::QuestQuestnpc,
            GmTicketType::Technical => CMSG_GMTICKET_CREATE_GmTicketType::Technical,
            GmTicketType::AccountBilling => CMSG_GMTICKET_CREATE_GmTicketType::AccountBilling,
            GmTicketType::Character => CMSG_GMTICKET_CREATE_GmTicketType::Character,
        };

        Ok(Self {
            category: category_if,
            map,
            position,
            message,
            reserved_for_future_use,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GMTICKET_CREATE {}

impl CMSG_GMTICKET_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.category.size() // category: CMSG_GMTICKET_CREATE_GmTicketType
        + 4 // map: Map
        + 12 // position: Vector3d
        + self.message.len() + 1 // message: CString
        + self.reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMSG_GMTICKET_CREATE_GmTicketType {
    Stuck,
    BehaviorHarassment {
        chat_data_line_count: u32,
        chat_data_size_uncompressed: u32,
        compressed_chat_data: Vec<u8>,
    },
    Guild,
    Item,
    Environmental,
    NonquestCreep,
    QuestQuestnpc,
    Technical,
    AccountBilling,
    Character,
}

impl Default for CMSG_GMTICKET_CREATE_GmTicketType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Stuck
    }
}

impl CMSG_GMTICKET_CREATE_GmTicketType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Stuck => 1,
            Self::BehaviorHarassment { .. } => 2,
            Self::Guild => 3,
            Self::Item => 4,
            Self::Environmental => 5,
            Self::NonquestCreep => 6,
            Self::QuestQuestnpc => 7,
            Self::Technical => 8,
            Self::AccountBilling => 9,
            Self::Character => 10,
        }
    }

}

impl CMSG_GMTICKET_CREATE_GmTicketType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Stuck => {
                1
            }
            Self::BehaviorHarassment {
                chat_data_line_count,
                chat_data_size_uncompressed,
                compressed_chat_data,
            } => {
                1
                + 4 // chat_data_line_count: u32
                + 4 // chat_data_size_uncompressed: u32
                + compressed_chat_data.len() * core::mem::size_of::<u8>() // compressed_chat_data: u8[-]
            }
            Self::Guild => {
                1
            }
            Self::Item => {
                1
            }
            Self::Environmental => {
                1
            }
            Self::NonquestCreep => {
                1
            }
            Self::QuestQuestnpc => {
                1
            }
            Self::Technical => {
                1
            }
            Self::AccountBilling => {
                1
            }
            Self::Character => {
                1
            }
        }
    }
}

