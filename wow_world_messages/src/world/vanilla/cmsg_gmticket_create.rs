use crate::vanilla::Vector3d;
use crate::vanilla::GmTicketType;
use crate::vanilla::Map;
use crate::Message;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm#L1):
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

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // category: GmTicketType
        w.write_all(&u8::from(self.category.as_int()).to_le_bytes())?;

        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // reserved_for_future_use: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.reserved_for_future_use.as_bytes().iter().rev().next(), Some(&0_u8), "String `reserved_for_future_use` must not be null-terminated.");
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
                let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
                for i in compressed_chat_data.iter() {
                    encoder.write_all(&i.to_le_bytes())?;
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
        if !(19..=66072).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0205, size: body_size as u32 });
        }

        // category: GmTicketType
        let category: GmTicketType = crate::util::read_u8_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(message)?
        };

        // reserved_for_future_use: CString
        let reserved_for_future_use = {
            let reserved_for_future_use = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(reserved_for_future_use)?
        };

        let category_if = match category {
            GmTicketType::Stuck => CMSG_GMTICKET_CREATE_GmTicketType::Stuck,
            GmTicketType::BehaviorHarassment => {
                // chat_data_line_count: u32
                let chat_data_line_count = crate::util::read_u32_le(r)?;

                // chat_data_size_uncompressed: u32
                let chat_data_size_uncompressed = crate::util::read_u32_le(r)?;

                // compressed_chat_data: u8[-]
                let compressed_chat_data = {
                    let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

                    let mut current_size = {
                        1 // category: CMSG_GMTICKET_CREATE_GmTicketType
                        + 4 // map: Map
                        + 12 // position: Vector3d
                        + message.len() + 1 // message: CString
                        + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
                    };
                    let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                    while decoder.total_out() < (chat_data_size_uncompressed as u64) {
                        compressed_chat_data.push(crate::util::read_u8_le(decoder)?);
                        current_size += 1;
                    }
                    compressed_chat_data
                };

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
impl crate::vanilla::ClientMessage for CMSG_GMTICKET_CREATE {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let mut v = crate::util::vanilla_get_unencrypted_client(Self::OPCODE as u16, 0);
        self.write_into_vec(&mut v)?;
        let size = v.len().saturating_sub(2);
        let s = size.to_le_bytes();
        v[0] = s[1];
        v[1] = s[0];
        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        w: &mut W,
        e: &mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = crate::util::vanilla_get_unencrypted_client(Self::OPCODE as u16, 0);
        self.write_into_vec(&mut v)?;
        let size = v.len().saturating_sub(2) as u16;
        let header = e.encrypt_client_header(size, Self::OPCODE);
        for (i, e) in header.iter().enumerate() {
            v[i] = *e;
        }
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'s, 'w, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = crate::util::vanilla_get_unencrypted_client(Self::OPCODE as u16, 0);
            self.write_into_vec(&mut v)?;
            let size = v.len().saturating_sub(2);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_client<'s, 'w, 'e, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
        e: &'e mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = crate::util::vanilla_get_unencrypted_client(Self::OPCODE as u16, 0);
            self.write_into_vec(&mut v)?;
            let size = v.len().saturating_sub(2) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'s, 'w, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = crate::util::vanilla_get_unencrypted_client(Self::OPCODE as u16, 0);
            self.write_into_vec(&mut v)?;
            let size = v.len().saturating_sub(2);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'w, 'e, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
        e: &'e mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = crate::util::vanilla_get_unencrypted_client(Self::OPCODE as u16, 0);
            self.write_into_vec(&mut v)?;
            let size = v.len().saturating_sub(2) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

}

impl CMSG_GMTICKET_CREATE {
    pub(crate) fn size(&self) -> usize {
        self.category.size() // category: CMSG_GMTICKET_CREATE_GmTicketType
        + 4 // map: Map
        + 12 // position: Vector3d
        + self.message.len() + 1 // message: CString
        + self.reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
                + crate::util::zlib_compressed_size(compressed_chat_data) // compressed_chat_data: u8[-]
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

