use crate::Message;
use std::io::{Read, Write};

use crate::tbc::{
    GmTicketType, Map, Vector3d,
};

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

impl crate::private::Sealed for CMSG_GMTICKET_CREATE {}
impl crate::Message for CMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0205;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // category: GmTicketType
        w.write_all(&(self.category.as_int().to_le_bytes()))?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

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
            _ => {}
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(19..=66072).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0205, size: body_size });
        }

        // category: GmTicketType
        let category: GmTicketType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        // reserved_for_future_use: CString
        let reserved_for_future_use = {
            let reserved_for_future_use = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(reserved_for_future_use)?
        };

        let category_if = match category {
            GmTicketType::NotSet => CMSG_GMTICKET_CREATE_GmTicketType::NotSet,
            GmTicketType::Stuck => CMSG_GMTICKET_CREATE_GmTicketType::Stuck,
            GmTicketType::BehaviorHarassment => {
                // chat_data_line_count: u32
                let chat_data_line_count = crate::util::read_u32_le(&mut r)?;

                // chat_data_size_uncompressed: u32
                let chat_data_size_uncompressed = crate::util::read_u32_le(&mut r)?;

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
                        compressed_chat_data.push(crate::util::read_u8_le(&mut decoder)?);
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
            GmTicketType::NonQuestCreep => CMSG_GMTICKET_CREATE_GmTicketType::NonQuestCreep,
            GmTicketType::QuestQuestNpc => CMSG_GMTICKET_CREATE_GmTicketType::QuestQuestNpc,
            GmTicketType::Technical => CMSG_GMTICKET_CREATE_GmTicketType::Technical,
            GmTicketType::AccountBilling => CMSG_GMTICKET_CREATE_GmTicketType::AccountBilling,
            GmTicketType::Character => CMSG_GMTICKET_CREATE_GmTicketType::Character,
            GmTicketType::ArenaHonorItemIssues => CMSG_GMTICKET_CREATE_GmTicketType::ArenaHonorItemIssues,
            GmTicketType::ArenaHonorPointsIssues => CMSG_GMTICKET_CREATE_GmTicketType::ArenaHonorPointsIssues,
            GmTicketType::BottingCheatingHacking => CMSG_GMTICKET_CREATE_GmTicketType::BottingCheatingHacking,
            GmTicketType::BugReport => CMSG_GMTICKET_CREATE_GmTicketType::BugReport,
            GmTicketType::CompromisedAccountIssue => CMSG_GMTICKET_CREATE_GmTicketType::CompromisedAccountIssue,
            GmTicketType::GameSuggestions => CMSG_GMTICKET_CREATE_GmTicketType::GameSuggestions,
            GmTicketType::GameplayQuestion => CMSG_GMTICKET_CREATE_GmTicketType::GameplayQuestion,
            GmTicketType::GuildBankIssue => CMSG_GMTICKET_CREATE_GmTicketType::GuildBankIssue,
            GmTicketType::GuildMasterIssue => CMSG_GMTICKET_CREATE_GmTicketType::GuildMasterIssue,
            GmTicketType::HarassmentScamReport => CMSG_GMTICKET_CREATE_GmTicketType::HarassmentScamReport,
            GmTicketType::InappropriateNameGuildArenaCharacterPet => CMSG_GMTICKET_CREATE_GmTicketType::InappropriateNameGuildArenaCharacterPet,
            GmTicketType::KnownIssueFix => CMSG_GMTICKET_CREATE_GmTicketType::KnownIssueFix,
            GmTicketType::LatencyLagReport => CMSG_GMTICKET_CREATE_GmTicketType::LatencyLagReport,
            GmTicketType::LootingIssueMistake => CMSG_GMTICKET_CREATE_GmTicketType::LootingIssueMistake,
            GmTicketType::MailIssue => CMSG_GMTICKET_CREATE_GmTicketType::MailIssue,
            GmTicketType::NonInGameRelatedInquiry => CMSG_GMTICKET_CREATE_GmTicketType::NonInGameRelatedInquiry,
            GmTicketType::ParentalControlsCais => CMSG_GMTICKET_CREATE_GmTicketType::ParentalControlsCais,
            GmTicketType::Pcnc => CMSG_GMTICKET_CREATE_GmTicketType::Pcnc,
            GmTicketType::Pct => CMSG_GMTICKET_CREATE_GmTicketType::Pct,
            GmTicketType::RestorationStatusFollowUp => CMSG_GMTICKET_CREATE_GmTicketType::RestorationStatusFollowUp,
            GmTicketType::ServerInstanceIssues => CMSG_GMTICKET_CREATE_GmTicketType::ServerInstanceIssues,
            GmTicketType::Spam => CMSG_GMTICKET_CREATE_GmTicketType::Spam,
            GmTicketType::SuicideCase => CMSG_GMTICKET_CREATE_GmTicketType::SuicideCase,
            GmTicketType::SuspensionQuestions => CMSG_GMTICKET_CREATE_GmTicketType::SuspensionQuestions,
            GmTicketType::TechnicalSoundGraphicsIssue => CMSG_GMTICKET_CREATE_GmTicketType::TechnicalSoundGraphicsIssue,
            GmTicketType::UiIssue => CMSG_GMTICKET_CREATE_GmTicketType::UiIssue,
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
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMTICKET_CREATE {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len);
        let s = size.to_le_bytes();
        v[0] = s[1];
        v[1] = s[0];
        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut wow_srp::tbc_header::EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len) as u16;
        let header = e.encrypt_client_header(size, Self::OPCODE);
        for (i, e) in header.iter().enumerate() {
            v[i] = *e;
        }
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::tbc_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::tbc_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
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
    NotSet,
    Stuck,
    BehaviorHarassment {
        chat_data_line_count: u32,
        chat_data_size_uncompressed: u32,
        compressed_chat_data: Vec<u8>,
    },
    Guild,
    Item,
    Environmental,
    NonQuestCreep,
    QuestQuestNpc,
    Technical,
    AccountBilling,
    Character,
    ArenaHonorItemIssues,
    ArenaHonorPointsIssues,
    BottingCheatingHacking,
    BugReport,
    CompromisedAccountIssue,
    GameSuggestions,
    GameplayQuestion,
    GuildBankIssue,
    GuildMasterIssue,
    HarassmentScamReport,
    InappropriateNameGuildArenaCharacterPet,
    KnownIssueFix,
    LatencyLagReport,
    LootingIssueMistake,
    MailIssue,
    NonInGameRelatedInquiry,
    ParentalControlsCais,
    Pcnc,
    Pct,
    RestorationStatusFollowUp,
    ServerInstanceIssues,
    Spam,
    SuicideCase,
    SuspensionQuestions,
    TechnicalSoundGraphicsIssue,
    UiIssue,
}

