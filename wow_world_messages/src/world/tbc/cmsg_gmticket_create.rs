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
    pub reserved_for_future_use: String,
}

impl crate::private::Sealed for CMSG_GMTICKET_CREATE {}
impl CMSG_GMTICKET_CREATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(19..=66072).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // category: GmTicketType
        let category = crate::util::read_u8_le(&mut r)?.try_into()?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

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

                // compressed_chat_data: u8[-]
                let compressed_chat_data = {
                    let compressed_chat_data_decompressed_size = crate::util::read_u32_le(&mut r)?;

                    let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

                    let mut current_size = {
                        1 // category: CMSG_GMTICKET_CREATE_GmTicketType
                        + 4 // map: Map
                        + 12 // position: Vector3d
                        + message.len() + 1 // message: CString
                        + reserved_for_future_use.len() + 1 // reserved_for_future_use: CString
                    };
                    let mut compressed_chat_data = Vec::with_capacity(body_size as usize - current_size);
                    while decoder.total_out() < (compressed_chat_data_decompressed_size as u64) {
                        compressed_chat_data.push(crate::util::read_u8_le(&mut decoder)?);
                        current_size += 1;
                    }
                    compressed_chat_data
                };

                CMSG_GMTICKET_CREATE_GmTicketType::BehaviorHarassment {
                    chat_data_line_count,
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

impl crate::Message for CMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0205;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GMTICKET_CREATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GMTICKET_CREATE {{").unwrap();
        // Members
        writeln!(s, "    category = {};", GmTicketType::try_from(self.category.as_int()).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "        x = {};", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "        y = {};", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "        z = {};", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    message = \"{}\";", self.message).unwrap();
        writeln!(s, "    reserved_for_future_use = \"{}\";", self.reserved_for_future_use).unwrap();
        match &self.category {
            crate::tbc::CMSG_GMTICKET_CREATE_GmTicketType::BehaviorHarassment {
                chat_data_line_count,
                compressed_chat_data,
            } => {
                writeln!(s, "    chat_data_line_count = {};", chat_data_line_count).unwrap();
                writeln!(s, "    compressed_chat_data = [").unwrap();
                for v in compressed_chat_data.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 517_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "category", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 1, "message", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.reserved_for_future_use.len() + 1, "reserved_for_future_use", "    ");
        match &self.category {
            crate::tbc::CMSG_GMTICKET_CREATE_GmTicketType::BehaviorHarassment {
                chat_data_line_count,
                compressed_chat_data,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "chat_data_line_count", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, compressed_chat_data.len(), "compressed_chat_data", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // category: GmTicketType
        w.write_all(&(self.category.as_int().to_le_bytes()))?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().next_back(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // reserved_for_future_use: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.reserved_for_future_use.as_bytes().iter().next_back(), Some(&0_u8), "String `reserved_for_future_use` must not be null-terminated.");
        w.write_all(self.reserved_for_future_use.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        match &self.category {
            CMSG_GMTICKET_CREATE_GmTicketType::BehaviorHarassment {
                chat_data_line_count,
                compressed_chat_data,
            } => {
                // chat_data_line_count: u32
                w.write_all(&chat_data_line_count.to_le_bytes())?;

                // compressed_chat_data: u8[-]
                let decompressed_size: u32 = 1 * compressed_chat_data.len() as u32;
                w.write_all(&decompressed_size.to_le_bytes())?;
                let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
                for i in compressed_chat_data.iter() {
                    encoder.write_all(&i.to_le_bytes())?;
                }

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(517, "CMSG_GMTICKET_CREATE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMTICKET_CREATE {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: Write>(&self, mut w: W) -> Result<(), std::io::Error> {
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
    fn write_encrypted_client<W: Write>(
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

impl std::fmt::Display for CMSG_GMTICKET_CREATE_GmTicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSet => f.write_str("NotSet"),
            Self::Stuck => f.write_str("Stuck"),
            Self::BehaviorHarassment{ .. } => f.write_str("BehaviorHarassment"),
            Self::Guild => f.write_str("Guild"),
            Self::Item => f.write_str("Item"),
            Self::Environmental => f.write_str("Environmental"),
            Self::NonQuestCreep => f.write_str("NonQuestCreep"),
            Self::QuestQuestNpc => f.write_str("QuestQuestNpc"),
            Self::Technical => f.write_str("Technical"),
            Self::AccountBilling => f.write_str("AccountBilling"),
            Self::Character => f.write_str("Character"),
            Self::ArenaHonorItemIssues => f.write_str("ArenaHonorItemIssues"),
            Self::ArenaHonorPointsIssues => f.write_str("ArenaHonorPointsIssues"),
            Self::BottingCheatingHacking => f.write_str("BottingCheatingHacking"),
            Self::BugReport => f.write_str("BugReport"),
            Self::CompromisedAccountIssue => f.write_str("CompromisedAccountIssue"),
            Self::GameSuggestions => f.write_str("GameSuggestions"),
            Self::GameplayQuestion => f.write_str("GameplayQuestion"),
            Self::GuildBankIssue => f.write_str("GuildBankIssue"),
            Self::GuildMasterIssue => f.write_str("GuildMasterIssue"),
            Self::HarassmentScamReport => f.write_str("HarassmentScamReport"),
            Self::InappropriateNameGuildArenaCharacterPet => f.write_str("InappropriateNameGuildArenaCharacterPet"),
            Self::KnownIssueFix => f.write_str("KnownIssueFix"),
            Self::LatencyLagReport => f.write_str("LatencyLagReport"),
            Self::LootingIssueMistake => f.write_str("LootingIssueMistake"),
            Self::MailIssue => f.write_str("MailIssue"),
            Self::NonInGameRelatedInquiry => f.write_str("NonInGameRelatedInquiry"),
            Self::ParentalControlsCais => f.write_str("ParentalControlsCais"),
            Self::Pcnc => f.write_str("Pcnc"),
            Self::Pct => f.write_str("Pct"),
            Self::RestorationStatusFollowUp => f.write_str("RestorationStatusFollowUp"),
            Self::ServerInstanceIssues => f.write_str("ServerInstanceIssues"),
            Self::Spam => f.write_str("Spam"),
            Self::SuicideCase => f.write_str("SuicideCase"),
            Self::SuspensionQuestions => f.write_str("SuspensionQuestions"),
            Self::TechnicalSoundGraphicsIssue => f.write_str("TechnicalSoundGraphicsIssue"),
            Self::UiIssue => f.write_str("UiIssue"),
        }
    }
}

impl CMSG_GMTICKET_CREATE_GmTicketType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::BehaviorHarassment {
                compressed_chat_data,
                ..
            } => {
                1
                + 4 // chat_data_line_count: u32
                + crate::util::zlib_compressed_size(compressed_chat_data) + 4 // compressed_chat_data: u8[-]
            }
            _ => 1,
        }
    }
}