impl Default for CMSG_GMTICKET_CREATE_GmTicketType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotSet
    }
}

impl CMSG_GMTICKET_CREATE_GmTicketType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NotSet => 0,
            Self::Stuck => 1,
            Self::BehaviorHarassment { .. } => 2,
            Self::Guild => 3,
            Self::Item => 4,
            Self::Environmental => 5,
            Self::NonQuestCreep => 6,
            Self::QuestQuestNpc => 7,
            Self::Technical => 8,
            Self::AccountBilling => 9,
            Self::Character => 10,
            Self::ArenaHonorItemIssues => 11,
            Self::ArenaHonorPointsIssues => 12,
            Self::BottingCheatingHacking => 13,
            Self::BugReport => 14,
            Self::CompromisedAccountIssue => 15,
            Self::GameSuggestions => 16,
            Self::GameplayQuestion => 17,
            Self::GuildBankIssue => 18,
            Self::GuildMasterIssue => 19,
            Self::HarassmentScamReport => 20,
            Self::InappropriateNameGuildArenaCharacterPet => 21,
            Self::KnownIssueFix => 22,
            Self::LatencyLagReport => 23,
            Self::LootingIssueMistake => 24,
            Self::MailIssue => 25,
            Self::NonInGameRelatedInquiry => 26,
            Self::ParentalControlsCais => 27,
            Self::Pcnc => 28,
            Self::Pct => 29,
            Self::RestorationStatusFollowUp => 30,
            Self::ServerInstanceIssues => 31,
            Self::Spam => 32,
            Self::SuicideCase => 33,
            Self::SuspensionQuestions => 34,
            Self::TechnicalSoundGraphicsIssue => 35,
            Self::UiIssue => 36,
        }
    }

}

impl CMSG_GMTICKET_CREATE_GmTicketType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::NotSet => {
                1
            }
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
            Self::NonQuestCreep => {
                1
            }
            Self::QuestQuestNpc => {
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
            Self::ArenaHonorItemIssues => {
                1
            }
            Self::ArenaHonorPointsIssues => {
                1
            }
            Self::BottingCheatingHacking => {
                1
            }
            Self::BugReport => {
                1
            }
            Self::CompromisedAccountIssue => {
                1
            }
            Self::GameSuggestions => {
                1
            }
            Self::GameplayQuestion => {
                1
            }
            Self::GuildBankIssue => {
                1
            }
            Self::GuildMasterIssue => {
                1
            }
            Self::HarassmentScamReport => {
                1
            }
            Self::InappropriateNameGuildArenaCharacterPet => {
                1
            }
            Self::KnownIssueFix => {
                1
            }
            Self::LatencyLagReport => {
                1
            }
            Self::LootingIssueMistake => {
                1
            }
            Self::MailIssue => {
                1
            }
            Self::NonInGameRelatedInquiry => {
                1
            }
            Self::ParentalControlsCais => {
                1
            }
            Self::Pcnc => {
                1
            }
            Self::Pct => {
                1
            }
            Self::RestorationStatusFollowUp => {
                1
            }
            Self::ServerInstanceIssues => {
                1
            }
            Self::Spam => {
                1
            }
            Self::SuicideCase => {
                1
            }
            Self::SuspensionQuestions => {
                1
            }
            Self::TechnicalSoundGraphicsIssue => {
                1
            }
            Self::UiIssue => {
                1
            }
        }
    }
}

